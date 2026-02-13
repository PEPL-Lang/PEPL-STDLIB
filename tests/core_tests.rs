//! Integration tests for `pepl-stdlib` Phase 1: scaffolding + core module.

use pepl_stdlib::modules::core::CoreModule;
use pepl_stdlib::{StdlibError, StdlibModule, Value};
use std::collections::BTreeMap;

// ── Helpers ───────────────────────────────────────────────────────────────────

fn core() -> CoreModule {
    CoreModule::new()
}

// ══════════════════════════════════════════════════════════════════════════════
// Value type tests
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_value_type_names() {
    assert_eq!(Value::Number(42.0).type_name(), "number");
    assert_eq!(Value::String("hello".into()).type_name(), "string");
    assert_eq!(Value::Bool(true).type_name(), "bool");
    assert_eq!(Value::Nil.type_name(), "nil");
    assert_eq!(Value::List(vec![]).type_name(), "list");
    assert_eq!(Value::record(BTreeMap::new()).type_name(), "record");
    assert_eq!(
        Value::Color {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0
        }
        .type_name(),
        "color"
    );
    assert_eq!(Value::Number(1.0).ok().type_name(), "result");
    // Named record returns declared type name
    assert_eq!(
        Value::named_record("Todo", BTreeMap::new()).type_name(),
        "Todo"
    );
    // Sum variant returns declaring type name
    assert_eq!(
        Value::unit_variant("Status", "Active").type_name(),
        "Status"
    );
}

#[test]
fn test_value_display_number_integer() {
    assert_eq!(format!("{}", Value::Number(42.0)), "42");
    assert_eq!(format!("{}", Value::Number(0.0)), "0");
    assert_eq!(format!("{}", Value::Number(-7.0)), "-7");
}

#[test]
fn test_value_display_number_decimal() {
    assert_eq!(format!("{}", Value::Number(3.14)), "3.14");
    assert_eq!(format!("{}", Value::Number(-0.5)), "-0.5");
}

#[test]
fn test_value_display_string() {
    assert_eq!(format!("{}", Value::String("hello".into())), "hello");
    assert_eq!(format!("{}", Value::String("".into())), "");
}

#[test]
fn test_value_display_bool() {
    assert_eq!(format!("{}", Value::Bool(true)), "true");
    assert_eq!(format!("{}", Value::Bool(false)), "false");
}

#[test]
fn test_value_display_nil() {
    assert_eq!(format!("{}", Value::Nil), "nil");
}

#[test]
fn test_value_display_list() {
    let list = Value::List(vec![
        Value::Number(1.0),
        Value::String("two".into()),
        Value::Bool(true),
    ]);
    assert_eq!(format!("{list}"), "[1, \"two\", true]");
}

#[test]
fn test_value_display_empty_list() {
    assert_eq!(format!("{}", Value::List(vec![])), "[]");
}

#[test]
fn test_value_display_record() {
    let mut fields = BTreeMap::new();
    fields.insert("name".to_string(), Value::String("Alice".into()));
    fields.insert("age".to_string(), Value::Number(30.0));
    let record = Value::record(fields);
    // BTreeMap iterates in alphabetical order
    assert_eq!(format!("{record}"), "{age: 30, name: \"Alice\"}");
}

#[test]
fn test_value_display_named_record() {
    let mut fields = BTreeMap::new();
    fields.insert("x".to_string(), Value::Number(1.0));
    let record = Value::named_record("Point", fields);
    assert_eq!(format!("{record}"), "Point{x: 1}");
}

#[test]
fn test_value_display_color() {
    let color = Value::Color {
        r: 1.0,
        g: 0.5,
        b: 0.0,
        a: 1.0,
    };
    assert_eq!(format!("{color}"), "color(1, 0.5, 0, 1)");
}

#[test]
fn test_value_display_result() {
    assert_eq!(format!("{}", Value::Number(42.0).ok()), "Ok(42)");
    assert_eq!(
        format!("{}", Value::String("fail".into()).err()),
        "Err(fail)"
    );
}

// ── Equality tests ────────────────────────────────────────────────────────────

#[test]
fn test_value_equality_numbers() {
    assert_eq!(Value::Number(1.0), Value::Number(1.0));
    assert_ne!(Value::Number(1.0), Value::Number(2.0));
}

#[test]
fn test_value_equality_nan() {
    // NaN != NaN per IEEE 754 and PEPL spec
    assert_ne!(Value::Number(f64::NAN), Value::Number(f64::NAN));
}

