use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn eins() -> String {
    String::from("1")
}

#[wasm_bindgen]
pub fn zwei() -> i32 {
    2
}

#[wasm_bindgen]
pub fn drei() -> f64 {
    3.14
}
