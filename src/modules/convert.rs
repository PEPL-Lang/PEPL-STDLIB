//! `convert` stdlib module — type conversion utilities.
//!
//! Functions: to_string, to_number, parse_int, parse_float, to_bool.

use crate::error::StdlibError;
use crate::module::StdlibModule;
use crate::value::Value;

/// The `convert` stdlib module.
pub struct ConvertModule;

impl ConvertModule {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ConvertModule {
    fn default() -> Self {
        Self::new()
    }
}

impl StdlibModule for ConvertModule {
    fn name(&self) -> &'static str {
        "convert"
    }

    fn has_function(&self, function: &str) -> bool {
        matches!(
            function,
            "to_string" | "to_number" | "parse_int" | "parse_float" | "to_bool"
        )
    }

    fn call(&self, function: &str, args: Vec<Value>) -> Result<Value, StdlibError> {
        match function {
            "to_string" => self.to_string_fn(args),
            "to_number" => self.to_number(args),
            "parse_int" => self.parse_int(args),
            "parse_float" => self.parse_float(args),
            "to_bool" => self.to_bool(args),
            _ => Err(StdlibError::unknown_function("convert", function)),
        }
    }
}

impl ConvertModule {
    /// convert.to_string(value) → string
    /// Always succeeds — uses Value's Display impl.
    fn to_string_fn(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 1 {
            return Err(StdlibError::wrong_args("convert.to_string", 1, args.len()));
        }
        Ok(Value::String(format!("{}", args[0])))
    }

    /// convert.to_number(value) → Result<number, string>
    /// - Number → Ok(identity)
    /// - Bool → Ok(0 or 1)
    /// - String → parse as f64
    /// - Nil → Err("cannot convert nil to number")
    /// - Other → Err
    fn to_number(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 1 {
            return Err(StdlibError::wrong_args("convert.to_number", 1, args.len()));
        }
        match &args[0] {
            Value::Number(n) => Ok(Value::Number(*n).ok()),
            Value::Bool(b) => Ok(Value::Number(if *b { 1.0 } else { 0.0 }).ok()),
            Value::String(s) => match s.trim().parse::<f64>() {
                Ok(n) if n.is_finite() => Ok(Value::Number(n).ok()),
                _ => Ok(Value::String(format!("cannot convert '{}' to number", s)).err()),
            },
            other => {
                Ok(Value::String(format!("cannot convert {} to number", other.type_name())).err())
            }
        }
    }

    /// convert.parse_int(s) → Result<number, string>
    /// Parses an integer string. Rejects floats.
    fn parse_int(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 1 {
            return Err(StdlibError::wrong_args("convert.parse_int", 1, args.len()));
        }
        let s = extract_string("convert.parse_int", &args[0], 1)?;
        match s.trim().parse::<i64>() {
            Ok(n) => Ok(Value::Number(n as f64).ok()),
            Err(_) => Ok(Value::String(format!("cannot parse '{}' as integer", s)).err()),
        }
    }

    /// convert.parse_float(s) → Result<number, string>
    fn parse_float(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 1 {
            return Err(StdlibError::wrong_args(
                "convert.parse_float",
                1,
                args.len(),
            ));
        }
        let s = extract_string("convert.parse_float", &args[0], 1)?;
        match s.trim().parse::<f64>() {
            Ok(n) if n.is_finite() => Ok(Value::Number(n).ok()),
            _ => Ok(Value::String(format!("cannot parse '{}' as float", s)).err()),
        }
    }

    /// convert.to_bool(value) → bool
    /// Uses truthiness: false, nil, 0, "" are falsy; everything else is truthy.
    fn to_bool(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 1 {
            return Err(StdlibError::wrong_args("convert.to_bool", 1, args.len()));
        }
        Ok(Value::Bool(args[0].is_truthy()))
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
