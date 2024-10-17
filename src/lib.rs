pub mod error;
pub mod formatter;
pub mod parser;
pub mod tokenizer;

pub fn format_json(content: String) -> Result<String, error::Error> {
    let mut tokens = tokenizer::tokenize(&content)?;
    let parsed = parser::parser(&mut tokens)?;
    let formatted = formatter::format(&parsed);
    Ok(formatted)
}

#[cfg(feature="wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = format_json)]
pub fn format_json_for_wasm(content: String) -> Result<String, String> {
    format_json(content).map_err(|e| e.to_string())
}
