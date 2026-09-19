//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use sudoku_wasm::{generate_pdf, generate_sudoku};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_sudoku_generation() {
    generate_sudoku();
}

#[wasm_bindgen_test]
fn test_pdf_generation() {
    generate_pdf(1);
}