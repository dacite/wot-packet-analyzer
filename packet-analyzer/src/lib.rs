use wasm_bindgen::prelude::*;
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
pub fn parse_packets(replay: &[u8]) -> String {
    console_error_panic_hook::set_once();
    console_log!("Replay length: {}",  replay.len());
    let replay = wot_replay_parser::ReplayParser::parse(replay.to_vec()).unwrap();
    
    serde_json::to_string(replay.get_json()).unwrap()
}