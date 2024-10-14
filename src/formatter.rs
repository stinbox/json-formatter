use crate::parser::JsonValue;

pub fn format(value: &JsonValue) -> String {
    format_value(value, 1)
}

fn format_value(value: &JsonValue, indent_level: usize) -> String {
    match value {
        JsonValue::Null => "null".to_string(),
        JsonValue::Bool(b) => b.to_string(),
        JsonValue::Number(n) => n.to_string(),
        JsonValue::String(s) => format!("\"{}\"", s),
        JsonValue::Object(_) => format_object(value, indent_level),
        JsonValue::Array(_) => format_array(value, indent_level),
    }
}

fn format_object(value: &JsonValue, indent_level: usize) -> String {
    if let JsonValue::Object(entries) = value {
        if entries.len() == 0 {
            return "{}".to_string();
        }

        let entries_string = entries
            .iter()
            .map(|(key, value)| {
                format!(
                    "{}\"{}\": {}",
                    "  ".repeat(indent_level),
                    key,
                    format_value(value, indent_level + 1)
                )
            })
            .collect::<Vec<String>>()
            .join(",\n");

        format!(
            "{{\n{}\n{}}}",
            entries_string,
            "  ".repeat(indent_level - 1)
        )
    } else {
        panic!("Expected object");
    }
}

fn format_array(value: &JsonValue, indent_level: usize) -> String {
    if let JsonValue::Array(values) = value {
        if values.len() == 0 {
            return "[]".to_string();
        }

        let values_string = values
            .iter()
            .map(|value| {
                format!(
                    "{}{}",
                    "  ".repeat(indent_level),
                    format_value(value, indent_level + 1)
                )
            })
            .collect::<Vec<String>>()
            .join(",\n");

        format!("[\n{}\n{}]", values_string, "  ".repeat(indent_level - 1))
    } else {
        panic!("Expected array");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_null() {
        let value = JsonValue::Null;
        let result = format(&value);
        assert_eq!(result, "null");
    }

    #[test]
    fn format_true() {
        let value = JsonValue::Bool(true);
        let result = format(&value);
        assert_eq!(result, "true");
    }

    #[test]
    fn format_false() {
        let value = JsonValue::Bool(false);
        let result = format(&value);
        assert_eq!(result, "false");
    }

    #[test]
    fn format_number() {
        let value = JsonValue::Number(123.4);
        let result = format(&value);
        assert_eq!(result, "123.4");
    }

    #[test]
    fn format_string() {
        let value = JsonValue::String("hello".to_string());
        let result = format(&value);
        assert_eq!(result, "\"hello\"");
    }

    #[test]
    fn format_object_empty() {
        let value = JsonValue::Object(vec![]);
        let result = format(&value);
        assert_eq!(result, "{}");
    }

    #[test]
    fn format_object_nested() {
        let value = JsonValue::Object(vec![
            ("number".to_string(), JsonValue::Number(123.4)),
            (
                "object".to_string(),
                JsonValue::Object(vec![
                    ("string".to_string(), JsonValue::String("hello".to_string())),
                    (
                        "array".to_string(),
                        JsonValue::Array(vec![
                            JsonValue::Bool(true),
                            JsonValue::Bool(false),
                            JsonValue::Null,
                        ]),
                    ),
                ]),
            ),
        ]);
        let result = format(&value);
        assert_eq!(
            result,
            r#"{
  "number": 123.4,
  "object": {
    "string": "hello",
    "array": [
      true,
      false,
      null
    ]
  }
}"#
        );
    }

    #[test]
    fn format_array_empty() {
        let value = JsonValue::Array(vec![]);
        let result = format(&value);
        assert_eq!(result, "[]");
    }

    #[test]
    fn format_array_nested() {
        let value = JsonValue::Array(vec![
            JsonValue::String("hello".to_string()),
            JsonValue::Object(vec![
                ("age".to_string(), JsonValue::Number(18.0)),
                ("name".to_string(), JsonValue::String("Alice".to_string())),
                (
                    "hobbies".to_string(),
                    JsonValue::Array(vec![
                        JsonValue::Object(vec![
                            ("name".to_string(), JsonValue::String("Reading".to_string())),
                            ("level".to_string(), JsonValue::Number(3.0)),
                        ]),
                        JsonValue::Object(vec![
                            (
                                "name".to_string(),
                                JsonValue::String("Swimming".to_string()),
                            ),
                            ("level".to_string(), JsonValue::Number(2.0)),
                        ]),
                    ]),
                ),
            ]),
            JsonValue::Object(vec![
                ("age".to_string(), JsonValue::Number(24.0)),
                ("name".to_string(), JsonValue::String("Bob".to_string())),
                (
                    "hobbies".to_string(),
                    JsonValue::Array(vec![
                        JsonValue::Object(vec![
                            ("name".to_string(), JsonValue::String("Running".to_string())),
                            ("level".to_string(), JsonValue::Number(1.0)),
                        ]),
                        JsonValue::Object(vec![
                            ("name".to_string(), JsonValue::String("Cycling".to_string())),
                            ("level".to_string(), JsonValue::Number(2.0)),
                        ]),
                    ]),
                ),
            ]),
        ]);
        let result = format(&value);
        assert_eq!(
            result,
            r#"[
  "hello",
  {
    "age": 18,
    "name": "Alice",
    "hobbies": [
      {
        "name": "Reading",
        "level": 3
      },
      {
        "name": "Swimming",
        "level": 2
      }
    ]
  },
  {
    "age": 24,
    "name": "Bob",
    "hobbies": [
      {
        "name": "Running",
        "level": 1
      },
      {
        "name": "Cycling",
        "level": 2
      }
    ]
  }
]"#
        );
    }
}
