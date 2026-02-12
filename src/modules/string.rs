//! The `string` module — 20 functions.
//!
//! | Function           | Signature                                              | Description                      |
//! |--------------------|--------------------------------------------------------|----------------------------------|
//! | `string.length`    | `(s: string) -> number`                                | Number of characters             |
//! | `string.concat`    | `(a: string, b: string) -> string`                     | Concatenate two strings          |
//! | `string.contains`  | `(haystack: string, needle: string) -> bool`           | True if needle found             |
//! | `string.slice`     | `(s: string, start: number, end: number) -> string`   | Substring \[start, end)          |
//! | `string.trim`      | `(s: string) -> string`                                | Remove leading/trailing WS       |
//! | `string.split`     | `(s: string, delimiter: string) -> list<string>`       | Split by delimiter               |
//! | `string.to_upper`  | `(s: string) -> string`                                | Uppercase                        |
//! | `string.to_lower`  | `(s: string) -> string`                                | Lowercase                        |
//! | `string.starts_with` | `(s: string, prefix: string) -> bool`                | Prefix check                     |
//! | `string.ends_with` | `(s: string, suffix: string) -> bool`                  | Suffix check                     |
//! | `string.replace`   | `(s: string, old: string, new: string) -> string`     | Replace first occurrence         |
//! | `string.replace_all` | `(s: string, old: string, new: string) -> string`   | Replace all occurrences          |
//! | `string.pad_start` | `(s: string, length: number, pad: string) -> string`  | Left-pad to target length        |
//! | `string.pad_end`   | `(s: string, length: number, pad: string) -> string`  | Right-pad to target length       |
//! | `string.repeat`    | `(s: string, count: number) -> string`                 | Repeat string N times            |
//! | `string.join`      | `(items: list<string>, separator: string) -> string`   | Join list with separator         |
//! | `string.format`    | `(template: string, values: record) -> string`        | `{key}` placeholder replacement  |
//! | `string.from`      | `(value: any) -> string`                               | Any value to string              |
//! | `string.is_empty`  | `(s: string) -> bool`                                  | True if zero length              |
//! | `string.index_of`  | `(s: string, sub: string) -> number`                   | Index of sub, or -1              |

use crate::error::StdlibError;
use crate::module::StdlibModule;
use crate::value::Value;

/// The `string` stdlib module.
pub struct StringModule;

impl StringModule {
    pub fn new() -> Self {
        Self
    }
}

impl Default for StringModule {
    fn default() -> Self {
        Self::new()
    }
}

