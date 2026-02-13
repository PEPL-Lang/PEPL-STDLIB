//! The `math` module — 10 functions + 2 constants.
//!
//! | Function     | Signature                                  | Description                  |
//! |--------------|--------------------------------------------|------------------------------|
//! | `math.abs`   | `(a: number) -> number`                    | Absolute value               |
//! | `math.min`   | `(a: number, b: number) -> number`         | Smaller of two values        |
//! | `math.max`   | `(a: number, b: number) -> number`         | Larger of two values         |
//! | `math.floor` | `(a: number) -> number`                    | Round down                   |
//! | `math.ceil`  | `(a: number) -> number`                    | Round up                     |
//! | `math.round` | `(a: number) -> number`                    | Round (0.5 rounds up)        |
//! | `math.round_to` | `(a: number, decimals: number) -> number` | Round to N decimal places |
//! | `math.pow`   | `(base: number, exp: number) -> number`    | Exponentiation               |
//! | `math.clamp` | `(value: number, min: number, max: number) -> number` | Clamp to range |
//! | `math.sqrt`  | `(a: number) -> number`                    | Square root (trap on negative) |
//! | `math.PI`    | constant `number`                          | 3.14159265358979…            |
//! | `math.E`     | constant `number`                          | 2.71828182845904…            |

use crate::error::StdlibError;
use crate::module::StdlibModule;
use crate::value::Value;

/// The `math` stdlib module.
pub struct MathModule;

impl MathModule {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MathModule {
    fn default() -> Self {
        Self::new()
    }
}

impl StdlibModule for MathModule {
    fn name(&self) -> &'static str {
        "math"
    }

    fn has_function(&self, function: &str) -> bool {
        matches!(
            function,
            "abs"
                | "min"
                | "max"
                | "floor"
                | "ceil"
                | "round"
                | "round_to"
                | "pow"
                | "clamp"
                | "sqrt"
                | "PI"
                | "E"
        )
    }

