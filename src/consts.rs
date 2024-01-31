use wasm_bindgen::prelude::*;

/// Specify whether an option is put or call
#[wasm_bindgen]
#[derive(PartialEq, Debug, Copy, Clone, PartialOrd)]
pub enum OptionDir {
    CALL = 2,
    PUT = 1,
}
