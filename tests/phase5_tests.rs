//! Tests for Phase 5 stdlib modules: record, time, convert, json, timer.

use std::collections::BTreeMap;

use pepl_stdlib::modules::convert::ConvertModule;
use pepl_stdlib::modules::json::JsonModule;
use pepl_stdlib::modules::record::RecordModule;
use pepl_stdlib::modules::time::TimeModule;
use pepl_stdlib::modules::timer::TimerModule;
use pepl_stdlib::{StdlibModule, Value};

// ══════════════════════════════════════════════════════════════════════════════
// Helpers
// ══════════════════════════════════════════════════════════════════════════════

fn rec(pairs: Vec<(&str, Value)>) -> Value {
    let mut fields = BTreeMap::new();
    for (k, v) in pairs {
        fields.insert(k.to_string(), v);
    }
    Value::record(fields)
}

fn s(val: &str) -> Value {
    Value::String(val.to_string())
}

fn n(val: f64) -> Value {
    Value::Number(val)
}

fn b(val: bool) -> Value {
    Value::Bool(val)
}

fn is_ok(val: &Value) -> bool {
    matches!(val, Value::Result(rv) if matches!(rv.as_ref(), pepl_stdlib::ResultValue::Ok(_)))
}

fn is_err(val: &Value) -> bool {
    matches!(val, Value::Result(rv) if matches!(rv.as_ref(), pepl_stdlib::ResultValue::Err(_)))
}

