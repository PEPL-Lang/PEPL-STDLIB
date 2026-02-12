//! The `core` module — 4 functions.
//!
//! | Function | Signature | Description |
//! |----------|-----------|-------------|
//! | `core.log` | `(value: any) -> nil` | Debug logging (no-op in production) |
//! | `core.assert` | `(condition: bool, message?: string) -> nil` | Trap if false |
//! | `core.type_of` | `(value: any) -> string` | Returns type name |
//! | `core.capability` | `(name: string) -> bool` | Check capability availability |

use crate::error::StdlibError;
use crate::module::StdlibModule;
use crate::value::Value;

/// The `core` stdlib module.
pub struct CoreModule;

impl CoreModule {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CoreModule {
    fn default() -> Self {
        Self::new()
    }
}

impl StdlibModule for CoreModule {
    fn name(&self) -> &'static str {
        "core"
    }

    fn has_function(&self, function: &str) -> bool {
        matches!(function, "log" | "assert" | "type_of" | "capability")
    }

    fn call(&self, function: &str, args: Vec<Value>) -> Result<Value, StdlibError> {
        match function {
            "log" => self.log(args),
            "assert" => self.assert(args),
            "type_of" => self.type_of(args),
            "capability" => self.capability(args),
            _ => Err(StdlibError::unknown_function("core", function)),
        }
    }
}

impl CoreModule {
    /// `core.log(value: any) -> nil`
    ///
    /// Debug logging. In production this is a no-op. In dev/test, the value
    /// is printed to stderr. Always returns `Nil`.
    fn log(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 1 {
            return Err(StdlibError::wrong_args("core.log", 1, args.len()));
        }
        // No-op in production — the value is consumed but not output.
        // A dev/test host can intercept this via a log callback.
        Ok(Value::Nil)
    }

    /// `core.assert(condition: bool, message?: string) -> nil`
    ///
    /// Traps (returns error) if condition is false. The optional message
    /// provides context in test output.
    fn assert(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.is_empty() || args.len() > 2 {
            return Err(StdlibError::wrong_args("core.assert", 1, args.len()));
        }

        let condition = match &args[0] {
            Value::Bool(b) => *b,
            other => {
                return Err(StdlibError::type_mismatch(
                    "core.assert",
                    1,
                    "bool",
                    other.type_name(),
                ));
            }
        };

        if let Some(msg_val) = args.get(1) {
            if !matches!(msg_val, Value::String(_)) {
                return Err(StdlibError::type_mismatch(
                    "core.assert",
                    2,
                    "string",
                    msg_val.type_name(),
                ));
            }
        }

        if !condition {
            let message = match args.get(1) {
                Some(Value::String(s)) => s.clone(),
                _ => "assertion failed".to_string(),
            };
            return Err(StdlibError::AssertionFailed { message });
        }

        Ok(Value::Nil)
    }

    /// `core.type_of(value: any) -> string`
    ///
    /// Returns the type name: "number", "string", "bool", "nil", "list", "record".
    fn type_of(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 1 {
            return Err(StdlibError::wrong_args("core.type_of", 1, args.len()));
        }
        Ok(Value::String(args[0].type_name().to_string()))
    }

    /// `core.capability(name: string) -> bool`
    ///
    /// Returns whether a declared optional capability is available at runtime.
    /// In Phase 0, no capabilities are declared, so this always returns `false`.
    fn capability(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 1 {
            return Err(StdlibError::wrong_args("core.capability", 1, args.len()));
        }
        match &args[0] {
            Value::String(_) => {
                // Phase 0: no capabilities are ever available
                Ok(Value::Bool(false))
            }
            other => Err(StdlibError::type_mismatch(
                "core.capability",
                1,
                "string",
                other.type_name(),
            )),
        }
    }
}
