extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn foo(x: &str) -> String {
    if x == "abc" {
        "yes".to_string()
    } else {
        "no".to_string()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
#[wasm_bindgen]
pub fn use_macro_console_log() {
    console_log!("Hello {}!", "world");
    console_log!("Lets print some numbers");
    console_log!("1 + 3 = {}", 1 + 3);
}
#[wasm_bindgen]
pub fn use_console_log() {
    log("Hello from rust!");
    log_u32(42);
    log_many("Hello world!", "Secondary log!");
}
