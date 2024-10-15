pub mod formatter;
pub mod parser;
pub mod tokenizer;

#[cfg_attr(feature="wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub fn format_json(content: String) -> String {
    let mut tokens = match tokenizer::tokenize(&content) {
        Ok(tokens) => tokens,
        Err(error) => {
            match error {
                tokenizer::JsonTokenizeError::InvalidEscapeCharacter(character) => {
                    eprintln!("Invalid escape character: '{}'", character);
                }
                tokenizer::JsonTokenizeError::InvalidNumberLiteral(literal) => {
                    eprintln!("Invalid number literal: '{}'", literal);
                }
                tokenizer::JsonTokenizeError::UnexpectedCharacter(character) => {
                    eprintln!("Unexpected character: '{}'", character);
                }
                tokenizer::JsonTokenizeError::UnexpectedEndOfInput => {
                    eprintln!("Unexpected end of input");
                }
                tokenizer::JsonTokenizeError::UnexpectedLiteral(literal) => {
                    eprintln!("Unexpected literal: '{}'", literal);
                }
            }
            std::process::exit(1);
        }
    };

    let parsed = match parser::parser(&mut tokens) {
        Ok(parsed) => parsed,
        Err(error) => {
            match error {
                parser::JsonParserError::UnexpectedEndOfInput => {
                    eprintln!("Unexpected end of input");
                }
                parser::JsonParserError::UnexpectedToken(token) => {
                    eprintln!("Unexpected token: '{}'", token);
                }
            }

            std::process::exit(1);
        }
    };

    formatter::format(&parsed)
}
