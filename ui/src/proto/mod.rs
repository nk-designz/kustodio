use wasm_bindgen::prelude::*;

pub fn null() -> JsValue {
    JsValue::NULL
}

#[wasm_bindgen(raw_module = "/web_pb.js")]
extern "C" {
    #[derive(Clone, Debug)]
    pub type LockRequest;
    #[wasm_bindgen(constructor)]
    pub fn new(opt: JsValue) -> LockRequest;
    #[wasm_bindgen(method, js_name = "deserializeBinary")]
    pub fn deserialize(this: &LockRequest, bytes: &[u8]);
    #[wasm_bindgen(method, js_name = "serializeBinary")]
    pub fn serialize(this: &LockRequest) -> Vec<u8>;
    #[wasm_bindgen(method, getter, js_name = "getName")]
    pub fn name(this: &LockRequest) -> String;
    #[wasm_bindgen(method, setter, js_name = "setName")]
    pub fn set_name(this: &LockRequest, name: String);

    #[derive(Clone, Debug)]
    pub type LockResponse;
    #[wasm_bindgen(constructor)]
    pub fn new(opt: JsValue) -> LockResponse;
    #[wasm_bindgen(method, js_name = "deserializeBinary")]
    pub fn deserialize(this: &LockResponse, bytes: &[u8]);
    #[wasm_bindgen(method, js_name = "serializeBinary")]
    pub fn serialize(this: &LockResponse) -> Vec<u8>;
    #[wasm_bindgen(method, getter, js_name = "getError")]
    pub fn get_error(this: &LockResponse) -> String;
    #[wasm_bindgen(method, js_name = "hasError")]
    pub fn has_error(this: &LockResponse) -> bool;
    #[wasm_bindgen(method, getter, js_name = "getState")]
    pub fn get_state(this: &LockResponse) -> bool;
    #[wasm_bindgen(method, js_name = "hasState")]
    pub fn has_state(this: &LockResponse) -> bool;

    #[derive(Clone, Debug)]
    pub type LockEvent;
    #[derive(Clone, Debug)]
    #[wasm_bindgen(js_name = "proto.api.messages.LockEvent.Status")]
    pub type LockEventStatus;
    #[wasm_bindgen(constructor)]
    pub fn new(opt: JsValue) -> LockEvent;
    #[wasm_bindgen(method, js_name = "deserializeBinary")]
    pub fn deserialize(this: &LockEvent, bytes: &[u8]);
    #[wasm_bindgen(method, js_name = "serializeBinary")]
    pub fn serialize(this: &LockEvent) -> Vec<u8>;
    #[wasm_bindgen(method, getter, js_name = "getName")]
    pub fn name(this: &LockEvent) -> String;
    #[wasm_bindgen(method, getter, js_name = "getStatus")]
    pub fn status(this: &LockEvent) -> LockEventStatus;

    #[derive(Clone, Debug)]
    pub type PeersResponse;
    #[wasm_bindgen(constructor)]
    pub fn new(opt: JsValue) -> PeersResponse;
    #[wasm_bindgen(method, js_name = "deserializeBinary")]
    pub fn deserialize(this: &PeersResponse, bytes: &[u8]);
    #[wasm_bindgen(method, js_name = "serializeBinary")]
    pub fn serialize(this: &PeersResponse) -> Vec<u8>;
    #[wasm_bindgen(method, getter, js_name = "getPeersList")]
    pub fn get_peers_list(this: &PeersResponse) -> JsValue;

    #[derive(Clone, Debug)]
    pub type ListResponse;
    #[wasm_bindgen(constructor)]
    pub fn new(opt: JsValue) -> ListResponse;

    #[derive(Clone, Debug)]
    pub type Empty;
    #[wasm_bindgen(constructor)]
    pub fn new() -> Empty;
    #[wasm_bindgen(method, js_name = "serializeBinary")]
    pub fn serialize(this: &Empty) -> Vec<u8>;
}
