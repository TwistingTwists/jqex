use std::io::{self, Read};
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    match serde_json::from_str(&input) {
        Ok(json) => {
            println!("{}", format_as_elixir_map(&json));
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}

fn format_as_elixir_map(value: &Value) -> String {
    match value {
        Value::Null => "nil".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => format!("\"{}\"", s),
        Value::Array(arr) => {
            let elements: Vec<String> = arr.iter().map(format_as_elixir_map).collect();
            format!("[{}]", elements.join(", "))
        }
        Value::Object(obj) => {
            let pairs: Vec<String> = obj
                .iter()
                .map(|(k, v)| format!("\"{}\" => {}", k, format_as_elixir_map(v)))
                .collect();
            format!("%{{{}}}", pairs.join(", "))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_format_null() {
        assert_eq!(format_as_elixir_map(&json!(null)), r#"nil"#);
    }

    #[test]
    fn test_format_boolean() {
        assert_eq!(format_as_elixir_map(&json!(true)), r#"true"#);
        assert_eq!(format_as_elixir_map(&json!(false)), r#"false"#);
    }

    #[test]
    fn test_format_number() {
        assert_eq!(format_as_elixir_map(&json!(42)), r#"42"#);
        assert_eq!(format_as_elixir_map(&json!(3.14)), r#"3.14"#);
    }

    #[test]
    fn test_format_string() {
        assert_eq!(format_as_elixir_map(&json!("hello")), r#""hello""#);
    }

    #[test]
    fn test_format_array() {
        assert_eq!(format_as_elixir_map(&json!([1, 2, 3])), r#"[1, 2, 3]"#);
        assert_eq!(format_as_elixir_map(&json!(["a", "b", "c"])), r#"["a", "b", "c"]"#);
    }

    #[test]
    fn test_format_object() {
        let json = json!({"name": "John", "age": 30});
        let expected = r#"%{"age" => 30, "name" => "John"}"#;
        assert_eq!(format_as_elixir_map(&json), expected);
    }

    #[test]
    fn test_format_nested() {
        let json = json!({
            "person": {
                "name": "Alice",
                "age": 25,
                "balance": 2500.0,
                "hobbies": ["reading", "swimming"]
            },
            "is_student": true
        });
        let expected = r#"%{"is_student" => true, "person" => %{"age" => 25, "balance" => 2500.0, "hobbies" => ["reading", "swimming"], "name" => "Alice"}}"#;
        assert_eq!(format_as_elixir_map(&json), expected);
    }
}