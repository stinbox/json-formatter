pub mod error;
pub mod formatter;
pub mod parser;
pub mod tokenizer;

#[cfg_attr(feature="wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub fn format_json(content: String) -> Result<String, error::Error> {
    let mut tokens = tokenizer::tokenize(&content)?;
    let parsed = parser::parser(&mut tokens)?;
    let formatted = formatter::format(&parsed);
    Ok(formatted)
}
