use wasm_bindgen::prelude::*;

/// Returns a simple greeting with the provided name.
/// 
/// `name` - The name to use in the greeting. 
#[wasm_bindgen]
pub fn greeting(name: String) -> String {
    format!("Hello, {}", name)
}