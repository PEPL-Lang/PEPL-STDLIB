//! Integration tests for `pepl-stdlib` Phase 2: math module.

use pepl_stdlib::modules::math::MathModule;
use pepl_stdlib::{StdlibError, StdlibModule, Value};

// ── Helpers ───────────────────────────────────────────────────────────────────

fn math() -> MathModule {
    MathModule::new()
}

fn num(n: f64) -> Value {
    Value::Number(n)
}

fn call(func: &str, args: Vec<Value>) -> Result<Value, StdlibError> {
    math().call(func, args)
}

fn call_ok(func: &str, args: Vec<Value>) -> Value {
    call(func, args).expect(&format!("math.{func} should succeed"))
}

fn expect_num(func: &str, args: Vec<Value>) -> f64 {
    match call_ok(func, args) {
        Value::Number(n) => n,
        other => panic!("expected Number, got {other:?}"),
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// StdlibModule trait
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_module_name() {
    assert_eq!(math().name(), "math");
}

#[test]
fn test_has_function_known() {
    let m = math();
    for f in &[
        "abs", "min", "max", "floor", "ceil", "round", "round_to", "pow", "clamp", "sqrt", "PI",
        "E",
    ] {
        assert!(m.has_function(f), "math should have function {f}");
    }
}

#[test]
fn test_has_function_unknown() {
    assert!(!math().has_function("nonexistent"));
    assert!(!math().has_function("sin"));
    assert!(!math().has_function("cos"));
}

#[test]
fn test_unknown_function_error() {
    let err = call("nonexistent", vec![]).unwrap_err();
    assert!(matches!(err, StdlibError::UnknownFunction { .. }));
}

// ══════════════════════════════════════════════════════════════════════════════
// math.abs
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_abs_positive() {
    assert_eq!(expect_num("abs", vec![num(42.0)]), 42.0);
}

#[test]
fn test_abs_negative() {
    assert_eq!(expect_num("abs", vec![num(-42.0)]), 42.0);
}

#[test]
fn test_abs_zero() {
    assert_eq!(expect_num("abs", vec![num(0.0)]), 0.0);
}

#[test]
fn test_abs_negative_zero() {
    // abs(-0.0) should be 0.0
    let result = expect_num("abs", vec![num(-0.0)]);
    assert_eq!(result, 0.0);
    assert!(result.is_sign_positive());
}

#[test]
fn test_abs_decimal() {
    assert_eq!(expect_num("abs", vec![num(-3.14)]), 3.14);
}

#[test]
fn test_abs_wrong_type() {
    let err = call("abs", vec![Value::String("hello".into())]).unwrap_err();
    assert!(matches!(err, StdlibError::TypeMismatch { .. }));
}

#[test]
fn test_abs_wrong_arg_count() {
    let err = call("abs", vec![]).unwrap_err();
    assert!(matches!(err, StdlibError::WrongArgCount { .. }));

    let err = call("abs", vec![num(1.0), num(2.0)]).unwrap_err();
    assert!(matches!(err, StdlibError::WrongArgCount { .. }));
}

// ══════════════════════════════════════════════════════════════════════════════
// math.min
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_min_first_smaller() {
    assert_eq!(expect_num("min", vec![num(1.0), num(5.0)]), 1.0);
}

#[test]
fn test_min_second_smaller() {
    assert_eq!(expect_num("min", vec![num(10.0), num(3.0)]), 3.0);
}

#[test]
fn test_min_equal() {
    assert_eq!(expect_num("min", vec![num(7.0), num(7.0)]), 7.0);
}

#[test]
fn test_min_negative() {
    assert_eq!(expect_num("min", vec![num(-5.0), num(-3.0)]), -5.0);
}

#[test]
fn test_min_mixed_sign() {
    assert_eq!(expect_num("min", vec![num(-1.0), num(1.0)]), -1.0);
}

#[test]
fn test_min_wrong_type() {
    let err = call("min", vec![Value::Bool(true), num(1.0)]).unwrap_err();
    assert!(matches!(err, StdlibError::TypeMismatch { .. }));
}

// ══════════════════════════════════════════════════════════════════════════════
// math.max
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_max_first_larger() {
    assert_eq!(expect_num("max", vec![num(10.0), num(3.0)]), 10.0);
}

#[test]
fn test_max_second_larger() {
    assert_eq!(expect_num("max", vec![num(1.0), num(5.0)]), 5.0);
}

#[test]
fn test_max_equal() {
    assert_eq!(expect_num("max", vec![num(7.0), num(7.0)]), 7.0);
}

#[test]
fn test_max_negative() {
    assert_eq!(expect_num("max", vec![num(-5.0), num(-3.0)]), -3.0);
}

// ══════════════════════════════════════════════════════════════════════════════
// math.floor
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_floor_positive_fraction() {
    assert_eq!(expect_num("floor", vec![num(3.7)]), 3.0);
}

#[test]
fn test_floor_negative_fraction() {
    assert_eq!(expect_num("floor", vec![num(-3.2)]), -4.0);
}

#[test]
fn test_floor_integer() {
    assert_eq!(expect_num("floor", vec![num(5.0)]), 5.0);
}

#[test]
fn test_floor_zero() {
    assert_eq!(expect_num("floor", vec![num(0.0)]), 0.0);
}

#[test]
fn test_floor_half() {
    assert_eq!(expect_num("floor", vec![num(0.5)]), 0.0);
}

// ══════════════════════════════════════════════════════════════════════════════
// math.ceil
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_ceil_positive_fraction() {
    assert_eq!(expect_num("ceil", vec![num(3.2)]), 4.0);
}

#[test]
fn test_ceil_negative_fraction() {
    assert_eq!(expect_num("ceil", vec![num(-3.7)]), -3.0);
}

#[test]
fn test_ceil_integer() {
    assert_eq!(expect_num("ceil", vec![num(5.0)]), 5.0);
}

#[test]
fn test_ceil_zero() {
    assert_eq!(expect_num("ceil", vec![num(0.0)]), 0.0);
}

// ══════════════════════════════════════════════════════════════════════════════
// math.round — "0.5 rounds up" (towards +infinity)
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_round_down() {
    assert_eq!(expect_num("round", vec![num(3.2)]), 3.0);
}

#[test]
fn test_round_up() {
    assert_eq!(expect_num("round", vec![num(3.7)]), 4.0);
}

#[test]
fn test_round_half_up() {
    // Spec: 0.5 rounds up
    assert_eq!(expect_num("round", vec![num(0.5)]), 1.0);
    assert_eq!(expect_num("round", vec![num(1.5)]), 2.0);
    assert_eq!(expect_num("round", vec![num(2.5)]), 3.0);
}

#[test]
fn test_round_negative_half() {
    // "0.5 rounds up" means towards +infinity:
    // -0.5 → 0 (rounded up towards +infinity)
    // -1.5 → -1 (rounded up towards +infinity)
    assert_eq!(expect_num("round", vec![num(-0.5)]), 0.0);
    assert_eq!(expect_num("round", vec![num(-1.5)]), -1.0);
}

#[test]
fn test_round_negative() {
    assert_eq!(expect_num("round", vec![num(-3.2)]), -3.0);
    assert_eq!(expect_num("round", vec![num(-3.7)]), -4.0);
}

#[test]
fn test_round_integer() {
    assert_eq!(expect_num("round", vec![num(5.0)]), 5.0);
    assert_eq!(expect_num("round", vec![num(-5.0)]), -5.0);
}

#[test]
fn test_round_zero() {
    assert_eq!(expect_num("round", vec![num(0.0)]), 0.0);
}

// ══════════════════════════════════════════════════════════════════════════════
// math.round_to
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_round_to_two_decimals() {
    assert_eq!(expect_num("round_to", vec![num(3.14159), num(2.0)]), 3.14);
}

#[test]
fn test_round_to_zero_decimals() {
    assert_eq!(expect_num("round_to", vec![num(3.7), num(0.0)]), 4.0);
}

#[test]
fn test_round_to_one_decimal() {
    assert_eq!(expect_num("round_to", vec![num(2.75), num(1.0)]), 2.8);
}

#[test]
fn test_round_to_half_up() {
    // 2.55 rounded to 1 decimal: 2.55 * 10 = 25.5, +0.5 = 26, floor = 26, /10 = 2.6
    assert_eq!(expect_num("round_to", vec![num(2.55), num(1.0)]), 2.6);
}

#[test]
fn test_round_to_negative_decimals_error() {
    let err = call("round_to", vec![num(3.14), num(-1.0)]).unwrap_err();
    assert!(matches!(err, StdlibError::RuntimeError(_)));
}

#[test]
fn test_round_to_fractional_decimals_error() {
    let err = call("round_to", vec![num(3.14), num(1.5)]).unwrap_err();
    assert!(matches!(err, StdlibError::RuntimeError(_)));
}

// ══════════════════════════════════════════════════════════════════════════════
// math.pow
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_pow_basic() {
    assert_eq!(expect_num("pow", vec![num(2.0), num(3.0)]), 8.0);
}

#[test]
fn test_pow_square() {
    assert_eq!(expect_num("pow", vec![num(5.0), num(2.0)]), 25.0);
}

#[test]
fn test_pow_zero_exp() {
    assert_eq!(expect_num("pow", vec![num(100.0), num(0.0)]), 1.0);
}

#[test]
fn test_pow_one_exp() {
    assert_eq!(expect_num("pow", vec![num(42.0), num(1.0)]), 42.0);
}

#[test]
fn test_pow_negative_exp() {
    assert_eq!(expect_num("pow", vec![num(2.0), num(-1.0)]), 0.5);
}

#[test]
fn test_pow_fractional_exp() {
    // 4^0.5 = 2.0 (square root)
    assert_eq!(expect_num("pow", vec![num(4.0), num(0.5)]), 2.0);
}

#[test]
fn test_pow_nan_trap() {
    // (-1)^0.5 would produce NaN → should trap
    let err = call("pow", vec![num(-1.0), num(0.5)]).unwrap_err();
    assert!(matches!(err, StdlibError::RuntimeError(_)));
}

#[test]
fn test_pow_infinity_trap() {
    // Very large exponent → infinity → should trap
    let err = call("pow", vec![num(10.0), num(1000.0)]).unwrap_err();
    assert!(matches!(err, StdlibError::RuntimeError(_)));
}

// ══════════════════════════════════════════════════════════════════════════════
// math.clamp
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_clamp_within_range() {
    assert_eq!(expect_num("clamp", vec![num(5.0), num(0.0), num(10.0)]), 5.0);
}

#[test]
fn test_clamp_below_min() {
    assert_eq!(expect_num("clamp", vec![num(-5.0), num(0.0), num(10.0)]), 0.0);
}

#[test]
fn test_clamp_above_max() {
    assert_eq!(expect_num("clamp", vec![num(15.0), num(0.0), num(10.0)]), 10.0);
}

#[test]
fn test_clamp_at_min() {
    assert_eq!(expect_num("clamp", vec![num(0.0), num(0.0), num(10.0)]), 0.0);
}

#[test]
fn test_clamp_at_max() {
    assert_eq!(expect_num("clamp", vec![num(10.0), num(0.0), num(10.0)]), 10.0);
}

#[test]
fn test_clamp_min_equals_max() {
    assert_eq!(expect_num("clamp", vec![num(5.0), num(3.0), num(3.0)]), 3.0);
}

#[test]
fn test_clamp_negative_range() {
    assert_eq!(
        expect_num("clamp", vec![num(0.0), num(-10.0), num(-5.0)]),
        -5.0
    );
}

#[test]
fn test_clamp_min_greater_than_max_error() {
    let err = call("clamp", vec![num(5.0), num(10.0), num(0.0)]).unwrap_err();
    assert!(matches!(err, StdlibError::RuntimeError(_)));
}

#[test]
fn test_clamp_wrong_arg_count() {
    let err = call("clamp", vec![num(1.0), num(2.0)]).unwrap_err();
    assert!(matches!(err, StdlibError::WrongArgCount { .. }));
}

#[test]
fn test_clamp_wrong_type() {
    let err = call(
        "clamp",
        vec![Value::String("x".into()), num(0.0), num(10.0)],
    )
    .unwrap_err();
    assert!(matches!(err, StdlibError::TypeMismatch { .. }));
}

// ══════════════════════════════════════════════════════════════════════════════
// math.sqrt
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_sqrt_perfect_square() {
    assert_eq!(expect_num("sqrt", vec![num(4.0)]), 2.0);
    assert_eq!(expect_num("sqrt", vec![num(9.0)]), 3.0);
    assert_eq!(expect_num("sqrt", vec![num(16.0)]), 4.0);
    assert_eq!(expect_num("sqrt", vec![num(100.0)]), 10.0);
}

#[test]
fn test_sqrt_non_perfect() {
    let result = expect_num("sqrt", vec![num(2.0)]);
    assert!((result - std::f64::consts::SQRT_2).abs() < 1e-10);
}

#[test]
fn test_sqrt_zero() {
    assert_eq!(expect_num("sqrt", vec![num(0.0)]), 0.0);
}

#[test]
fn test_sqrt_one() {
    assert_eq!(expect_num("sqrt", vec![num(1.0)]), 1.0);
}

#[test]
fn test_sqrt_negative_trap() {
    let err = call("sqrt", vec![num(-1.0)]).unwrap_err();
    assert!(matches!(err, StdlibError::RuntimeError(_)));
    let msg = err.to_string();
    assert!(msg.contains("negative"), "error should mention negative: {msg}");
}

#[test]
fn test_sqrt_small_negative_trap() {
    // Even very small negatives should trap
    let err = call("sqrt", vec![num(-0.001)]).unwrap_err();
    assert!(matches!(err, StdlibError::RuntimeError(_)));
}

// ══════════════════════════════════════════════════════════════════════════════
// math.PI and math.E constants
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_pi_value() {
    let pi = expect_num("PI", vec![]);
    assert!((pi - std::f64::consts::PI).abs() < 1e-15);
}

#[test]
fn test_e_value() {
    let e = expect_num("E", vec![]);
    assert!((e - std::f64::consts::E).abs() < 1e-15);
}

#[test]
fn test_pi_no_args() {
    let err = call("PI", vec![num(1.0)]).unwrap_err();
    assert!(matches!(err, StdlibError::WrongArgCount { .. }));
}

#[test]
fn test_e_no_args() {
    let err = call("E", vec![num(1.0)]).unwrap_err();
    assert!(matches!(err, StdlibError::WrongArgCount { .. }));
}

// ══════════════════════════════════════════════════════════════════════════════
// NaN prevention — comprehensive
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_nan_prevention_pow_negative_fractional() {
    // (-2)^0.3 → NaN
    let err = call("pow", vec![num(-2.0), num(0.3)]).unwrap_err();
    assert!(matches!(err, StdlibError::RuntimeError(_)));
}

#[test]
fn test_nan_prevention_sqrt_negative() {
    let err = call("sqrt", vec![num(-4.0)]).unwrap_err();
    assert!(matches!(err, StdlibError::RuntimeError(_)));
}

// ══════════════════════════════════════════════════════════════════════════════
// Edge cases
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_very_large_numbers() {
    assert_eq!(expect_num("abs", vec![num(1e308)]), 1e308);
    assert_eq!(expect_num("abs", vec![num(-1e308)]), 1e308);
}

#[test]
fn test_very_small_numbers() {
    let tiny = 5e-324; // smallest positive subnormal
    assert_eq!(expect_num("abs", vec![num(tiny)]), tiny);
}

#[test]
fn test_floor_large_number() {
    assert_eq!(expect_num("floor", vec![num(1e15 + 0.5)]), 1e15);
}

#[test]
fn test_ceil_large_number() {
    assert_eq!(expect_num("ceil", vec![num(1e15 - 0.5)]), 1e15);
}

#[test]
fn test_pow_zero_to_zero() {
    // 0^0 = 1 (standard math convention)
    assert_eq!(expect_num("pow", vec![num(0.0), num(0.0)]), 1.0);
}

#[test]
fn test_pow_zero_base_positive_exp() {
    assert_eq!(expect_num("pow", vec![num(0.0), num(5.0)]), 0.0);
}

#[test]
fn test_sqrt_large() {
    assert_eq!(expect_num("sqrt", vec![num(1e16)]), 1e8);
}

#[test]
fn test_min_with_zero() {
    assert_eq!(expect_num("min", vec![num(0.0), num(0.0)]), 0.0);
}

#[test]
fn test_max_with_zero() {
    assert_eq!(expect_num("max", vec![num(0.0), num(0.0)]), 0.0);
}

#[test]
fn test_round_large_half() {
    // 1000000.5 → 1000001 (rounds up)
    assert_eq!(expect_num("round", vec![num(1_000_000.5)]), 1_000_001.0);
}

#[test]
fn test_clamp_decimal_range() {
    assert_eq!(
        expect_num("clamp", vec![num(0.75), num(0.0), num(1.0)]),
        0.75
    );
}

// ══════════════════════════════════════════════════════════════════════════════
// Determinism proof — 100-iteration test
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_determinism_100_iterations() {
    let m = math();

    // Capture reference results
    let ref_abs = m.call("abs", vec![num(-42.5)]).unwrap();
    let ref_min = m.call("min", vec![num(3.0), num(7.0)]).unwrap();
    let ref_max = m.call("max", vec![num(3.0), num(7.0)]).unwrap();
    let ref_floor = m.call("floor", vec![num(3.7)]).unwrap();
    let ref_ceil = m.call("ceil", vec![num(3.2)]).unwrap();
    let ref_round = m.call("round", vec![num(2.5)]).unwrap();
    let ref_round_to = m.call("round_to", vec![num(3.14159), num(2.0)]).unwrap();
    let ref_pow = m.call("pow", vec![num(2.0), num(10.0)]).unwrap();
    let ref_clamp = m.call("clamp", vec![num(15.0), num(0.0), num(10.0)]).unwrap();
    let ref_sqrt = m.call("sqrt", vec![num(144.0)]).unwrap();
    let ref_pi = m.call("PI", vec![]).unwrap();
    let ref_e = m.call("E", vec![]).unwrap();

    for i in 0..100 {
        assert_eq!(m.call("abs", vec![num(-42.5)]).unwrap(), ref_abs, "abs iter {i}");
        assert_eq!(m.call("min", vec![num(3.0), num(7.0)]).unwrap(), ref_min, "min iter {i}");
        assert_eq!(m.call("max", vec![num(3.0), num(7.0)]).unwrap(), ref_max, "max iter {i}");
        assert_eq!(m.call("floor", vec![num(3.7)]).unwrap(), ref_floor, "floor iter {i}");
        assert_eq!(m.call("ceil", vec![num(3.2)]).unwrap(), ref_ceil, "ceil iter {i}");
        assert_eq!(m.call("round", vec![num(2.5)]).unwrap(), ref_round, "round iter {i}");
        assert_eq!(
            m.call("round_to", vec![num(3.14159), num(2.0)]).unwrap(),
            ref_round_to,
            "round_to iter {i}"
        );
        assert_eq!(m.call("pow", vec![num(2.0), num(10.0)]).unwrap(), ref_pow, "pow iter {i}");
        assert_eq!(
            m.call("clamp", vec![num(15.0), num(0.0), num(10.0)]).unwrap(),
            ref_clamp,
            "clamp iter {i}"
        );
        assert_eq!(m.call("sqrt", vec![num(144.0)]).unwrap(), ref_sqrt, "sqrt iter {i}");
        assert_eq!(m.call("PI", vec![]).unwrap(), ref_pi, "PI iter {i}");
        assert_eq!(m.call("E", vec![]).unwrap(), ref_e, "E iter {i}");
    }
}
