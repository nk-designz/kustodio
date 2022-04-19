use wasm_bindgen::prelude::*;

#[wasm_bindgen(raw_module = "/web_pb.js")]
extern "C" {
    type proto;
}