#[test]
fn test_value_equality_strings() {
    assert_eq!(Value::String("hello".into()), Value::String("hello".into()));
    assert_ne!(Value::String("hello".into()), Value::String("world".into()));
}

#[test]
fn test_value_equality_cross_type() {
    // Different types are never equal
    assert_ne!(Value::Number(0.0), Value::Bool(false));
    assert_ne!(Value::Number(0.0), Value::Nil);
    assert_ne!(Value::String("".into()), Value::Nil);
    assert_ne!(Value::Bool(false), Value::Nil);
}

#[test]
fn test_value_equality_lists() {
    let a = Value::List(vec![Value::Number(1.0), Value::Number(2.0)]);
    let b = Value::List(vec![Value::Number(1.0), Value::Number(2.0)]);
    let c = Value::List(vec![Value::Number(1.0), Value::Number(3.0)]);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn test_value_equality_records() {
    let mut r1 = BTreeMap::new();
    r1.insert("x".to_string(), Value::Number(1.0));
    let mut r2 = BTreeMap::new();
    r2.insert("x".to_string(), Value::Number(1.0));
    let mut r3 = BTreeMap::new();
    r3.insert("x".to_string(), Value::Number(2.0));

    assert_eq!(Value::record(r1.clone()), Value::record(r2));
    assert_ne!(Value::record(r1), Value::record(r3));
}

#[test]
fn test_value_equality_records_ignore_type_name() {
    // Structural equality ignores type_name — type checker ensures compatibility
    let mut fields = BTreeMap::new();
    fields.insert("x".to_string(), Value::Number(1.0));
    let a = Value::named_record("Foo", fields.clone());
    let b = Value::named_record("Bar", fields.clone());
    let c = Value::record(fields);
    assert_eq!(a, b); // same fields, different name → equal
    assert_eq!(a, c); // named vs anonymous → equal
}

#[test]
fn test_value_equality_colors() {
    let c1 = Value::Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    let c2 = Value::Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    let c3 = Value::Color {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    assert_eq!(c1, c2);
    assert_ne!(c1, c3);
}

#[test]
fn test_value_equality_results() {
    assert_eq!(Value::Number(1.0).ok(), Value::Number(1.0).ok());
    assert_ne!(Value::Number(1.0).ok(), Value::Number(1.0).err());
    assert_ne!(Value::Number(1.0).ok(), Value::Number(2.0).ok());
}

// ── Truthiness tests ──────────────────────────────────────────────────────────

#[test]
fn test_value_truthy() {
    assert!(Value::Bool(true).is_truthy());
    assert!(Value::Number(1.0).is_truthy());
    assert!(Value::Number(-1.0).is_truthy());
    assert!(Value::String("hello".into()).is_truthy());
    assert!(Value::List(vec![]).is_truthy());
    assert!(Value::record(BTreeMap::new()).is_truthy());
    assert!(Value::unit_variant("Status", "Active").is_truthy());
}

#[test]
fn test_value_falsy() {
    assert!(!Value::Bool(false).is_truthy());
    assert!(!Value::Nil.is_truthy());
    assert!(!Value::Number(0.0).is_truthy());
    assert!(!Value::String("".into()).is_truthy());
}

// ── Accessor tests ────────────────────────────────────────────────────────────

#[test]
fn test_value_as_number() {
    assert_eq!(Value::Number(42.0).as_number(), Some(42.0));
    assert_eq!(Value::String("x".into()).as_number(), None);
}

#[test]
fn test_value_as_str() {
    assert_eq!(Value::String("hi".into()).as_str(), Some("hi"));
    assert_eq!(Value::Number(1.0).as_str(), None);
}

#[test]
fn test_value_as_bool() {
    assert_eq!(Value::Bool(true).as_bool(), Some(true));
    assert_eq!(Value::Nil.as_bool(), None);
}

#[test]
fn test_value_as_list() {
    let v = Value::List(vec![Value::Number(1.0)]);
    assert_eq!(v.as_list().unwrap().len(), 1);
    assert_eq!(Value::Nil.as_list(), None);
}

#[test]
fn test_value_as_record() {
    let v = Value::record(BTreeMap::new());
    assert!(v.as_record().unwrap().is_empty());
    assert_eq!(Value::Nil.as_record(), None);
}

// ── From impls ────────────────────────────────────────────────────────────────

#[test]
fn test_value_from_f64() {
    let v: Value = 3.14.into();
    assert_eq!(v, Value::Number(3.14));
}

#[test]
fn test_value_from_i64() {
    let v: Value = 42i64.into();
    assert_eq!(v, Value::Number(42.0));
}

#[test]
fn test_value_from_str() {
    let v: Value = "hello".into();
    assert_eq!(v, Value::String("hello".into()));
}

#[test]
fn test_value_from_string() {
    let v: Value = String::from("world").into();
    assert_eq!(v, Value::String("world".into()));
}

#[test]
fn test_value_from_bool() {
    let v: Value = true.into();
    assert_eq!(v, Value::Bool(true));
}

// ══════════════════════════════════════════════════════════════════════════════
// core.log tests
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_core_log_returns_nil() {
    let result = core().call("log", vec![Value::Number(42.0)]).unwrap();
    assert_eq!(result, Value::Nil);
}

#[test]
fn test_core_log_accepts_any_type() {
    let c = core();
    assert_eq!(
        c.call("log", vec![Value::String("hi".into())]).unwrap(),
        Value::Nil
    );
    assert_eq!(c.call("log", vec![Value::Bool(true)]).unwrap(), Value::Nil);
    assert_eq!(c.call("log", vec![Value::Nil]).unwrap(), Value::Nil);
    assert_eq!(
        c.call("log", vec![Value::List(vec![])]).unwrap(),
        Value::Nil
    );
}

#[test]
fn test_core_log_wrong_arg_count() {
    let err = core().call("log", vec![]).unwrap_err();
    assert!(matches!(err, StdlibError::WrongArgCount { .. }));

    let err = core()
        .call("log", vec![Value::Nil, Value::Nil])
        .unwrap_err();
    assert!(matches!(err, StdlibError::WrongArgCount { .. }));
}

// ══════════════════════════════════════════════════════════════════════════════
// core.assert tests
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_core_assert_true() {
    let result = core().call("assert", vec![Value::Bool(true)]).unwrap();
    assert_eq!(result, Value::Nil);
}

#[test]
fn test_core_assert_true_with_message() {
    let result = core()
        .call(
            "assert",
            vec![Value::Bool(true), Value::String("ok".into())],
        )
        .unwrap();
    assert_eq!(result, Value::Nil);
}

#[test]
fn test_core_assert_false_no_message() {
    let err = core().call("assert", vec![Value::Bool(false)]).unwrap_err();
    match err {
        StdlibError::AssertionFailed { message } => {
            assert_eq!(message, "assertion failed");
        }
        other => panic!("expected AssertionFailed, got {other:?}"),
    }
}

#[test]
fn test_core_assert_false_with_message() {
    let err = core()
        .call(
            "assert",
            vec![
                Value::Bool(false),
                Value::String("count must be positive".into()),
            ],
        )
        .unwrap_err();
    match err {
        StdlibError::AssertionFailed { message } => {
            assert_eq!(message, "count must be positive");
        }
        other => panic!("expected AssertionFailed, got {other:?}"),
    }
}

#[test]
fn test_core_assert_type_mismatch_condition() {
    let err = core().call("assert", vec![Value::Number(1.0)]).unwrap_err();
    assert!(matches!(err, StdlibError::TypeMismatch { .. }));
}

#[test]
fn test_core_assert_type_mismatch_message() {
    let err = core()
        .call("assert", vec![Value::Bool(true), Value::Number(1.0)])
        .unwrap_err();
    assert!(matches!(err, StdlibError::TypeMismatch { .. }));
}

#[test]
fn test_core_assert_wrong_arg_count() {
    let err = core().call("assert", vec![]).unwrap_err();
    assert!(matches!(err, StdlibError::WrongArgCount { .. }));

    let err = core()
        .call(
            "assert",
            vec![Value::Bool(true), Value::String("a".into()), Value::Nil],
        )
        .unwrap_err();
    assert!(matches!(err, StdlibError::WrongArgCount { .. }));
}

// ══════════════════════════════════════════════════════════════════════════════
// core.type_of tests
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_core_type_of_number() {
    let result = core().call("type_of", vec![Value::Number(3.14)]).unwrap();
    assert_eq!(result, Value::String("number".into()));
}

#[test]
fn test_core_type_of_string() {
    let result = core()
        .call("type_of", vec![Value::String("hi".into())])
        .unwrap();
    assert_eq!(result, Value::String("string".into()));
}

#[test]
fn test_core_type_of_bool() {
    let result = core().call("type_of", vec![Value::Bool(false)]).unwrap();
    assert_eq!(result, Value::String("bool".into()));
}

#[test]
fn test_core_type_of_nil() {
    let result = core().call("type_of", vec![Value::Nil]).unwrap();
    assert_eq!(result, Value::String("nil".into()));
}

#[test]
fn test_core_type_of_list() {
    let result = core().call("type_of", vec![Value::List(vec![])]).unwrap();
    assert_eq!(result, Value::String("list".into()));
}

#[test]
fn test_core_type_of_record() {
    let result = core()
        .call("type_of", vec![Value::record(BTreeMap::new())])
        .unwrap();
    assert_eq!(result, Value::String("record".into()));
}

#[test]
fn test_core_type_of_named_record() {
    let result = core()
        .call(
            "type_of",
            vec![Value::named_record("Todo", BTreeMap::new())],
        )
        .unwrap();
    assert_eq!(result, Value::String("Todo".into()));
}

#[test]
fn test_core_type_of_sum_variant() {
    let result = core()
        .call("type_of", vec![Value::unit_variant("Status", "Active")])
        .unwrap();
    assert_eq!(result, Value::String("Status".into()));
}

#[test]
fn test_core_type_of_wrong_arg_count() {
    let err = core().call("type_of", vec![]).unwrap_err();
    assert!(matches!(err, StdlibError::WrongArgCount { .. }));
}

// ══════════════════════════════════════════════════════════════════════════════
// core.capability tests
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_core_capability_returns_false() {
    let result = core()
        .call("capability", vec![Value::String("http".into())])
        .unwrap();
    assert_eq!(result, Value::Bool(false));
}

#[test]
fn test_core_capability_any_name_returns_false() {
    let c = core();
    for name in &[
        "http",
        "storage",
        "location",
        "notifications",
        "nonexistent",
    ] {
        let result = c
            .call("capability", vec![Value::String((*name).into())])
            .unwrap();
        assert_eq!(
            result,
            Value::Bool(false),
            "capability({name}) should be false"
        );
    }
}

#[test]
fn test_core_capability_type_mismatch() {
    let err = core()
        .call("capability", vec![Value::Number(1.0)])
        .unwrap_err();
    assert!(matches!(err, StdlibError::TypeMismatch { .. }));
}

#[test]
fn test_core_capability_wrong_arg_count() {
    let err = core().call("capability", vec![]).unwrap_err();
    assert!(matches!(err, StdlibError::WrongArgCount { .. }));
}

// ══════════════════════════════════════════════════════════════════════════════
// Module trait tests
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_core_module_name() {
    assert_eq!(core().name(), "core");
}

#[test]
fn test_core_has_function() {
    let c = core();
    assert!(c.has_function("log"));
    assert!(c.has_function("assert"));
    assert!(c.has_function("type_of"));
    assert!(c.has_function("capability"));
    assert!(!c.has_function("nonexistent"));
    assert!(!c.has_function(""));
}

#[test]
fn test_core_unknown_function() {
    let err = core().call("nonexistent", vec![]).unwrap_err();
    assert!(matches!(err, StdlibError::UnknownFunction { .. }));
}

// ══════════════════════════════════════════════════════════════════════════════
// Error display tests
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_error_display_wrong_args() {
    let err = StdlibError::wrong_args("core.log", 1, 0);
    assert_eq!(format!("{err}"), "core.log: expected 1 argument(s), got 0");
}

#[test]
fn test_error_display_type_mismatch() {
    let err = StdlibError::type_mismatch("core.assert", 1, "bool", "number");
    assert_eq!(
        format!("{err}"),
        "core.assert: argument 1 expected bool, got number"
    );
}

#[test]
fn test_error_display_unknown_function() {
    let err = StdlibError::unknown_function("core", "foo");
    assert_eq!(format!("{err}"), "Unknown function: core.foo");
}

#[test]
fn test_error_display_assertion_failed() {
    let err = StdlibError::AssertionFailed {
        message: "x > 0".into(),
    };
    assert_eq!(format!("{err}"), "Assertion failed: x > 0");
}

// ══════════════════════════════════════════════════════════════════════════════
// SumVariant tests
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_sum_variant_unit() {
    let v = Value::unit_variant("Status", "Active");
    assert_eq!(v.type_name(), "Status");
    assert_eq!(format!("{v}"), "Active");
    assert!(v.is_truthy());
}

#[test]
fn test_sum_variant_single_field() {
    let v = Value::sum_variant("Shape", "Circle", vec![Value::Number(5.0)]);
    assert_eq!(v.type_name(), "Shape");
    assert_eq!(format!("{v}"), "Circle(5)");
}

#[test]
fn test_sum_variant_multi_field() {
    let v = Value::sum_variant(
        "Shape",
        "Rectangle",
        vec![Value::Number(10.0), Value::Number(20.0)],
    );
    assert_eq!(format!("{v}"), "Rectangle(10, 20)");
}

#[test]
fn test_sum_variant_equality_same() {
    let a = Value::unit_variant("Status", "Active");
    let b = Value::unit_variant("Status", "Active");
    assert_eq!(a, b);
}

#[test]
fn test_sum_variant_equality_different_variant() {
    let a = Value::unit_variant("Status", "Active");
    let b = Value::unit_variant("Status", "Inactive");
    assert_ne!(a, b);
}

#[test]
fn test_sum_variant_equality_different_type() {
    // Nominal: same variant name but different declaring type → not equal
    let a = Value::unit_variant("Status", "Active");
    let b = Value::unit_variant("Priority", "Active");
    assert_ne!(a, b);
}

#[test]
fn test_sum_variant_equality_with_fields() {
    let a = Value::sum_variant("Shape", "Circle", vec![Value::Number(5.0)]);
    let b = Value::sum_variant("Shape", "Circle", vec![Value::Number(5.0)]);
    let c = Value::sum_variant("Shape", "Circle", vec![Value::Number(10.0)]);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn test_sum_variant_not_equal_to_other_types() {
    let v = Value::unit_variant("Status", "Active");
    assert_ne!(v, Value::String("Active".into()));
    assert_ne!(v, Value::Bool(true));
    assert_ne!(v, Value::Nil);
}

#[test]
fn test_sum_variant_accessor() {
    let v = Value::sum_variant("Shape", "Circle", vec![Value::Number(5.0)]);
    let (type_name, variant, fields) = v.as_variant().unwrap();
    assert_eq!(type_name, "Shape");
    assert_eq!(variant, "Circle");
    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0], Value::Number(5.0));
    // Non-variant returns None
    assert_eq!(Value::Nil.as_variant(), None);
}

#[test]
fn test_declared_type_name() {
    assert_eq!(
        Value::unit_variant("Status", "Active").declared_type_name(),
        Some("Status")
    );
    assert_eq!(
        Value::named_record("Todo", BTreeMap::new()).declared_type_name(),
        Some("Todo")
    );
    assert_eq!(Value::record(BTreeMap::new()).declared_type_name(), None);
    assert_eq!(Value::Number(1.0).declared_type_name(), None);
    assert_eq!(Value::Nil.declared_type_name(), None);
}

#[test]
fn test_from_btreemap_creates_anonymous_record() {
    let mut fields = BTreeMap::new();
    fields.insert("x".to_string(), Value::Number(1.0));
    let v: Value = fields.into();
    assert_eq!(v.type_name(), "record");
    assert_eq!(v.declared_type_name(), None);
}

// ══════════════════════════════════════════════════════════════════════════════
// Determinism test
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_core_determinism_100_iterations() {
    let c = core();
    let args_log = vec![Value::Number(42.0)];
    let args_assert = vec![Value::Bool(true), Value::String("ok".into())];
    let args_type_of = vec![Value::List(vec![Value::Number(1.0)])];
    let args_cap = vec![Value::String("http".into())];

    let ref_log = c.call("log", args_log.clone()).unwrap();
    let ref_assert = c.call("assert", args_assert.clone()).unwrap();
    let ref_type_of = c.call("type_of", args_type_of.clone()).unwrap();
    let ref_cap = c.call("capability", args_cap.clone()).unwrap();

    for i in 0..100 {
        assert_eq!(
            c.call("log", args_log.clone()).unwrap(),
            ref_log,
            "log diverged at iteration {i}"
        );
        assert_eq!(
            c.call("assert", args_assert.clone()).unwrap(),
            ref_assert,
            "assert diverged at iteration {i}"
        );
        assert_eq!(
            c.call("type_of", args_type_of.clone()).unwrap(),
            ref_type_of,
            "type_of diverged at iteration {i}"
        );
        assert_eq!(
            c.call("capability", args_cap.clone()).unwrap(),
            ref_cap,
            "capability diverged at iteration {i}"
        );
    }
}
