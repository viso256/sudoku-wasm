mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn generate_sudoku() -> String {
    sudoku::generate_puzzle().to_json()
}

#[wasm_bindgen]
pub fn generate_pdf(pages: usize) -> Vec<u8> {
    sudoku::generate_pdf(pages)
}
