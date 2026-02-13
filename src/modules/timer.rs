//! `timer` stdlib module — timer management (host-delegated).
//!
//! Functions: start, start_once, stop, stop_all.
//! All timer operations are host-delegated stubs — the runtime host
//! implements actual scheduling. This module validates arguments and
//! returns the expected types.

use crate::error::StdlibError;
use crate::module::StdlibModule;
use crate::value::Value;

/// The `timer` stdlib module.
pub struct TimerModule;

impl TimerModule {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TimerModule {
    fn default() -> Self {
        Self::new()
    }
}

impl StdlibModule for TimerModule {
    fn name(&self) -> &'static str {
        "timer"
    }

    fn has_function(&self, function: &str) -> bool {
        matches!(function, "start" | "start_once" | "stop" | "stop_all")
    }

    fn call(&self, function: &str, args: Vec<Value>) -> Result<Value, StdlibError> {
        match function {
            "start" => self.start(args),
            "start_once" => self.start_once(args),
            "stop" => self.stop(args),
            "stop_all" => self.stop_all(args),
            _ => Err(StdlibError::unknown_function("timer", function)),
        }
    }
}

impl TimerModule {
    /// timer.start(id, interval_ms) → string
    /// Starts a repeating timer. Returns the timer ID.
    /// In the stdlib, this is a stub that validates args and returns the ID.
    fn start(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 2 {
            return Err(StdlibError::wrong_args("timer.start", 2, args.len()));
        }
        let id = extract_string("timer.start", &args[0], 1)?;
        let _interval = extract_number("timer.start", &args[1], 2)?;
        Ok(Value::String(id.to_string()))
    }

    /// timer.start_once(id, delay_ms) → string
    /// Starts a one-shot timer. Returns the timer ID.
    fn start_once(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 2 {
            return Err(StdlibError::wrong_args("timer.start_once", 2, args.len()));
        }
        let id = extract_string("timer.start_once", &args[0], 1)?;
        let _delay = extract_number("timer.start_once", &args[1], 2)?;
        Ok(Value::String(id.to_string()))
    }

    /// timer.stop(id) → nil
    /// Stops a timer by ID. No-op if the timer doesn't exist.
    fn stop(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 1 {
            return Err(StdlibError::wrong_args("timer.stop", 1, args.len()));
        }
        let _id = extract_string("timer.stop", &args[0], 1)?;
        Ok(Value::Nil)
    }

    /// timer.stop_all() → nil
    /// Stops all active timers.
    fn stop_all(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if !args.is_empty() {
            return Err(StdlibError::wrong_args("timer.stop_all", 0, args.len()));
        }
        Ok(Value::Nil)
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn extract_string<'a>(func: &str, val: &'a Value, pos: usize) -> Result<&'a str, StdlibError> {
    match val {
        Value::String(s) => Ok(s),
        _ => Err(StdlibError::type_mismatch(func, pos, "string", val.type_name())),
    }
}

fn extract_number(func: &str, val: &Value, pos: usize) -> Result<f64, StdlibError> {
    match val {
        Value::Number(n) => Ok(*n),
        _ => Err(StdlibError::type_mismatch(func, pos, "number", val.type_name())),
    }
}