impl StdlibModule for StringModule {
    fn name(&self) -> &'static str {
        "string"
    }

    fn has_function(&self, function: &str) -> bool {
        matches!(
            function,
            "length"
                | "concat"
                | "contains"
                | "slice"
                | "trim"
                | "split"
                | "to_upper"
                | "to_lower"
                | "starts_with"
                | "ends_with"
                | "replace"
                | "replace_all"
                | "pad_start"
                | "pad_end"
                | "repeat"
                | "join"
                | "format"
                | "from"
                | "is_empty"
                | "index_of"
        )
    }

    fn call(&self, function: &str, args: Vec<Value>) -> Result<Value, StdlibError> {
        match function {
            "length" => self.length(args),
            "concat" => self.concat(args),
            "contains" => self.contains(args),
            "slice" => self.slice(args),
            "trim" => self.trim(args),
            "split" => self.split(args),
            "to_upper" => self.to_upper(args),
            "to_lower" => self.to_lower(args),
            "starts_with" => self.starts_with(args),
            "ends_with" => self.ends_with(args),
            "replace" => self.replace(args),
            "replace_all" => self.replace_all(args),
            "pad_start" => self.pad_start(args),
            "pad_end" => self.pad_end(args),
            "repeat" => self.repeat(args),
            "join" => self.join(args),
            "format" => self.format(args),
            "from" => self.value_to_string(args),
            "is_empty" => self.is_empty(args),
            "index_of" => self.index_of(args),
            _ => Err(StdlibError::unknown_function("string", function)),
        }
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Extract a single string argument.
fn expect_one_string(fn_name: &str, args: &[Value]) -> Result<String, StdlibError> {
    if args.len() != 1 {
        return Err(StdlibError::wrong_args(fn_name, 1, args.len()));
    }
    match &args[0] {
        Value::String(s) => Ok(s.clone()),
        other => Err(StdlibError::type_mismatch(fn_name, 1, "string", other.type_name())),
    }
}

/// Extract two string arguments.
fn expect_two_strings(fn_name: &str, args: &[Value]) -> Result<(String, String), StdlibError> {
    if args.len() != 2 {
        return Err(StdlibError::wrong_args(fn_name, 2, args.len()));
    }
    let a = match &args[0] {
        Value::String(s) => s.clone(),
        other => {
            return Err(StdlibError::type_mismatch(fn_name, 1, "string", other.type_name()));
        }
    };
    let b = match &args[1] {
        Value::String(s) => s.clone(),
        other => {
            return Err(StdlibError::type_mismatch(fn_name, 2, "string", other.type_name()));
        }
    };
    Ok((a, b))
}

/// Extract three string arguments.
fn expect_three_strings(
    fn_name: &str,
    args: &[Value],
) -> Result<(String, String, String), StdlibError> {
    if args.len() != 3 {
        return Err(StdlibError::wrong_args(fn_name, 3, args.len()));
    }
    let a = match &args[0] {
        Value::String(s) => s.clone(),
        other => {
            return Err(StdlibError::type_mismatch(fn_name, 1, "string", other.type_name()));
        }
    };
    let b = match &args[1] {
        Value::String(s) => s.clone(),
        other => {
            return Err(StdlibError::type_mismatch(fn_name, 2, "string", other.type_name()));
        }
    };
    let c = match &args[2] {
        Value::String(s) => s.clone(),
        other => {
            return Err(StdlibError::type_mismatch(fn_name, 3, "string", other.type_name()));
        }
    };
    Ok((a, b, c))
}

// ── Function implementations ──────────────────────────────────────────────────

impl StringModule {
    /// `string.length(s: string) -> number`
    ///
    /// Returns the number of Unicode characters (not bytes).
    fn length(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        let s = expect_one_string("string.length", &args)?;
        Ok(Value::Number(s.chars().count() as f64))
    }

    /// `string.concat(a: string, b: string) -> string`
    fn concat(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        let (a, b) = expect_two_strings("string.concat", &args)?;
        Ok(Value::String(format!("{a}{b}")))
    }

    /// `string.contains(haystack: string, needle: string) -> bool`
    fn contains(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        let (haystack, needle) = expect_two_strings("string.contains", &args)?;
        Ok(Value::Bool(haystack.contains(&needle)))
    }

    /// `string.slice(s: string, start: number, end: number) -> string`
    ///
    /// Substring from start (inclusive) to end (exclusive).
    /// Indices are character-based (not byte-based).
    /// Clamps out-of-range indices to valid bounds.
    fn slice(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 3 {
            return Err(StdlibError::wrong_args("string.slice", 3, args.len()));
        }
        let s = match &args[0] {
            Value::String(s) => s.clone(),
            other => {
                return Err(StdlibError::type_mismatch(
                    "string.slice", 1, "string", other.type_name(),
                ));
            }
        };
        let start = match &args[1] {
            Value::Number(n) => *n,
            other => {
                return Err(StdlibError::type_mismatch(
                    "string.slice", 2, "number", other.type_name(),
                ));
            }
        };
        let end = match &args[2] {
            Value::Number(n) => *n,
            other => {
                return Err(StdlibError::type_mismatch(
                    "string.slice", 3, "number", other.type_name(),
                ));
            }
        };

        let len = s.chars().count() as isize;
        let start = (start as isize).clamp(0, len) as usize;
        let end = (end as isize).clamp(0, len) as usize;

        if start >= end {
            return Ok(Value::String(String::new()));
        }

        let result: String = s.chars().skip(start).take(end - start).collect();
        Ok(Value::String(result))
    }

    /// `string.trim(s: string) -> string`
    fn trim(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        let s = expect_one_string("string.trim", &args)?;
        Ok(Value::String(s.trim().to_string()))
    }

    /// `string.split(s: string, delimiter: string) -> list<string>`
    fn split(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        let (s, delimiter) = expect_two_strings("string.split", &args)?;
        let parts: Vec<Value> = if delimiter.is_empty() {
            // Split on empty delimiter → each character becomes an element
            s.chars().map(|c| Value::String(c.to_string())).collect()
        } else {
            s.split(&delimiter)
                .map(|part| Value::String(part.to_string()))
                .collect()
        };
        Ok(Value::List(parts))
    }

    /// `string.to_upper(s: string) -> string`
    fn to_upper(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        let s = expect_one_string("string.to_upper", &args)?;
        Ok(Value::String(s.to_uppercase()))
    }

    /// `string.to_lower(s: string) -> string`
    fn to_lower(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        let s = expect_one_string("string.to_lower", &args)?;
        Ok(Value::String(s.to_lowercase()))
    }

    /// `string.starts_with(s: string, prefix: string) -> bool`
    fn starts_with(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        let (s, prefix) = expect_two_strings("string.starts_with", &args)?;
        Ok(Value::Bool(s.starts_with(&prefix)))
    }

    /// `string.ends_with(s: string, suffix: string) -> bool`
    fn ends_with(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        let (s, suffix) = expect_two_strings("string.ends_with", &args)?;
        Ok(Value::Bool(s.ends_with(&suffix)))
    }

    /// `string.replace(s: string, old: string, new: string) -> string`
    ///
    /// Replace first occurrence only.
    fn replace(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        let (s, old, new) = expect_three_strings("string.replace", &args)?;
        if old.is_empty() {
            // Replacing empty string → return original (no-op)
            return Ok(Value::String(s));
        }
        let result = if let Some(pos) = s.find(&old) {
            format!("{}{new}{}", &s[..pos], &s[pos + old.len()..])
        } else {
            s
        };
        Ok(Value::String(result))
    }

    /// `string.replace_all(s: string, old: string, new: string) -> string`
    fn replace_all(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        let (s, old, new) = expect_three_strings("string.replace_all", &args)?;
        if old.is_empty() {
            return Ok(Value::String(s));
        }
        Ok(Value::String(s.replace(&old, &new)))
    }

    /// `string.pad_start(s: string, length: number, pad: string) -> string`
    ///
    /// Pad string on the left to reach target length. If already >= length,
    /// returns original. The pad string is repeated/truncated as needed.
    fn pad_start(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 3 {
            return Err(StdlibError::wrong_args("string.pad_start", 3, args.len()));
        }
        let s = match &args[0] {
            Value::String(s) => s.clone(),
            other => {
                return Err(StdlibError::type_mismatch(
                    "string.pad_start", 1, "string", other.type_name(),
                ));
            }
        };
        let target_len = match &args[1] {
            Value::Number(n) => *n,
            other => {
                return Err(StdlibError::type_mismatch(
                    "string.pad_start", 2, "number", other.type_name(),
                ));
            }
        };
        let pad = match &args[2] {
            Value::String(s) => s.clone(),
            other => {
                return Err(StdlibError::type_mismatch(
                    "string.pad_start", 3, "string", other.type_name(),
                ));
            }
        };

        let current_len = s.chars().count();
        let target_len = target_len as usize;

        if current_len >= target_len || pad.is_empty() {
            return Ok(Value::String(s));
        }

        let needed = target_len - current_len;
        let padding: String = pad.chars().cycle().take(needed).collect();
        Ok(Value::String(format!("{padding}{s}")))
    }

    /// `string.pad_end(s: string, length: number, pad: string) -> string`
    ///
    /// Pad string on the right to reach target length.
    fn pad_end(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 3 {
            return Err(StdlibError::wrong_args("string.pad_end", 3, args.len()));
        }
        let s = match &args[0] {
            Value::String(s) => s.clone(),
            other => {
                return Err(StdlibError::type_mismatch(
                    "string.pad_end", 1, "string", other.type_name(),
                ));
            }
        };
        let target_len = match &args[1] {
            Value::Number(n) => *n,
            other => {
                return Err(StdlibError::type_mismatch(
                    "string.pad_end", 2, "number", other.type_name(),
                ));
            }
        };
        let pad = match &args[2] {
            Value::String(s) => s.clone(),
            other => {
                return Err(StdlibError::type_mismatch(
                    "string.pad_end", 3, "string", other.type_name(),
                ));
            }
        };

        let current_len = s.chars().count();
        let target_len = target_len as usize;

        if current_len >= target_len || pad.is_empty() {
            return Ok(Value::String(s));
        }

        let needed = target_len - current_len;
        let padding: String = pad.chars().cycle().take(needed).collect();
        Ok(Value::String(format!("{s}{padding}")))
    }

    /// `string.repeat(s: string, count: number) -> string`
    fn repeat(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 2 {
            return Err(StdlibError::wrong_args("string.repeat", 2, args.len()));
        }
        let s = match &args[0] {
            Value::String(s) => s.clone(),
            other => {
                return Err(StdlibError::type_mismatch(
                    "string.repeat", 1, "string", other.type_name(),
                ));
            }
        };
        let count = match &args[1] {
            Value::Number(n) => *n,
            other => {
                return Err(StdlibError::type_mismatch(
                    "string.repeat", 2, "number", other.type_name(),
                ));
            }
        };

        if count < 0.0 || count.fract() != 0.0 {
            return Err(StdlibError::RuntimeError(
                "string.repeat: count must be a non-negative integer".to_string(),
            ));
        }

        Ok(Value::String(s.repeat(count as usize)))
    }

    /// `string.join(items: list<string>, separator: string) -> string`
    fn join(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 2 {
            return Err(StdlibError::wrong_args("string.join", 2, args.len()));
        }
        let items = match &args[0] {
            Value::List(l) => l.clone(),
            other => {
                return Err(StdlibError::type_mismatch(
                    "string.join", 1, "list", other.type_name(),
                ));
            }
        };
        let separator = match &args[1] {
            Value::String(s) => s.clone(),
            other => {
                return Err(StdlibError::type_mismatch(
                    "string.join", 2, "string", other.type_name(),
                ));
            }
        };

        let mut parts = Vec::with_capacity(items.len());
        for (i, item) in items.iter().enumerate() {
            match item {
                Value::String(s) => parts.push(s.clone()),
                other => {
                    return Err(StdlibError::TypeMismatch {
                        function: "string.join".to_string(),
                        position: i + 1,
                        expected: "string".to_string(),
                        got: other.type_name().to_string(),
                    });
                }
            }
        }

        Ok(Value::String(parts.join(&separator)))
    }

    /// `string.format(template: string, values: record) -> string`
    ///
    /// Replace `{key}` placeholders in template with values from the record.
    /// Unrecognized placeholders are left as-is.
    fn format(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 2 {
            return Err(StdlibError::wrong_args("string.format", 2, args.len()));
        }
        let template = match &args[0] {
            Value::String(s) => s.clone(),
            other => {
                return Err(StdlibError::type_mismatch(
                    "string.format", 1, "string", other.type_name(),
                ));
            }
        };
        let fields = match &args[1] {
            Value::Record { fields, .. } => fields.clone(),
            other => {
                return Err(StdlibError::type_mismatch(
                    "string.format", 2, "record", other.type_name(),
                ));
            }
        };

        let mut result = template;
        for (key, val) in &fields {
            let placeholder = format!("{{{key}}}");
            let replacement = format!("{val}");
            result = result.replace(&placeholder, &replacement);
        }

        Ok(Value::String(result))
    }

    /// `string.from(value: any) -> string`
    ///
    /// Convert any value to its string representation. Uses Display impl.
    fn value_to_string(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 1 {
            return Err(StdlibError::wrong_args("string.from", 1, args.len()));
        }
        Ok(Value::String(format!("{}", args[0])))
    }

    /// `string.is_empty(s: string) -> bool`
    fn is_empty(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        let s = expect_one_string("string.is_empty", &args)?;
        Ok(Value::Bool(s.is_empty()))
    }

    /// `string.index_of(s: string, sub: string) -> number`
    ///
    /// Returns the character index of the first occurrence of `sub` in `s`,
    /// or -1 if not found. Index is character-based (not byte-based).
    fn index_of(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        let (s, sub) = expect_two_strings("string.index_of", &args)?;
        if sub.is_empty() {
            return Ok(Value::Number(0.0));
        }
        // Find byte position, then convert to char index
        match s.find(&sub) {
            Some(byte_pos) => {
                let char_index = s[..byte_pos].chars().count();
                Ok(Value::Number(char_index as f64))
            }
            None => Ok(Value::Number(-1.0)),
        }
    }
}
