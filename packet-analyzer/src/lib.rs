use std::{collections::HashMap, sync::Mutex};
use aho_corasick::AhoCorasick;
use once_cell::sync::Lazy;
use serde::Serialize;
use standard_format::WotValue;
use wasm_bindgen::prelude::*;
use wot_replay_parser::ReplayParser;

extern crate console_error_panic_hook;

static ANALYSIS_RESULT: Lazy<Mutex<Option<PacketAnalysisResult>>> = Lazy::new(|| Mutex::new(None));

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn parse_packets(replay: &[u8]) -> Result<JsValue, JsValue> {
    console_error_panic_hook::set_once();
    console_log!("Replay length: {}", replay.len());
    let replay = wot_replay_parser::ReplayParser::parse(replay.to_vec()).unwrap();

    let result = from_replay_parser(replay);

    let mut value = ANALYSIS_RESULT.lock().unwrap();
    *value = Some(result.clone());

    Ok(serde_wasm_bindgen::to_value(&result)?)
    // Ok(serde_json::to_string(&result).unwrap())
}

#[wasm_bindgen]
#[derive(Serialize, Clone, Debug)]
pub struct PacketSearchResult {
    pub packet_id: i32,
    pub offset: i32
}

#[wasm_bindgen]
pub fn search_value(needle: &[u8], from: i32) -> Result<JsValue, JsValue> {
    let analysis_result = ANALYSIS_RESULT.lock().unwrap();
    let ac = AhoCorasick::new([needle]);

    for packet in analysis_result.as_ref().unwrap().packets.iter().skip(from as usize + 1) {
        if let Some(mat) = ac.find(&packet.data) {
            return Ok(serde_wasm_bindgen::to_value(&PacketSearchResult {
                packet_id: packet.index as i32,
                offset: mat.start() as i32
            })?)
        }
    }
    Ok(JsValue::null())
}

#[wasm_bindgen]
pub fn get_json() -> String {
    let analysis_result = ANALYSIS_RESULT.lock().unwrap();

    analysis_result.as_ref().unwrap().json.clone()
}

#[wasm_bindgen]
pub fn decompress_and_parse_pickle_stream(stream: &[u8]) -> Result<String, String> {
    let decompressed = miniz_oxide::inflate::decompress_to_vec_zlib(stream).map_err(|e| e.to_string())?;
    parse_pickle_stream(&decompressed)
}

#[wasm_bindgen]
pub fn parse_pickle_stream(stream: &[u8]) -> Result<String, String> {
    console_error_panic_hook::set_once();
    console_log!("Stream length: {:02X?}", stream);
    let pickle_value = serde_pickle::value_from_slice(
        stream,
        serde_pickle::DeOptions::new().replace_unresolved_globals(),
    )
    .map_err(|e| e.to_string())?;

    let wot_value: WotValue = serde_pickle::from_value(pickle_value).map_err(|e| e.to_string())?;

    serde_json::to_string_pretty(&wot_value).map_err(|e| e.to_string())
}

#[derive(Serialize, Clone, Debug)]
#[wasm_bindgen]
pub struct PacketSummary {
    packet_type: u32,
    count: i32,
}

#[derive(Serialize, Clone, Debug)]
pub struct Packet {
    index: usize,
    data: Vec<u8>,
    packet_type: u32,
    time: f32,
    adjusted_time: String,
    packet_segments: PacketSegments,
}

#[derive(Serialize, Clone, Debug)]
pub struct PacketAnalysisResult {
    pub packet_summary: Vec<PacketSummary>,
    pub tank: String,
    pub map: String,
    pub version: String,
    pub time: String,
    pub packets: Vec<Packet>,
    pub players: HashMap<i32, String>,

    #[serde(skip)]
    pub json: String,
}

pub fn from_replay_parser(replay_parser: ReplayParser) -> PacketAnalysisResult {
    let json = &replay_parser.get_json()[0];

    let tank = json
        .pointer("/playerVehicle")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();
    let map = json
        .pointer("/mapDisplayName")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();

    let time = json
        .pointer("/dateTime")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();

    let mut packet_summary = HashMap::new();
    let mut packets = Vec::new();

    let start_time = replay_parser.get_battle_start_time();
    for (index, packet) in replay_parser.packet_stream().unwrap().enumerate() {
        let packet = packet.unwrap();

        // Keep track of how many packets of a type occurs
        let count = packet_summary.entry(packet.get_type()).or_insert(0);
        *count += 1;

        // let packet: Vec<String> = packet.get_inner().iter().map(|&byte| hex::encode_upper(&[byte])).collect();
        let packet = Packet {
            index,
            data: packet.get_inner().to_vec(),
            packet_type: packet.get_type(),
            time: packet.get_time(),
            adjusted_time: wot_replay_parser::get_replay_time(
                start_time as f64,
                packet.get_time() as f64,
                15,
            ),
            packet_segments: segment_packet(packet.get_inner()),
        };

        packets.push(packet);
    }

    let packet_summary: Vec<_> = packet_summary
        .into_iter()
        .map(|(key, value)| PacketSummary {
            packet_type: key,
            count: value,
        })
        .collect();

    PacketAnalysisResult {
        packet_summary,
        tank,
        map,
        version: format!("{:?}", replay_parser.get_version().unwrap_or([0,0,0,0])),
        time,
        packets,
        players: get_player_list(json),
        json: serde_json::to_string_pretty(replay_parser.get_json()).unwrap_or_else(|_|"".to_owned()),
    }
}

#[derive(Copy, Clone)]
pub enum State {
    Normal,
    Pickle(i32),
}

fn is_pickle_start(input: &[u8]) -> bool {
    input.len() > 1 && input[0] == 0x80 && input[1] == 0x02
}

fn is_zlib_start(input: &[u8]) -> bool {
    input.len() > 1 && input[0] == 0x78 && input[1] == 0x9C
}

#[derive(Serialize, Clone, Debug)]
pub struct PacketSegments {
    pub pickles: Vec<(i32, i32)>,
    pub zlibs: Vec<i32>,
}

fn segment_packet(packet: &[u8]) -> PacketSegments {
    let mut pickles = Vec::new();
    let mut zlibs = Vec::new();

    let mut state = State::Normal;
    for (i, byte) in packet.iter().enumerate() {
        match (state, byte) {
            (State::Normal, 0x80) if is_pickle_start(&packet[i..]) => {
                state = State::Pickle(i as i32)
            }
            (State::Pickle(begin), 0x2E) => {
                pickles.push((begin, i as i32));
                state = State::Normal
            }
            (_, 0x78) if is_zlib_start(&packet[i..]) => zlibs.push(i as i32),
            (_, _) => {}
        }
    }

    PacketSegments { pickles, zlibs }
}

fn get_player_list(json: &serde_json::Value) -> HashMap<i32, String> {
    let mut player_list = HashMap::new();
    let vehicles = json["vehicles"].as_object().unwrap();
    for i in vehicles.into_iter() {
        let avatar_id = i.0.parse::<i32>().unwrap();
        let name = i.1["name"].as_str().unwrap();
        let tank = i.1["vehicleType"].as_str().unwrap();

        player_list.insert(avatar_id, format!("{}, {}", name, tank));
    }
    player_list
}