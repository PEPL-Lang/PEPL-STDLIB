//! `json` stdlib module — JSON parsing and serialization.
//!
//! Functions: parse, stringify.
//! Max parse depth: 32 (prevents stack overflow on deeply nested JSON).

use std::collections::BTreeMap;

use crate::error::StdlibError;
use crate::module::StdlibModule;
use crate::value::{ResultValue, Value};

/// Maximum allowed nesting depth when parsing JSON.
const MAX_DEPTH: usize = 32;

/// The `json` stdlib module.
pub struct JsonModule;

impl JsonModule {
    pub fn new() -> Self {
        Self
    }
}

impl Default for JsonModule {
    fn default() -> Self {
        Self::new()
    }
}

impl StdlibModule for JsonModule {
    fn name(&self) -> &'static str {
        "json"
    }

    fn has_function(&self, function: &str) -> bool {
        matches!(function, "parse" | "stringify")
    }

    fn call(&self, function: &str, args: Vec<Value>) -> Result<Value, StdlibError> {
        match function {
            "parse" => self.parse(args),
            "stringify" => self.stringify(args),
            _ => Err(StdlibError::unknown_function("json", function)),
        }
    }
}

impl JsonModule {
    /// json.parse(s) → Result<any, string>
    /// Parses a JSON string into a PEPL Value.
    fn parse(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 1 {
            return Err(StdlibError::wrong_args("json.parse", 1, args.len()));
        }
        let s = extract_string("json.parse", &args[0], 1)?;

        match serde_json::from_str::<serde_json::Value>(s) {
            Ok(json_val) => match json_to_value(&json_val, 0) {
                Ok(v) => Ok(v.ok()),
                Err(msg) => Ok(Value::String(msg).err()),
            },
            Err(e) => Ok(Value::String(format!("JSON parse error: {}", e)).err()),
        }
    }

    /// json.stringify(value) → string
    /// Converts a PEPL Value to a JSON string.
    fn stringify(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 1 {
            return Err(StdlibError::wrong_args("json.stringify", 1, args.len()));
        }
        let json_val = value_to_json(&args[0]);
        Ok(Value::String(
            serde_json::to_string(&json_val).unwrap_or_else(|_| "null".to_string()),
        ))
    }
}

// ── JSON ↔ Value conversion ────────────────────────────────────────────────

/// Convert a serde_json::Value to a PEPL Value, respecting depth limits.
fn json_to_value(json: &serde_json::Value, depth: usize) -> Result<Value, String> {
    if depth > MAX_DEPTH {
        return Err(format!(
            "JSON nesting exceeds maximum depth of {}",
            MAX_DEPTH
        ));
    }

    match json {
        serde_json::Value::Null => Ok(Value::Nil),
        serde_json::Value::Bool(b) => Ok(Value::Bool(*b)),
        serde_json::Value::Number(n) => Ok(Value::Number(n.as_f64().unwrap_or(0.0))),
        serde_json::Value::String(s) => Ok(Value::String(s.clone())),
        serde_json::Value::Array(arr) => {
            let mut items = Vec::with_capacity(arr.len());
            for item in arr {
                items.push(json_to_value(item, depth + 1)?);
            }
            Ok(Value::List(items))
        }
        serde_json::Value::Object(obj) => {
            let mut fields = BTreeMap::new();
            for (key, val) in obj {
                fields.insert(key.clone(), json_to_value(val, depth + 1)?);
            }
            Ok(Value::record(fields))
        }
    }
}

/// Convert a PEPL Value to a serde_json::Value for serialization.
fn value_to_json(value: &Value) -> serde_json::Value {
    match value {
        Value::Nil => serde_json::Value::Null,
        Value::Bool(b) => serde_json::Value::Bool(*b),
        Value::Number(n) => {
            if n.is_finite() {
                serde_json::Value::Number(
                    serde_json::Number::from_f64(*n).unwrap_or_else(|| serde_json::Number::from(0)),
                )
            } else {
                serde_json::Value::Null // NaN/Infinity → null
            }
        }
        Value::String(s) => serde_json::Value::String(s.clone()),
        Value::List(items) => serde_json::Value::Array(items.iter().map(value_to_json).collect()),
        Value::Record { fields, .. } => {
            let obj: serde_json::Map<String, serde_json::Value> = fields
                .iter()
                .map(|(k, v)| (k.clone(), value_to_json(v)))
                .collect();
            serde_json::Value::Object(obj)
        }
        Value::Color { r, g, b, a } => {
            let mut obj = serde_json::Map::new();
            obj.insert("r".into(), value_to_json(&Value::Number(*r)));
            obj.insert("g".into(), value_to_json(&Value::Number(*g)));
            obj.insert("b".into(), value_to_json(&Value::Number(*b)));
            obj.insert("a".into(), value_to_json(&Value::Number(*a)));
            serde_json::Value::Object(obj)
        }
        Value::Result(rv) => match rv.as_ref() {
            ResultValue::Ok(v) => {
                let mut obj = serde_json::Map::new();
                obj.insert("ok".into(), value_to_json(v));
                serde_json::Value::Object(obj)
            }
            ResultValue::Err(v) => {
                let mut obj = serde_json::Map::new();
                obj.insert("err".into(), value_to_json(v));
                serde_json::Value::Object(obj)
            }
        },
        Value::SumVariant {
            type_name,
            variant,
            fields,
        } => {
            let mut obj = serde_json::Map::new();
            obj.insert("_type".into(), serde_json::Value::String(type_name.clone()));
            obj.insert(
                "_variant".into(),
                serde_json::Value::String(variant.clone()),
            );
            if !fields.is_empty() {
                obj.insert(
                    "_fields".into(),
                    serde_json::Value::Array(fields.iter().map(value_to_json).collect()),
                );
            }
            serde_json::Value::Object(obj)
        }
        Value::Function(_) => serde_json::Value::String("<function>".to_string()),
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn extract_string<'a>(func: &str, val: &'a Value, pos: usize) -> Result<&'a str, StdlibError> {
    match val {
        Value::String(s) => Ok(s),
        _ => Err(StdlibError::type_mismatch(
            func,
            pos,
            "string",
            val.type_name(),
        )),
    }
}
