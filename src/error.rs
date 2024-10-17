use crate::parser::JsonParserError;
use crate::tokenizer::JsonTokenizeError;

/// note : [thiserror](https://crates.io/crates/thiserror) を使うと楽
#[derive(Debug)]
pub enum Error {
    Parse(JsonParserError),
    Tokenize(JsonTokenizeError),
}

// std::error::Error の super trait として必要
// 
// これにより ToString も自動的に impl される
// ref: https://github.com/rust-lang/rust/blob/3ed6e3cc69857129c1d314daec00119ff47986ed/library/alloc/src/string.rs#L2677-L2691
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(e) => write!(f, "{e}"),
            Self::Tokenize(e) => write!(f, "{e}"),
        }
    }
}

// なくても最低限動きはするが、(特に直接使われる) エラー型は
// std::error::Error を impl していることが期待される
impl std::error::Error for Error {}

// ? 演算子で JsonParserError, JsonTokenizeError から変換できるように
impl From<JsonParserError> for Error {
    fn from(e: JsonParserError) -> Self {
        Self::Parse(e)
    }
}
impl From<JsonTokenizeError> for Error {
    fn from(e: JsonTokenizeError) -> Self {
        Self::Tokenize(e)
    }
}

impl Into<wasm_bindgen::JsValue> for Error {
    fn into(self) -> wasm_bindgen::JsValue {
        wasm_bindgen::JsValue::from_str(&self.to_string())
    }
}