fn unwrap_ok(val: Value) -> Value {
    match val {
        Value::Result(rv) => match *rv {
            pepl_stdlib::ResultValue::Ok(v) => v,
            pepl_stdlib::ResultValue::Err(e) => panic!("expected Ok, got Err({:?})", e),
        },
        _ => panic!("expected Result, got {:?}", val),
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// record module
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn record_get_existing_key() {
    let m = RecordModule::new();
    let r = rec(vec![("name", s("alice")), ("age", n(30.0))]);
    let result = m.call("get", vec![r, s("name")]).unwrap();
    assert_eq!(result, s("alice"));
}

#[test]
fn record_get_missing_key_returns_nil() {
    let m = RecordModule::new();
    let r = rec(vec![("name", s("alice"))]);
    let result = m.call("get", vec![r, s("missing")]).unwrap();
    assert_eq!(result, Value::Nil);
}

#[test]
fn record_set_adds_field() {
    let m = RecordModule::new();
    let r = rec(vec![("a", n(1.0))]);
    let result = m.call("set", vec![r, s("b"), n(2.0)]).unwrap();
    // Original field preserved, new field added
    let got_a = m.call("get", vec![result.clone(), s("a")]).unwrap();
    let got_b = m.call("get", vec![result, s("b")]).unwrap();
    assert_eq!(got_a, n(1.0));
    assert_eq!(got_b, n(2.0));
}

#[test]
fn record_set_overwrites_existing_field() {
    let m = RecordModule::new();
    let r = rec(vec![("x", n(1.0))]);
    let result = m.call("set", vec![r, s("x"), n(99.0)]).unwrap();
    let got = m.call("get", vec![result, s("x")]).unwrap();
    assert_eq!(got, n(99.0));
}

#[test]
fn record_has_true_and_false() {
    let m = RecordModule::new();
    let r = rec(vec![("key", s("val"))]);
    assert_eq!(m.call("has", vec![r.clone(), s("key")]).unwrap(), b(true));
    assert_eq!(m.call("has", vec![r, s("nope")]).unwrap(), b(false));
}

#[test]
fn record_keys_deterministic_order() {
    let m = RecordModule::new();
    let r = rec(vec![("z", n(1.0)), ("a", n(2.0)), ("m", n(3.0))]);
    let result = m.call("keys", vec![r]).unwrap();
    // BTreeMap guarantees alphabetical order
    assert_eq!(result, Value::List(vec![s("a"), s("m"), s("z")]));
}

#[test]
fn record_values_deterministic_order() {
    let m = RecordModule::new();
    let r = rec(vec![("z", n(1.0)), ("a", n(2.0)), ("m", n(3.0))]);
    let result = m.call("values", vec![r]).unwrap();
    assert_eq!(result, Value::List(vec![n(2.0), n(3.0), n(1.0)]));
}

#[test]
fn record_empty_record_keys_and_values() {
    let m = RecordModule::new();
    let r = rec(vec![]);
    assert_eq!(
        m.call("keys", vec![r.clone()]).unwrap(),
        Value::List(vec![])
    );
    assert_eq!(m.call("values", vec![r]).unwrap(), Value::List(vec![]));
}

#[test]
fn record_wrong_arg_count() {
    let m = RecordModule::new();
    assert!(m.call("get", vec![]).is_err());
    assert!(m.call("set", vec![rec(vec![])]).is_err());
    assert!(m.call("has", vec![]).is_err());
    assert!(m.call("keys", vec![]).is_err());
    assert!(m.call("values", vec![]).is_err());
}

#[test]
fn record_wrong_type() {
    let m = RecordModule::new();
    assert!(m.call("get", vec![n(1.0), s("key")]).is_err());
    assert!(m.call("keys", vec![s("not a record")]).is_err());
}

#[test]
fn record_unknown_function() {
    let m = RecordModule::new();
    assert!(m.call("nonexistent", vec![]).is_err());
}

#[test]
fn record_has_function() {
    let m = RecordModule::new();
    assert!(m.has_function("get"));
    assert!(m.has_function("set"));
    assert!(m.has_function("has"));
    assert!(m.has_function("keys"));
    assert!(m.has_function("values"));
    assert!(!m.has_function("delete"));
    assert_eq!(m.name(), "record");
}

// ══════════════════════════════════════════════════════════════════════════════
// time module
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn time_now_returns_zero_stub() {
    let m = TimeModule::new();
    assert_eq!(m.call("now", vec![]).unwrap(), n(0.0));
}

#[test]
fn time_diff_returns_difference() {
    let m = TimeModule::new();
    let result = m.call("diff", vec![n(5000.0), n(3000.0)]).unwrap();
    assert_eq!(result, n(2000.0));
}

#[test]
fn time_diff_negative() {
    let m = TimeModule::new();
    let result = m.call("diff", vec![n(1000.0), n(5000.0)]).unwrap();
    assert_eq!(result, n(-4000.0));
}

#[test]
fn time_start_of_day() {
    let m = TimeModule::new();
    // 2024-01-15 at 14:30:00 UTC = 1705325400000 ms
    let ts = 1_705_325_400_000.0;
    let result = m.call("start_of_day", vec![n(ts)]).unwrap();
    // Should truncate to midnight: 1705276800000
    let expected = 1_705_276_800_000.0;
    assert_eq!(result, n(expected));
}

#[test]
fn time_day_of_week_epoch() {
    let m = TimeModule::new();
    // Jan 1, 1970 = Thursday = 4
    assert_eq!(m.call("day_of_week", vec![n(0.0)]).unwrap(), n(4.0));
}

#[test]
fn time_day_of_week_known_sunday() {
    let m = TimeModule::new();
    // Jan 4, 1970 = Sunday = 0
    let ts = 3.0 * 86_400_000.0;
    assert_eq!(m.call("day_of_week", vec![n(ts)]).unwrap(), n(0.0));
}

#[test]
fn time_format_date() {
    let m = TimeModule::new();
    // 2024-01-15 00:00:00 UTC = 1705276800000 ms
    let ts = 1_705_276_800_000.0;
    let result = m.call("format", vec![n(ts), s("YYYY-MM-DD")]).unwrap();
    assert_eq!(result, s("2024-01-15"));
}

#[test]
fn time_format_datetime() {
    let m = TimeModule::new();
    // Epoch = 1970-01-01 00:00:00
    let result = m
        .call("format", vec![n(0.0), s("YYYY-MM-DD HH:mm:ss")])
        .unwrap();
    assert_eq!(result, s("1970-01-01 00:00:00"));
}

#[test]
fn time_wrong_arg_count() {
    let m = TimeModule::new();
    assert!(m.call("now", vec![n(1.0)]).is_err());
    assert!(m.call("diff", vec![n(1.0)]).is_err());
    assert!(m.call("format", vec![]).is_err());
    assert!(m.call("day_of_week", vec![]).is_err());
    assert!(m.call("start_of_day", vec![]).is_err());
}

#[test]
fn time_wrong_type() {
    let m = TimeModule::new();
    assert!(m.call("diff", vec![s("a"), n(1.0)]).is_err());
    assert!(m.call("format", vec![n(0.0), n(0.0)]).is_err());
}

#[test]
fn time_has_function() {
    let m = TimeModule::new();
    assert!(m.has_function("now"));
    assert!(m.has_function("format"));
    assert!(m.has_function("diff"));
    assert!(m.has_function("day_of_week"));
    assert!(m.has_function("start_of_day"));
    assert!(!m.has_function("sleep"));
    assert_eq!(m.name(), "time");
}

// ══════════════════════════════════════════════════════════════════════════════
// convert module
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn convert_to_string_number() {
    let m = ConvertModule::new();
    let result = m.call("to_string", vec![n(42.0)]).unwrap();
    assert_eq!(result, s("42"));
}

#[test]
fn convert_to_string_bool() {
    let m = ConvertModule::new();
    assert_eq!(m.call("to_string", vec![b(true)]).unwrap(), s("true"));
    assert_eq!(m.call("to_string", vec![b(false)]).unwrap(), s("false"));
}

#[test]
fn convert_to_string_nil() {
    let m = ConvertModule::new();
    assert_eq!(m.call("to_string", vec![Value::Nil]).unwrap(), s("nil"));
}

#[test]
fn convert_to_string_string() {
    let m = ConvertModule::new();
    assert_eq!(m.call("to_string", vec![s("hello")]).unwrap(), s("hello"));
}

#[test]
fn convert_to_number_from_string() {
    let m = ConvertModule::new();
    let result = m.call("to_number", vec![s("42")]).unwrap();
    assert!(is_ok(&result));
    assert_eq!(unwrap_ok(result), n(42.0));
}

#[test]
fn convert_to_number_from_float_string() {
    let m = ConvertModule::new();
    let result = m.call("to_number", vec![s("3.14")]).unwrap();
    assert!(is_ok(&result));
    assert_eq!(unwrap_ok(result), n(3.14));
}

#[test]
fn convert_to_number_invalid_string() {
    let m = ConvertModule::new();
    let result = m.call("to_number", vec![s("abc")]).unwrap();
    assert!(is_err(&result));
}

#[test]
fn convert_to_number_from_bool() {
    let m = ConvertModule::new();
    assert_eq!(
        unwrap_ok(m.call("to_number", vec![b(true)]).unwrap()),
        n(1.0)
    );
    assert_eq!(
        unwrap_ok(m.call("to_number", vec![b(false)]).unwrap()),
        n(0.0)
    );
}

#[test]
fn convert_to_number_from_number() {
    let m = ConvertModule::new();
    assert_eq!(
        unwrap_ok(m.call("to_number", vec![n(7.0)]).unwrap()),
        n(7.0)
    );
}

#[test]
fn convert_to_number_from_nil() {
    let m = ConvertModule::new();
    let result = m.call("to_number", vec![Value::Nil]).unwrap();
    assert!(is_err(&result));
}

#[test]
fn convert_parse_int_valid() {
    let m = ConvertModule::new();
    assert_eq!(
        unwrap_ok(m.call("parse_int", vec![s("42")]).unwrap()),
        n(42.0)
    );
    assert_eq!(
        unwrap_ok(m.call("parse_int", vec![s("-10")]).unwrap()),
        n(-10.0)
    );
}

#[test]
fn convert_parse_int_rejects_float() {
    let m = ConvertModule::new();
    let result = m.call("parse_int", vec![s("3.14")]).unwrap();
    assert!(is_err(&result));
}

#[test]
fn convert_parse_int_invalid() {
    let m = ConvertModule::new();
    let result = m.call("parse_int", vec![s("abc")]).unwrap();
    assert!(is_err(&result));
}

#[test]
fn convert_parse_float_valid() {
    let m = ConvertModule::new();
    assert_eq!(
        unwrap_ok(m.call("parse_float", vec![s("3.14")]).unwrap()),
        n(3.14)
    );
    assert_eq!(
        unwrap_ok(m.call("parse_float", vec![s("42")]).unwrap()),
        n(42.0)
    );
}

#[test]
fn convert_parse_float_invalid() {
    let m = ConvertModule::new();
    let result = m.call("parse_float", vec![s("abc")]).unwrap();
    assert!(is_err(&result));
}

#[test]
fn convert_to_bool_truthy() {
    let m = ConvertModule::new();
    assert_eq!(m.call("to_bool", vec![n(1.0)]).unwrap(), b(true));
    assert_eq!(m.call("to_bool", vec![s("hello")]).unwrap(), b(true));
    assert_eq!(m.call("to_bool", vec![b(true)]).unwrap(), b(true));
    assert_eq!(
        m.call("to_bool", vec![Value::List(vec![n(1.0)])]).unwrap(),
        b(true)
    );
}

#[test]
fn convert_to_bool_falsy() {
    let m = ConvertModule::new();
    assert_eq!(m.call("to_bool", vec![n(0.0)]).unwrap(), b(false));
    assert_eq!(m.call("to_bool", vec![s("")]).unwrap(), b(false));
    assert_eq!(m.call("to_bool", vec![b(false)]).unwrap(), b(false));
    assert_eq!(m.call("to_bool", vec![Value::Nil]).unwrap(), b(false));
}

#[test]
fn convert_wrong_arg_count() {
    let m = ConvertModule::new();
    assert!(m.call("to_string", vec![]).is_err());
    assert!(m.call("to_number", vec![]).is_err());
    assert!(m.call("parse_int", vec![]).is_err());
    assert!(m.call("parse_float", vec![]).is_err());
    assert!(m.call("to_bool", vec![]).is_err());
}

#[test]
fn convert_parse_int_wrong_type() {
    let m = ConvertModule::new();
    assert!(m.call("parse_int", vec![n(1.0)]).is_err());
}

#[test]
fn convert_has_function() {
    let m = ConvertModule::new();
    assert!(m.has_function("to_string"));
    assert!(m.has_function("to_number"));
    assert!(m.has_function("parse_int"));
    assert!(m.has_function("parse_float"));
    assert!(m.has_function("to_bool"));
    assert!(!m.has_function("cast"));
    assert_eq!(m.name(), "convert");
}

// ══════════════════════════════════════════════════════════════════════════════
// json module
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn json_parse_object() {
    let m = JsonModule::new();
    let result = m
        .call("parse", vec![s(r#"{"a": 1, "b": "hello"}"#)])
        .unwrap();
    assert!(is_ok(&result));
    let val = unwrap_ok(result);
    match &val {
        Value::Record { fields, .. } => {
            assert_eq!(fields.get("a"), Some(&n(1.0)));
            assert_eq!(fields.get("b"), Some(&s("hello")));
        }
        _ => panic!("expected record, got {:?}", val),
    }
}

#[test]
fn json_parse_array() {
    let m = JsonModule::new();
    let result = m.call("parse", vec![s("[1, 2, 3]")]).unwrap();
    let val = unwrap_ok(result);
    assert_eq!(val, Value::List(vec![n(1.0), n(2.0), n(3.0)]));
}

#[test]
fn json_parse_primitives() {
    let m = JsonModule::new();
    assert_eq!(unwrap_ok(m.call("parse", vec![s("42")]).unwrap()), n(42.0));
    assert_eq!(
        unwrap_ok(m.call("parse", vec![s("true")]).unwrap()),
        b(true)
    );
    assert_eq!(
        unwrap_ok(m.call("parse", vec![s("null")]).unwrap()),
        Value::Nil
    );
    assert_eq!(
        unwrap_ok(m.call("parse", vec![s(r#""hi""#)]).unwrap()),
        s("hi")
    );
}

#[test]
fn json_parse_invalid() {
    let m = JsonModule::new();
    let result = m.call("parse", vec![s("{invalid}")]).unwrap();
    assert!(is_err(&result));
}

#[test]
fn json_parse_empty_string() {
    let m = JsonModule::new();
    let result = m.call("parse", vec![s("")]).unwrap();
    assert!(is_err(&result));
}

#[test]
fn json_stringify_record() {
    let m = JsonModule::new();
    let r = rec(vec![("name", s("alice")), ("age", n(30.0))]);
    let result = m.call("stringify", vec![r]).unwrap();
    // Should be valid JSON
    match &result {
        Value::String(json_str) => {
            let parsed: serde_json::Value = serde_json::from_str(json_str).unwrap();
            assert_eq!(parsed["name"], "alice");
            assert_eq!(parsed["age"], 30.0);
        }
        _ => panic!("expected string"),
    }
}

#[test]
fn json_stringify_list() {
    let m = JsonModule::new();
    let result = m
        .call("stringify", vec![Value::List(vec![n(1.0), n(2.0)])])
        .unwrap();
    assert_eq!(result, s("[1.0,2.0]"));
}

#[test]
fn json_stringify_nil() {
    let m = JsonModule::new();
    assert_eq!(m.call("stringify", vec![Value::Nil]).unwrap(), s("null"));
}

#[test]
fn json_stringify_bool() {
    let m = JsonModule::new();
    assert_eq!(m.call("stringify", vec![b(true)]).unwrap(), s("true"));
}

#[test]
fn json_roundtrip() {
    let m = JsonModule::new();
    let original = rec(vec![
        ("items", Value::List(vec![n(1.0), n(2.0), n(3.0)])),
        ("name", s("test")),
        ("ok", b(true)),
    ]);
    let json_str = m.call("stringify", vec![original.clone()]).unwrap();
    let parsed = m.call("parse", vec![json_str]).unwrap();
    let roundtripped = unwrap_ok(parsed);
    // Should structurally match
    assert_eq!(original, roundtripped);
}

#[test]
fn json_wrong_arg_count() {
    let m = JsonModule::new();
    assert!(m.call("parse", vec![]).is_err());
    assert!(m.call("stringify", vec![]).is_err());
}

#[test]
fn json_parse_wrong_type() {
    let m = JsonModule::new();
    assert!(m.call("parse", vec![n(1.0)]).is_err());
}

#[test]
fn json_has_function() {
    let m = JsonModule::new();
    assert!(m.has_function("parse"));
    assert!(m.has_function("stringify"));
    assert!(!m.has_function("decode"));
    assert_eq!(m.name(), "json");
}

// ══════════════════════════════════════════════════════════════════════════════
// timer module
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn timer_start_returns_id() {
    let m = TimerModule::new();
    let result = m.call("start", vec![s("tick"), n(1000.0)]).unwrap();
    assert_eq!(result, s("tick"));
}

#[test]
fn timer_start_once_returns_id() {
    let m = TimerModule::new();
    let result = m.call("start_once", vec![s("delayed"), n(5000.0)]).unwrap();
    assert_eq!(result, s("delayed"));
}

#[test]
fn timer_stop_returns_nil() {
    let m = TimerModule::new();
    assert_eq!(m.call("stop", vec![s("tick")]).unwrap(), Value::Nil);
}

#[test]
fn timer_stop_all_returns_nil() {
    let m = TimerModule::new();
    assert_eq!(m.call("stop_all", vec![]).unwrap(), Value::Nil);
}

#[test]
fn timer_wrong_arg_count() {
    let m = TimerModule::new();
    assert!(m.call("start", vec![s("id")]).is_err());
    assert!(m.call("start_once", vec![]).is_err());
    assert!(m.call("stop", vec![]).is_err());
    assert!(m.call("stop_all", vec![n(1.0)]).is_err());
}

#[test]
fn timer_wrong_type() {
    let m = TimerModule::new();
    assert!(m.call("start", vec![n(1.0), n(1000.0)]).is_err());
    assert!(m.call("start", vec![s("id"), s("not_number")]).is_err());
    assert!(m.call("stop", vec![n(1.0)]).is_err());
}

#[test]
fn timer_has_function() {
    let m = TimerModule::new();
    assert!(m.has_function("start"));
    assert!(m.has_function("start_once"));
    assert!(m.has_function("stop"));
    assert!(m.has_function("stop_all"));
    assert!(!m.has_function("pause"));
    assert_eq!(m.name(), "timer");
}

// ══════════════════════════════════════════════════════════════════════════════
// 100-iteration determinism
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn phase5_determinism_100_iterations() {
    let record_mod = RecordModule::new();
    let time_mod = TimeModule::new();
    let convert_mod = ConvertModule::new();
    let json_mod = JsonModule::new();
    let timer_mod = TimerModule::new();

    let r = rec(vec![("x", n(1.0)), ("y", s("hello"))]);
    let ts = 1_705_276_800_000.0;

    for _ in 0..100 {
        // record
        assert_eq!(
            record_mod.call("get", vec![r.clone(), s("x")]).unwrap(),
            n(1.0)
        );
        assert_eq!(
            record_mod.call("has", vec![r.clone(), s("y")]).unwrap(),
            b(true)
        );
        assert_eq!(
            record_mod.call("keys", vec![r.clone()]).unwrap(),
            Value::List(vec![s("x"), s("y")])
        );

        // time
        assert_eq!(time_mod.call("now", vec![]).unwrap(), n(0.0));
        assert_eq!(
            time_mod.call("diff", vec![n(5000.0), n(3000.0)]).unwrap(),
            n(2000.0)
        );
        assert_eq!(
            time_mod
                .call("format", vec![n(ts), s("YYYY-MM-DD")])
                .unwrap(),
            s("2024-01-15")
        );

        // convert
        assert_eq!(
            convert_mod.call("to_string", vec![n(42.0)]).unwrap(),
            s("42")
        );
        assert_eq!(convert_mod.call("to_bool", vec![n(0.0)]).unwrap(), b(false));

        // json
        let json_str = json_mod.call("stringify", vec![r.clone()]).unwrap();
        let parsed = json_mod.call("parse", vec![json_str]).unwrap();
        assert_eq!(unwrap_ok(parsed), r);

        // timer
        assert_eq!(
            timer_mod.call("start", vec![s("t"), n(100.0)]).unwrap(),
            s("t")
        );
        assert_eq!(timer_mod.call("stop_all", vec![]).unwrap(), Value::Nil);
    }
}