    fn call(&self, function: &str, args: Vec<Value>) -> Result<Value, StdlibError> {
        match function {
            "abs" => self.abs(args),
            "min" => self.min(args),
            "max" => self.max(args),
            "floor" => self.floor(args),
            "ceil" => self.ceil(args),
            "round" => self.round(args),
            "round_to" => self.round_to(args),
            "pow" => self.pow(args),
            "clamp" => self.clamp(args),
            "sqrt" => self.sqrt(args),
            // Constants are dispatched as zero-arg "calls"
            "PI" => self.pi(args),
            "E" => self.e(args),
            _ => Err(StdlibError::unknown_function("math", function)),
        }
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Extract a single `Number` argument for a unary function.
fn expect_one_number(fn_name: &str, args: &[Value]) -> Result<f64, StdlibError> {
    if args.len() != 1 {
        return Err(StdlibError::wrong_args(fn_name, 1, args.len()));
    }
    match &args[0] {
        Value::Number(n) => Ok(*n),
        other => Err(StdlibError::type_mismatch(
            fn_name,
            1,
            "number",
            other.type_name(),
        )),
    }
}

/// Extract two `Number` arguments for a binary function.
fn expect_two_numbers(fn_name: &str, args: &[Value]) -> Result<(f64, f64), StdlibError> {
    if args.len() != 2 {
        return Err(StdlibError::wrong_args(fn_name, 2, args.len()));
    }
    let a = match &args[0] {
        Value::Number(n) => *n,
        other => {
            return Err(StdlibError::type_mismatch(
                fn_name,
                1,
                "number",
                other.type_name(),
            ));
        }
    };
    let b = match &args[1] {
        Value::Number(n) => *n,
        other => {
            return Err(StdlibError::type_mismatch(
                fn_name,
                2,
                "number",
                other.type_name(),
            ));
        }
    };
    Ok((a, b))
}

/// Guard against NaN results. Per PEPL spec: operations that would produce NaN
/// trap instead.
fn nan_guard(fn_name: &str, result: f64) -> Result<Value, StdlibError> {
    if result.is_nan() {
        Err(StdlibError::RuntimeError(format!(
            "{fn_name}: operation would produce NaN"
        )))
    } else if result.is_infinite() {
        Err(StdlibError::RuntimeError(format!(
            "{fn_name}: operation would produce infinity"
        )))
    } else {
        Ok(Value::Number(result))
    }
}

// ── Function implementations ──────────────────────────────────────────────────

impl MathModule {
    /// `math.abs(a: number) -> number`
    ///
    /// Absolute value. Always finite for finite input.
    fn abs(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        let a = expect_one_number("math.abs", &args)?;
        Ok(Value::Number(a.abs()))
    }

    /// `math.min(a: number, b: number) -> number`
    ///
    /// Returns the smaller of two values.
    fn min(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        let (a, b) = expect_two_numbers("math.min", &args)?;
        // Use f64::min which handles -0.0 vs 0.0 correctly
        Ok(Value::Number(a.min(b)))
    }

    /// `math.max(a: number, b: number) -> number`
    ///
    /// Returns the larger of two values.
    fn max(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        let (a, b) = expect_two_numbers("math.max", &args)?;
        Ok(Value::Number(a.max(b)))
    }

    /// `math.floor(a: number) -> number`
    ///
    /// Round down to nearest integer.
    fn floor(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        let a = expect_one_number("math.floor", &args)?;
        Ok(Value::Number(a.floor()))
    }

    /// `math.ceil(a: number) -> number`
    ///
    /// Round up to nearest integer.
    fn ceil(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        let a = expect_one_number("math.ceil", &args)?;
        Ok(Value::Number(a.ceil()))
    }

    /// `math.round(a: number) -> number`
    ///
    /// Round to nearest integer. Per PEPL spec: 0.5 rounds **up** (away from
    /// zero for positive, towards zero for negative). This matches the
    /// "round half up" convention.
    fn round(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        let a = expect_one_number("math.round", &args)?;
        // Rust's f64::round() uses "round half away from zero" (bankers' rounding
        // is f64::round_ties_even). The PEPL spec says "0.5 rounds up", meaning:
        //   0.5 → 1, 1.5 → 2, 2.5 → 3, -0.5 → 0, -1.5 → -1
        // This is "round half up" (towards +infinity).
        Ok(Value::Number((a + 0.5).floor()))
    }

    /// `math.round_to(a: number, decimals: number) -> number`
    ///
    /// Round to N decimal places using the same "0.5 rounds up" rule.
    fn round_to(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        let (a, decimals) = expect_two_numbers("math.round_to", &args)?;

        // Validate decimals is a non-negative integer
        if decimals < 0.0 || decimals.fract() != 0.0 {
            return Err(StdlibError::RuntimeError(
                "math.round_to: decimals must be a non-negative integer".to_string(),
            ));
        }

        let factor = 10_f64.powi(decimals as i32);
        let scaled = a * factor;
        let rounded = (scaled + 0.5).floor();
        let result = rounded / factor;

        nan_guard("math.round_to", result)
    }

    /// `math.pow(base: number, exp: number) -> number`
    ///
    /// Exponentiation. Traps if result would be NaN or infinity.
    fn pow(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        let (base, exp) = expect_two_numbers("math.pow", &args)?;
        let result = base.powf(exp);
        nan_guard("math.pow", result)
    }

    /// `math.clamp(value: number, min: number, max: number) -> number`
    ///
    /// Clamp value to [min, max] range.
    fn clamp(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 3 {
            return Err(StdlibError::wrong_args("math.clamp", 3, args.len()));
        }
        let value = match &args[0] {
            Value::Number(n) => *n,
            other => {
                return Err(StdlibError::type_mismatch(
                    "math.clamp",
                    1,
                    "number",
                    other.type_name(),
                ));
            }
        };
        let min = match &args[1] {
            Value::Number(n) => *n,
            other => {
                return Err(StdlibError::type_mismatch(
                    "math.clamp",
                    2,
                    "number",
                    other.type_name(),
                ));
            }
        };
        let max = match &args[2] {
            Value::Number(n) => *n,
            other => {
                return Err(StdlibError::type_mismatch(
                    "math.clamp",
                    3,
                    "number",
                    other.type_name(),
                ));
            }
        };

        if min > max {
            return Err(StdlibError::RuntimeError(
                "math.clamp: min must be <= max".to_string(),
            ));
        }

        Ok(Value::Number(value.clamp(min, max)))
    }

    /// `math.sqrt(a: number) -> number`
    ///
    /// Square root. Traps on negative input (NaN prevention).
    fn sqrt(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        let a = expect_one_number("math.sqrt", &args)?;
        if a < 0.0 {
            return Err(StdlibError::RuntimeError(
                "math.sqrt: cannot take square root of negative number".to_string(),
            ));
        }
        Ok(Value::Number(a.sqrt()))
    }

    /// `math.PI` constant — 3.14159265358979…
    fn pi(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if !args.is_empty() {
            return Err(StdlibError::wrong_args("math.PI", 0, args.len()));
        }
        Ok(Value::Number(std::f64::consts::PI))
    }

    /// `math.E` constant — 2.71828182845904…
    fn e(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if !args.is_empty() {
            return Err(StdlibError::wrong_args("math.E", 0, args.len()));
        }
        Ok(Value::Number(std::f64::consts::E))
    }
}
