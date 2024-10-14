use std::{env, fs};

use json_formatter::formatter;
use json_formatter::parser;
use json_formatter::tokenizer;

fn main() {
    let mut args = env::args();

    args.next();

    let filename = match args.next() {
        Some(filename) => filename,
        None => {
            eprintln!("No filename provided");
            std::process::exit(1);
        }
    };

    let content = match fs::read_to_string(&filename) {
        Ok(content) => content,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                eprintln!("No such file or directory: '{}'", filename);
                std::process::exit(1);
            }
            std::io::ErrorKind::PermissionDenied => {
                eprintln!("Permission denied: '{}'", filename);
                std::process::exit(1);
            }
            _ => {
                eprintln!("Error reading file '{}': {}", filename, error);
                std::process::exit(1);
            }
        },
    };

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

    let formatted = formatter::format(&parsed);

    println!("{}", formatted);
}
