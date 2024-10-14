use std::{iter::Peekable, str::Chars};

#[derive(Debug, PartialEq, Clone)]
pub enum JsonToken {
    LeftSquareBracket,  // [
    LeftCurlyBracket,   // {
    RightSquareBracket, // ]
    RightCurlyBracket,  // }
    Colon,              // :
    Comma,              // ,
    True,
    False,
    Null,
    String(String),
    Number(f64),
}

impl std::fmt::Display for JsonToken {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            JsonToken::LeftSquareBracket => write!(f, "["),
            JsonToken::LeftCurlyBracket => write!(f, "{{"),
            JsonToken::RightSquareBracket => write!(f, "]"),
            JsonToken::RightCurlyBracket => write!(f, "}}"),
            JsonToken::Colon => write!(f, ":"),
            JsonToken::Comma => write!(f, ","),
            JsonToken::True => write!(f, "true"),
            JsonToken::False => write!(f, "false"),
            JsonToken::Null => write!(f, "null"),
            JsonToken::String(value) => write!(f, "\"{}\"", value),
            JsonToken::Number(value) => write!(f, "{}", value),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum JsonTokenizeError {
    UnexpectedLiteral(String),
    UnexpectedCharacter(char),
    UnexpectedEndOfInput,
    InvalidEscapeCharacter(String),
    InvalidNumberLiteral(String),
}

type JsonTokenizeResult = Result<Vec<JsonToken>, JsonTokenizeError>;

pub fn tokenize(input: &str) -> JsonTokenizeResult {
    let mut chars = input.chars().peekable();
    let mut tokens = Vec::new();

    while let Some(&char) = chars.peek() {
        match char {
            ' ' | '\n' | '\t' | '\r' => {
                chars.next();
            }
            '[' => {
                chars.next();
                tokens.push(JsonToken::LeftSquareBracket);
            }
            '{' => {
                chars.next();
                tokens.push(JsonToken::LeftCurlyBracket);
            }
            ']' => {
                chars.next();
                tokens.push(JsonToken::RightSquareBracket);
            }
            '}' => {
                chars.next();
                tokens.push(JsonToken::RightCurlyBracket);
            }
            ':' => {
                chars.next();
                tokens.push(JsonToken::Colon);
            }
            ',' => {
                chars.next();
                tokens.push(JsonToken::Comma);
            }
            '"' => match tokenize_string(&mut chars) {
                Ok(token) => tokens.push(token),
                Err(err) => return Err(err),
            },
            '-' | '0'..='9' => match tokenize_number(&mut chars) {
                Ok(token) => tokens.push(token),
                Err(err) => return Err(err),
            },
            _ => match tokenize_literal(&mut chars) {
                Ok(token) => tokens.push(token),
                Err(err) => return Err(err),
            },
        }
    }

    Ok(tokens)
}

fn tokenize_string(chars: &mut Peekable<Chars>) -> Result<JsonToken, JsonTokenizeError> {
    chars.next(); // consume the opening quote

    let mut string_value = String::new();

    while let Some(char) = chars.next() {
        match char {
            '"' => break,
            '\\' => match chars.next() {
                Some('"') => string_value.push('\u{0022}'),
                Some('\\') => string_value.push('\u{005C}'),
                Some('/') => string_value.push('\u{002F}'),
                Some('b') => string_value.push('\u{0008}'),
                Some('f') => string_value.push('\u{000C}'),
                Some('n') => string_value.push('\u{000A}'),
                Some('r') => string_value.push('\u{000D}'),
                Some('t') => string_value.push('\u{0009}'),
                Some('u') => {
                    let mut hex_chars = String::new();
                    while let Some(&char) = chars.peek() {
                        if char == '"' {
                            break;
                        } else {
                            hex_chars.push(char);
                            chars.next();
                            if hex_chars.len() == 4 {
                                break;
                            }
                        }
                    }

                    if hex_chars.len() != 4 {
                        return Err(JsonTokenizeError::InvalidEscapeCharacter(hex_chars));
                    }

                    if let Ok(hex_as_char) = u32::from_str_radix(&hex_chars, 16).unwrap().try_into()
                    {
                        string_value.push(hex_as_char);
                    } else {
                        return Err(JsonTokenizeError::InvalidEscapeCharacter(hex_chars));
                    }
                }
                Some(char) => {
                    return Err(JsonTokenizeError::InvalidEscapeCharacter(char.to_string()))
                }
                None => return Err(JsonTokenizeError::UnexpectedEndOfInput),
            },
            _ => string_value.push(char),
        }
    }

    Ok(JsonToken::String(string_value))
}

fn tokenize_number(chars: &mut Peekable<Chars>) -> Result<JsonToken, JsonTokenizeError> {
    let mut number_chars = String::new();

    while let Some(&char) = chars.peek() {
        match char {
            '0'..='9' | '-' | '+' | 'e' | 'E' | '.' => {
                number_chars.push(char);
                chars.next();
            }
            _ => break,
        }
    }

    match number_chars.parse::<f64>() {
        Ok(number) => Ok(JsonToken::Number(number)),
        Err(_) => Err(JsonTokenizeError::InvalidNumberLiteral(number_chars)),
    }
}

fn tokenize_literal(chars: &mut Peekable<Chars>) -> Result<JsonToken, JsonTokenizeError> {
    let mut literal = String::new();

    while let Some(&char) = chars.peek() {
        match char {
            '[' | ']' | '{' | '}' | ':' | ',' | ' ' | '\n' | '\t' | '\r' => break,
            _ => {
                chars.next();
                literal.push(char);
            }
        }
    }

    match literal.as_str() {
        "true" => Ok(JsonToken::True),
        "false" => Ok(JsonToken::False),
        "null" => Ok(JsonToken::Null),
        _ => Err(JsonTokenizeError::UnexpectedLiteral(literal)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_empty() {
        let input = "";
        let actual = tokenize(&input);
        let expected = Ok(vec![]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn tokenize_left_square_bracket() {
        let input = "[";
        let actual = tokenize(&input);
        let expected = Ok(vec![JsonToken::LeftSquareBracket]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn tokenize_right_square_bracket() {
        let input = "]";
        let actual = tokenize(&input);
        let expected = Ok(vec![JsonToken::RightSquareBracket]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn tokenize_left_curly_bracket() {
        let input = "{";
        let actual = tokenize(&input);
        let expected = Ok(vec![JsonToken::LeftCurlyBracket]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn tokenize_right_curly_bracket() {
        let input = "}";
        let actual = tokenize(&input);
        let expected = Ok(vec![JsonToken::RightCurlyBracket]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn tokenize_colon() {
        let input = ":";
        let actual = tokenize(&input);
        let expected = Ok(vec![JsonToken::Colon]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn tokenize_comma() {
        let input = ",";
        let actual = tokenize(&input);
        let expected = Ok(vec![JsonToken::Comma]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn tokenize_ignore_whitespace() {
        let input = " \n\t\r";
        let actual = tokenize(&input);
        let expected = Ok(vec![]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn tokenize_true() {
        let input = "true";
        let actual = tokenize(&input);
        let expected = Ok(vec![JsonToken::True]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn tokenize_false() {
        let input = "false";
        let actual = tokenize(&input);
        let expected = Ok(vec![JsonToken::False]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn tokenize_null() {
        let input = "null";
        let actual = tokenize(&input);
        let expected = Ok(vec![JsonToken::Null]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn tokenize_unexpected_literal() {
        let input = "nulll";
        let actual = tokenize(&input);
        let expected = Err(JsonTokenizeError::UnexpectedLiteral("nulll".to_string()));
        assert_eq!(actual, expected);
    }

    #[test]
    fn tokenize_string() {
        let input = "\"hello\"";
        let actual = tokenize(&input);
        let expected = Ok(vec![JsonToken::String("hello".to_string())]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn tokenize_string_with_escaped_chars() {
        let input = "\" \\\" \\\\ \\/ \\b \\f \\n \\r \\t\"";
        let actual = tokenize(&input);
        let expected = Ok(vec![JsonToken::String(
            " \" \\ / \u{0008} \u{000C} \n \r \t".to_string(),
        )]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn tokenize_string_with_unicode_escape_chars() {
        let input = "\"\\u0048\\u0065\\u006C\\u006C\\u006F\"";
        let actual = tokenize(&input);
        let expected = Ok(vec![JsonToken::String("Hello".to_string())]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn tokenize_invalid_escape_character() {
        let input = "\"\\x\"";
        let actual = tokenize(&input);
        let expected = Err(JsonTokenizeError::InvalidEscapeCharacter("x".to_string()));
        assert_eq!(actual, expected);
    }

    #[test]
    fn tokenize_invalid_unicode_escape_character() {
        let input = "\"\\u123\"";
        let actual = tokenize(&input);
        let expected = Err(JsonTokenizeError::InvalidEscapeCharacter("123".to_string()));
        assert_eq!(actual, expected);
    }

    #[test]
    fn tokenize_number_positive() {
        assert_eq!(tokenize("123"), Ok(vec![JsonToken::Number(123.0)]));
        assert_eq!(tokenize("123.456"), Ok(vec![JsonToken::Number(123.456)]));
        assert_eq!(tokenize("123e4"), Ok(vec![JsonToken::Number(123e4)]));
        assert_eq!(tokenize("123e+4"), Ok(vec![JsonToken::Number(123e4)]));
        assert_eq!(tokenize("123E4"), Ok(vec![JsonToken::Number(123e4)]));
        assert_eq!(tokenize("123e-4"), Ok(vec![JsonToken::Number(123e-4)]));
        assert_eq!(
            tokenize("123.456e-789"),
            Ok(vec![JsonToken::Number(123.456e-789)])
        );
    }

    #[test]
    fn tokenize_number_negative() {
        assert_eq!(tokenize("-123"), Ok(vec![JsonToken::Number(-123.0)]));
        assert_eq!(tokenize("-123.456"), Ok(vec![JsonToken::Number(-123.456)]));
        assert_eq!(tokenize("-123e4"), Ok(vec![JsonToken::Number(-123e4)]));
        assert_eq!(tokenize("-123e+4"), Ok(vec![JsonToken::Number(-123e4)]));
        assert_eq!(tokenize("-123E4"), Ok(vec![JsonToken::Number(-123e4)]));
        assert_eq!(tokenize("-123e-4"), Ok(vec![JsonToken::Number(-123e-4)]));
        assert_eq!(
            tokenize("-123.456e-789"),
            Ok(vec![JsonToken::Number(-123.456e-789)])
        );
    }

    #[test]
    fn tokenize_invalid_number_literal() {
        let input = "123.456.789";
        let actual = tokenize(&input);
        let expected = Err(JsonTokenizeError::InvalidNumberLiteral(
            "123.456.789".to_string(),
        ));
        assert_eq!(actual, expected);
    }
}
