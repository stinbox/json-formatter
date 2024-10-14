use crate::tokenizer::JsonToken;
use std::{iter::Peekable, slice::Iter};

#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

#[derive(Debug, PartialEq)]
pub enum JsonParserError {
    UnexpectedToken(JsonToken),
    UnexpectedEndOfInput,
}

fn parser(tokens: &Vec<JsonToken>) -> Result<JsonValue, JsonParserError> {
    let mut tokens = tokens.iter().peekable();
    parser_value(&mut tokens)
}

fn parser_value(
    mut tokens: &mut Peekable<Iter<'_, JsonToken>>,
) -> Result<JsonValue, JsonParserError> {
    if let Some(&token) = tokens.peek() {
        match token {
            JsonToken::Null => {
                tokens.next();
                Ok(JsonValue::Null)
            }
            JsonToken::True => {
                tokens.next();
                Ok(JsonValue::Bool(true))
            }
            JsonToken::False => {
                tokens.next();
                Ok(JsonValue::Bool(false))
            }
            JsonToken::Number(number) => {
                tokens.next();
                Ok(JsonValue::Number(*number))
            }
            JsonToken::String(string) => {
                tokens.next();
                Ok(JsonValue::String(string.clone()))
            }
            JsonToken::LeftSquareBracket => parser_array(&mut tokens),
            JsonToken::LeftCurlyBracket => parser_object(&mut tokens),
            _ => Err(JsonParserError::UnexpectedToken(token.clone())),
        }
    } else {
        Err(JsonParserError::UnexpectedEndOfInput)
    }
}

fn parser_object(
    mut tokens: &mut Peekable<Iter<'_, JsonToken>>,
) -> Result<JsonValue, JsonParserError> {
    let mut object = Vec::new();

    tokens.next(); // consume the LeftCurlyBracket

    if let Some(&token) = tokens.peek() {
        match token {
            JsonToken::RightCurlyBracket => {
                tokens.next();
                return Ok(JsonValue::Object(object));
            }
            JsonToken::String(_) => {
                let (key, value) = parser_object_key_value(&mut tokens)?;
                object.push((key, value));
            }
            _ => {
                return Err(JsonParserError::UnexpectedToken(token.clone()));
            }
        }
    }

    while let Some(&token) = tokens.peek() {
        match token {
            JsonToken::Comma => {
                tokens.next();
                if let Some(&token) = tokens.peek() {
                    match token {
                        JsonToken::String(_) => {
                            let (key, value) = parser_object_key_value(&mut tokens)?;
                            object.push((key, value));
                        }
                        _ => {
                            return Err(JsonParserError::UnexpectedToken(token.clone()));
                        }
                    }
                }
            }
            JsonToken::RightCurlyBracket => {
                tokens.next();
                return Ok(JsonValue::Object(object));
            }
            _ => {
                println!("here????");
                return Err(JsonParserError::UnexpectedToken(token.clone()));
            }
        }
    }

    Err(JsonParserError::UnexpectedEndOfInput)
}

fn parser_object_key_value(
    tokens: &mut Peekable<Iter<'_, JsonToken>>,
) -> Result<(String, JsonValue), JsonParserError> {
    let key = tokens.next();
    let key = match key {
        Some(JsonToken::String(key)) => key.clone(),
        Some(token) => return Err(JsonParserError::UnexpectedToken(token.clone())),
        None => return Err(JsonParserError::UnexpectedEndOfInput),
    };

    let colon = tokens.next();
    if colon != Some(&JsonToken::Colon) {
        return Err(JsonParserError::UnexpectedEndOfInput);
    }

    let value = parser_value(tokens)?;

    Ok((key, value))
}

