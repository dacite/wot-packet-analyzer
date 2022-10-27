use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use wot_replay_parser::ReplayParser;
use serde::Serialize;

extern crate console_error_panic_hook;
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
pub fn parse_packets(replay: &[u8]) -> Result<String, String> {
    console_error_panic_hook::set_once();
    console_log!("Replay length: {}", replay.len());
    let replay = wot_replay_parser::ReplayParser::parse(replay.to_vec()).unwrap();

    let result = from_replay_parser(replay);
    Ok(serde_json::to_string(&result).unwrap())
}

#[derive(Serialize, Clone, Debug)]
pub struct PacketSummary {
    packet_type: u32,
    count: i32,
}

#[derive(Serialize, Clone, Debug)]
pub struct Packet {
    data: Vec<u8>,
    packet_type: u32,
    time: f32,
    adjusted_time: String,
    packet_segments: PacketSegments
}

#[derive(Serialize, Clone, Debug)]
pub struct PacketAnalysisResult {
    packet_summary: Vec<PacketSummary>,
    tank: String,
    map: String,
    version: String,
    time: String,
    packets: Vec<Packet>,
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
    let version = json
        .pointer("/clientVersionFromExe")
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
    for packet in replay_parser.packet_stream().unwrap() {
        let packet = packet.unwrap();

        // Keep track of how many packets of a type occurs
        let count = packet_summary.entry(packet.get_type()).or_insert(0);
        *count += 1;

        // let packet: Vec<String> = packet.get_inner().iter().map(|&byte| hex::encode_upper(&[byte])).collect();
        let packet = Packet {
            data: packet.get_inner().to_vec(),
            packet_type: packet.get_type(),
            time: packet.get_time(),
            adjusted_time: wot_replay_parser::get_replay_time(
                start_time as f64,
                packet.get_time() as f64,
                15,
            ),
            packet_segments: segment_packet(packet.get_inner())
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
        version,
        time,
        packets,
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
    pub zlibs: Vec<i32>
}

fn segment_packet(packet: &[u8]) -> PacketSegments {
    let mut pickles = Vec::new();
    let mut zlibs = Vec::new();

    let mut state = State::Normal;
    for (i, byte) in packet.iter().enumerate() {
        match (state, byte) {
            (State::Normal, 0x80) => {
                if is_pickle_start(&packet[i..]) {
                    state = State::Pickle(i as i32);
                }
            },
            (State::Pickle(begin), 0x2E) => {
                pickles.push((begin, i as i32));
                state = State::Normal 
            },
            (_, 0x78) => {
                if is_zlib_start(&packet[i..]) {
                    zlibs.push(i as i32)
                }
            },
            (_, _) => {}
        }
    }

    PacketSegments { pickles, zlibs }
}