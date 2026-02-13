//! `record` stdlib module — immutable record operations.
//!
//! Functions: get, set, has, keys, values.

use std::collections::BTreeMap;

use crate::error::StdlibError;
use crate::module::StdlibModule;
use crate::value::Value;

/// The `record` stdlib module.
pub struct RecordModule;

impl RecordModule {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RecordModule {
    fn default() -> Self {
        Self::new()
    }
}

impl StdlibModule for RecordModule {
    fn name(&self) -> &'static str {
        "record"
    }

    fn has_function(&self, function: &str) -> bool {
        matches!(function, "get" | "set" | "has" | "keys" | "values")
    }

    fn call(&self, function: &str, args: Vec<Value>) -> Result<Value, StdlibError> {
        match function {
            "get" => self.get(args),
            "set" => self.set(args),
            "has" => self.has(args),
            "keys" => self.keys(args),
            "values" => self.values(args),
            _ => Err(StdlibError::unknown_function("record", function)),
        }
    }
}

impl RecordModule {
    /// record.get(rec, key) → any
    /// Returns the value for `key`, or Nil if not present.
    fn get(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 2 {
            return Err(StdlibError::wrong_args("record.get", 2, args.len()));
        }
        let fields = extract_record("record.get", &args[0], 1)?;
        let key = extract_string("record.get", &args[1], 2)?;
        Ok(fields.get(key).cloned().unwrap_or(Value::Nil))
    }

    /// record.set(rec, key, value) → record
    /// Returns a new record with the key set to value.
    fn set(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 3 {
            return Err(StdlibError::wrong_args("record.set", 3, args.len()));
        }
        let fields = extract_record("record.set", &args[0], 1)?;
        let key = extract_string("record.set", &args[1], 2)?;
        let mut new_fields = fields.clone();
        new_fields.insert(key.to_string(), args[2].clone());
        Ok(Value::record(new_fields))
    }

    /// record.has(rec, key) → bool
    fn has(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 2 {
            return Err(StdlibError::wrong_args("record.has", 2, args.len()));
        }
        let fields = extract_record("record.has", &args[0], 1)?;
        let key = extract_string("record.has", &args[1], 2)?;
        Ok(Value::Bool(fields.contains_key(key)))
    }

    /// record.keys(rec) → list<string>
    /// Returns keys in deterministic BTreeMap order.
    fn keys(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 1 {
            return Err(StdlibError::wrong_args("record.keys", 1, args.len()));
        }
        let fields = extract_record("record.keys", &args[0], 1)?;
        let keys: Vec<Value> = fields.keys().map(|k| Value::String(k.clone())).collect();
        Ok(Value::List(keys))
    }

    /// record.values(rec) → list<any>
    /// Returns values in deterministic BTreeMap order.
    fn values(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 1 {
            return Err(StdlibError::wrong_args("record.values", 1, args.len()));
        }
        let fields = extract_record("record.values", &args[0], 1)?;
        let values: Vec<Value> = fields.values().cloned().collect();
        Ok(Value::List(values))
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn extract_record<'a>(
    func: &str,
    val: &'a Value,
    pos: usize,
) -> Result<&'a BTreeMap<String, Value>, StdlibError> {
    match val {
        Value::Record { fields, .. } => Ok(fields),
        _ => Err(StdlibError::type_mismatch(
            func,
            pos,
            "record",
            val.type_name(),
        )),
    }
}

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
