use wasm_bindgen::prelude::*;

#[wasm_bindgen(raw_module = "/web_pb.js")]
extern "C" {
    #[wasm_bindgen(js_name = "ListResponse")]
    pub fn list_response(opt_data: JsValue) -> JsValue;
}
