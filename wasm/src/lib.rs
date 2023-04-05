use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn xor(a: u32, b: u32) -> Result<u32, JsValue> {
    return Ok(a ^ b);
}

#[wasm_bindgen]
pub fn get(i: usize) -> Result<usize, JsError> {
    match i {
        69 => Ok(i),
        _ => Err(JsError::new("Could not get number"))
    }
    
}
