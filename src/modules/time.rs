//! `time` stdlib module — timestamp operations.
//!
//! All timestamps are milliseconds since Unix epoch as f64.
//! Functions: now, format, diff, day_of_week, start_of_day.

use crate::error::StdlibError;
use crate::module::StdlibModule;
use crate::value::Value;

/// Milliseconds per day.
const MS_PER_DAY: f64 = 86_400_000.0;
/// Milliseconds per second.
const MS_PER_SECOND: f64 = 1_000.0;

/// The `time` stdlib module.
pub struct TimeModule;

impl TimeModule {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TimeModule {
    fn default() -> Self {
        Self::new()
    }
}

impl StdlibModule for TimeModule {
    fn name(&self) -> &'static str {
        "time"
    }

    fn has_function(&self, function: &str) -> bool {
        matches!(
            function,
            "now" | "format" | "diff" | "day_of_week" | "start_of_day"
        )
    }

    fn call(&self, function: &str, args: Vec<Value>) -> Result<Value, StdlibError> {
        match function {
            "now" => self.now(args),
            "format" => self.format(args),
            "diff" => self.diff(args),
            "day_of_week" => self.day_of_week(args),
            "start_of_day" => self.start_of_day(args),
            _ => Err(StdlibError::unknown_function("time", function)),
        }
    }
}

impl TimeModule {
    /// time.now() → number
    /// Returns a deterministic stub value (0). In production, the host injects
    /// the current timestamp via `env.host_call`.
    fn now(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if !args.is_empty() {
            return Err(StdlibError::wrong_args("time.now", 0, args.len()));
        }
        // Deterministic stub — host provides real value at runtime
        Ok(Value::Number(0.0))
    }

    /// time.format(timestamp, pattern) → string
    /// Supports patterns: "YYYY-MM-DD", "HH:mm:ss", "HH:mm",
    /// "YYYY-MM-DD HH:mm:ss", and others via placeholder replacement.
    fn format(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 2 {
            return Err(StdlibError::wrong_args("time.format", 2, args.len()));
        }
        let ts = extract_number("time.format", &args[0], 1)?;
        let pattern = extract_string("time.format", &args[1], 2)?;

        let (year, month, day, hour, min, sec) = timestamp_to_parts(ts);

        let result = pattern
            .replace("YYYY", &format!("{:04}", year))
            .replace("MM", &format!("{:02}", month))
            .replace("DD", &format!("{:02}", day))
            .replace("HH", &format!("{:02}", hour))
            .replace("mm", &format!("{:02}", min))
            .replace("ss", &format!("{:02}", sec));

        Ok(Value::String(result))
    }

    /// time.diff(a, b) → number
    /// Returns `a - b` in milliseconds.
    fn diff(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 2 {
            return Err(StdlibError::wrong_args("time.diff", 2, args.len()));
        }
        let a = extract_number("time.diff", &args[0], 1)?;
        let b = extract_number("time.diff", &args[1], 2)?;
        Ok(Value::Number(a - b))
    }

    /// time.day_of_week(timestamp) → number
    /// Returns 0 (Sunday) through 6 (Saturday).
    /// Uses the fact that Unix epoch (Jan 1, 1970) was a Thursday (4).
    fn day_of_week(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 1 {
            return Err(StdlibError::wrong_args("time.day_of_week", 1, args.len()));
        }
        let ts = extract_number("time.day_of_week", &args[0], 1)?;
        // Days since epoch, Thursday = 4
        let days = (ts / MS_PER_DAY).floor() as i64;
        // (days + 4) % 7 — epoch was Thursday
        let dow = ((days % 7 + 4) % 7 + 7) % 7;
        Ok(Value::Number(dow as f64))
    }

    /// time.start_of_day(timestamp) → number
    /// Truncates to midnight (UTC).
    fn start_of_day(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 1 {
            return Err(StdlibError::wrong_args("time.start_of_day", 1, args.len()));
        }
        let ts = extract_number("time.start_of_day", &args[0], 1)?;
        let day_start = (ts / MS_PER_DAY).floor() * MS_PER_DAY;
        Ok(Value::Number(day_start))
    }
}

// ── Date arithmetic helpers ─────────────────────────────────────────────────

/// Convert a UTC millisecond timestamp to (year, month, day, hour, min, sec).
/// Uses a civil calendar algorithm (no external dependencies).
fn timestamp_to_parts(ts: f64) -> (i64, u32, u32, u32, u32, u32) {
    let total_ms = ts as i64;
    let total_sec = total_ms.div_euclid(MS_PER_SECOND as i64);
    let sec = total_sec.rem_euclid(60) as u32;
    let total_min = total_sec.div_euclid(60);
    let min = total_min.rem_euclid(60) as u32;
    let total_hour = total_min.div_euclid(60);
    let hour = total_hour.rem_euclid(24) as u32;

    let days = total_hour.div_euclid(24);
    let (year, month, day) = days_to_civil(days);
    (year, month, day, hour, min, sec)
}

/// Convert days since Unix epoch to (year, month, day).
/// Algorithm from Howard Hinnant's `chrono`-compatible civil calendar.
fn days_to_civil(days: i64) -> (i64, u32, u32) {
    let z = days + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u32; // day of era [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365; // year of era
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // day of year [0, 365]
    let mp = (5 * doy + 2) / 153; // month indicator [0, 11]
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn extract_number(func: &str, val: &Value, pos: usize) -> Result<f64, StdlibError> {
    match val {
        Value::Number(n) => Ok(*n),
        _ => Err(StdlibError::type_mismatch(
            func,
            pos,
            "number",
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
