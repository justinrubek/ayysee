use wasm_bindgen::prelude::*;

use ayysee_compiler::generate_program;
use ayysee_parser::grammar::ProgramParser;

#[wasm_bindgen(start)]
fn init_wasm() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen]
pub fn compile_code(code: String) -> Result<String, JsValue> {
    let parser = ProgramParser::new();
    let parsed = parser
        .parse(&code)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let compiled = generate_program(parsed).map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(compiled)
}