fn parser_array(
    mut tokens: &mut Peekable<Iter<'_, JsonToken>>,
) -> Result<JsonValue, JsonParserError> {
    let mut array = Vec::new();

    tokens.next(); // consume the LeftSquareBracket

    if let Some(&token) = tokens.peek() {
        match token {
            JsonToken::RightSquareBracket => {
                tokens.next();
                return Ok(JsonValue::Array(array));
            }
            _ => {
                let value = parser_value(&mut tokens)?;
                array.push(value);
            }
        }
    };

    while let Some(&token) = tokens.peek() {
        match token {
            JsonToken::Comma => {
                tokens.next();
                let value = parser_value(&mut tokens)?;
                array.push(value);
            }
            JsonToken::RightSquareBracket => {
                tokens.next();
                return Ok(JsonValue::Array(array));
            }
            _ => {
                return Err(JsonParserError::UnexpectedToken(token.clone()));
            }
        }
    }

    Err(JsonParserError::UnexpectedEndOfInput)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_null() {
        let tokens = vec![JsonToken::Null];
        let result = parser(&tokens);
        assert_eq!(result, Ok(JsonValue::Null));
    }

    #[test]
    fn parse_true() {
        let tokens = vec![JsonToken::True];
        let result = parser(&tokens);
        assert_eq!(result, Ok(JsonValue::Bool(true)));
    }

    #[test]
    fn parse_false() {
        let tokens = vec![JsonToken::False];
        let result = parser(&tokens);
        assert_eq!(result, Ok(JsonValue::Bool(false)));
    }

    #[test]
    fn parse_number() {
        let tokens = vec![JsonToken::Number(42.0)];
        let result = parser(&tokens);
        assert_eq!(result, Ok(JsonValue::Number(42.0)));
    }

    #[test]
    fn parse_string() {
        let tokens = vec![JsonToken::String("hello".to_string())];
        let result = parser(&tokens);
        assert_eq!(result, Ok(JsonValue::String("hello".to_string())));
    }

    #[test]
    fn parse_empty_array() {
        let tokens = vec![JsonToken::LeftSquareBracket, JsonToken::RightSquareBracket];
        let result = parser(&tokens);
        assert_eq!(result, Ok(JsonValue::Array(vec![])));
    }

    #[test]
    fn parse_array_with_literals() {
        let tokens = vec![
            JsonToken::LeftSquareBracket,
            JsonToken::Null,
            JsonToken::Comma,
            JsonToken::True,
            JsonToken::Comma,
            JsonToken::False,
            JsonToken::Comma,
            JsonToken::String("hello".to_string()),
            JsonToken::Comma,
            JsonToken::Number(42.0),
            JsonToken::RightSquareBracket,
        ];
        let result = parser(&tokens);
        assert_eq!(
            result,
            Ok(JsonValue::Array(vec![
                JsonValue::Null,
                JsonValue::Bool(true),
                JsonValue::Bool(false),
                JsonValue::String("hello".to_string()),
                JsonValue::Number(42.0),
            ]))
        );
    }

    #[test]
    fn parse_empty_object() {
        let tokens = vec![JsonToken::LeftCurlyBracket, JsonToken::RightCurlyBracket];
        let result = parser(&tokens);
        assert_eq!(result, Ok(JsonValue::Object(vec![])));
    }

    #[test]
    fn parse_object_with_literals() {
        let tokens = vec![
            JsonToken::LeftCurlyBracket,
            JsonToken::String("null".to_string()),
            JsonToken::Colon,
            JsonToken::Null,
            JsonToken::Comma,
            JsonToken::String("true".to_string()),
            JsonToken::Colon,
            JsonToken::True,
            JsonToken::Comma,
            JsonToken::String("false".to_string()),
            JsonToken::Colon,
            JsonToken::False,
            JsonToken::Comma,
            JsonToken::String("string".to_string()),
            JsonToken::Colon,
            JsonToken::String("hello".to_string()),
            JsonToken::Comma,
            JsonToken::String("number".to_string()),
            JsonToken::Colon,
            JsonToken::Number(42.0),
            JsonToken::RightCurlyBracket,
        ];
        let result = parser(&tokens);
        assert_eq!(
            result,
            Ok(JsonValue::Object(vec![
                ("null".to_string(), JsonValue::Null),
                ("true".to_string(), JsonValue::Bool(true)),
                ("false".to_string(), JsonValue::Bool(false)),
                ("string".to_string(), JsonValue::String("hello".to_string())),
                ("number".to_string(), JsonValue::Number(42.0))
            ]))
        );
    }

    #[test]
    fn parse_nested_object() {
        let tokens = vec![
            JsonToken::LeftCurlyBracket,
            JsonToken::String("true".to_string()),
            JsonToken::Colon,
            JsonToken::True,
            JsonToken::Comma,
            JsonToken::String("object".to_string()),
            JsonToken::Colon,
            JsonToken::LeftCurlyBracket,
            JsonToken::String("null".to_string()),
            JsonToken::Colon,
            JsonToken::Null,
            JsonToken::Comma,
            JsonToken::String("array".to_string()),
            JsonToken::Colon,
            JsonToken::LeftSquareBracket,
            JsonToken::Number(42.0),
            JsonToken::RightSquareBracket,
            JsonToken::RightCurlyBracket,
            JsonToken::RightCurlyBracket,
        ];
        let result = parser(&tokens);
        assert_eq!(
            result,
            Ok(JsonValue::Object(vec![
                ("true".to_string(), JsonValue::Bool(true)),
                (
                    "object".to_string(),
                    JsonValue::Object(vec![
                        ("null".to_string(), JsonValue::Null),
                        (
                            "array".to_string(),
                            JsonValue::Array(vec![JsonValue::Number(42.0)])
                        )
                    ]),
                )
            ]))
        );
    }
}
