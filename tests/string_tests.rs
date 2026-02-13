//! Integration tests for `pepl-stdlib` Phase 3: string module.

use pepl_stdlib::modules::string::StringModule;
use pepl_stdlib::{StdlibError, StdlibModule, Value};
use std::collections::BTreeMap;

// ── Helpers ───────────────────────────────────────────────────────────────────

fn string_mod() -> StringModule {
    StringModule::new()
}

fn s(val: &str) -> Value {
    Value::String(val.to_string())
}

fn num(n: f64) -> Value {
    Value::Number(n)
}

fn call(func: &str, args: Vec<Value>) -> Result<Value, StdlibError> {
    string_mod().call(func, args)
}

fn call_ok(func: &str, args: Vec<Value>) -> Value {
    call(func, args).expect(&format!("string.{func} should succeed"))
}

fn expect_str(func: &str, args: Vec<Value>) -> String {
    match call_ok(func, args) {
        Value::String(s) => s,
        other => panic!("expected String, got {other:?}"),
    }
}

fn expect_bool(func: &str, args: Vec<Value>) -> bool {
    match call_ok(func, args) {
        Value::Bool(b) => b,
        other => panic!("expected Bool, got {other:?}"),
    }
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
    assert_eq!(string_mod().name(), "string");
}

#[test]
fn test_has_function_known() {
    let m = string_mod();
    for f in &[
        "length",
        "concat",
        "contains",
        "slice",
        "trim",
        "split",
        "to_upper",
        "to_lower",
        "starts_with",
        "ends_with",
        "replace",
        "replace_all",
        "pad_start",
        "pad_end",
        "repeat",
        "join",
        "format",
        "from",
        "is_empty",
        "index_of",
    ] {
        assert!(m.has_function(f), "string should have function {f}");
    }
}

#[test]
fn test_has_function_unknown() {
    assert!(!string_mod().has_function("nonexistent"));
}

#[test]
fn test_unknown_function_error() {
    let err = call("nonexistent", vec![]).unwrap_err();
    assert!(matches!(err, StdlibError::UnknownFunction { .. }));
}

// ══════════════════════════════════════════════════════════════════════════════
// string.length
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_length_basic() {
    assert_eq!(expect_num("length", vec![s("hello")]), 5.0);
}

#[test]
fn test_length_empty() {
    assert_eq!(expect_num("length", vec![s("")]), 0.0);
}

#[test]
fn test_length_unicode() {
    // "café" has 4 characters
    assert_eq!(expect_num("length", vec![s("café")]), 4.0);
}

#[test]
fn test_length_emoji() {
    // Each emoji is one char (but multiple bytes)
    assert_eq!(expect_num("length", vec![s("😀")]), 1.0);
}

#[test]
fn test_length_multibyte() {
    // Chinese characters — 3 bytes each but 1 char
    assert_eq!(expect_num("length", vec![s("你好")]), 2.0);
}

#[test]
fn test_length_wrong_type() {
    let err = call("length", vec![num(42.0)]).unwrap_err();
    assert!(matches!(err, StdlibError::TypeMismatch { .. }));
}

// ══════════════════════════════════════════════════════════════════════════════
// string.concat
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_concat_basic() {
    assert_eq!(
        expect_str("concat", vec![s("hello"), s(" world")]),
        "hello world"
    );
}

#[test]
fn test_concat_empty_left() {
    assert_eq!(expect_str("concat", vec![s(""), s("world")]), "world");
}

#[test]
fn test_concat_empty_right() {
    assert_eq!(expect_str("concat", vec![s("hello"), s("")]), "hello");
}

#[test]
fn test_concat_both_empty() {
    assert_eq!(expect_str("concat", vec![s(""), s("")]), "");
}

// ══════════════════════════════════════════════════════════════════════════════
// string.contains
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_contains_found() {
    assert!(expect_bool("contains", vec![s("hello world"), s("world")]));
}

#[test]
fn test_contains_not_found() {
    assert!(!expect_bool("contains", vec![s("hello world"), s("xyz")]));
}

#[test]
fn test_contains_empty_needle() {
    assert!(expect_bool("contains", vec![s("hello"), s("")]));
}

#[test]
fn test_contains_empty_haystack() {
    assert!(!expect_bool("contains", vec![s(""), s("a")]));
}

#[test]
fn test_contains_case_sensitive() {
    assert!(!expect_bool("contains", vec![s("Hello"), s("hello")]));
}

// ══════════════════════════════════════════════════════════════════════════════
// string.slice
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_slice_basic() {
    assert_eq!(
        expect_str("slice", vec![s("hello"), num(1.0), num(4.0)]),
        "ell"
    );
}

#[test]
fn test_slice_from_start() {
    assert_eq!(
        expect_str("slice", vec![s("hello"), num(0.0), num(3.0)]),
        "hel"
    );
}

#[test]
fn test_slice_to_end() {
    assert_eq!(
        expect_str("slice", vec![s("hello"), num(2.0), num(5.0)]),
        "llo"
    );
}

#[test]
fn test_slice_empty_range() {
    assert_eq!(
        expect_str("slice", vec![s("hello"), num(2.0), num(2.0)]),
        ""
    );
}

#[test]
fn test_slice_reversed_range() {
    assert_eq!(
        expect_str("slice", vec![s("hello"), num(4.0), num(2.0)]),
        ""
    );
}

#[test]
fn test_slice_out_of_bounds_clamps() {
    assert_eq!(
        expect_str("slice", vec![s("hello"), num(0.0), num(100.0)]),
        "hello"
    );
}

#[test]
fn test_slice_negative_start_clamps() {
    assert_eq!(
        expect_str("slice", vec![s("hello"), num(-5.0), num(3.0)]),
        "hel"
    );
}

#[test]
fn test_slice_unicode() {
    assert_eq!(
        expect_str("slice", vec![s("café"), num(0.0), num(3.0)]),
        "caf"
    );
    assert_eq!(
        expect_str("slice", vec![s("café"), num(3.0), num(4.0)]),
        "é"
    );
}

// ══════════════════════════════════════════════════════════════════════════════
// string.trim
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_trim_spaces() {
    assert_eq!(expect_str("trim", vec![s("  hello  ")]), "hello");
}

#[test]
fn test_trim_tabs_newlines() {
    assert_eq!(expect_str("trim", vec![s("\t\nhello\n\t")]), "hello");
}

#[test]
fn test_trim_no_whitespace() {
    assert_eq!(expect_str("trim", vec![s("hello")]), "hello");
}

#[test]
fn test_trim_all_whitespace() {
    assert_eq!(expect_str("trim", vec![s("   ")]), "");
}

#[test]
fn test_trim_empty() {
    assert_eq!(expect_str("trim", vec![s("")]), "");
}

// ══════════════════════════════════════════════════════════════════════════════
// string.split
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_split_basic() {
    let result = call_ok("split", vec![s("a,b,c"), s(",")]);
    assert_eq!(result, Value::List(vec![s("a"), s("b"), s("c")]));
}

#[test]
fn test_split_not_found() {
    let result = call_ok("split", vec![s("hello"), s(",")]);
    assert_eq!(result, Value::List(vec![s("hello")]));
}

#[test]
fn test_split_empty_delimiter() {
    let result = call_ok("split", vec![s("abc"), s("")]);
    assert_eq!(result, Value::List(vec![s("a"), s("b"), s("c")]));
}

#[test]
fn test_split_empty_string() {
    let result = call_ok("split", vec![s(""), s(",")]);
    assert_eq!(result, Value::List(vec![s("")]));
}

#[test]
fn test_split_multi_char_delimiter() {
    let result = call_ok("split", vec![s("a::b::c"), s("::")]);
    assert_eq!(result, Value::List(vec![s("a"), s("b"), s("c")]));
}

#[test]
fn test_split_trailing_delimiter() {
    let result = call_ok("split", vec![s("a,b,"), s(",")]);
    assert_eq!(result, Value::List(vec![s("a"), s("b"), s("")]));
}

// ══════════════════════════════════════════════════════════════════════════════
// string.to_upper / string.to_lower
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_to_upper_basic() {
    assert_eq!(expect_str("to_upper", vec![s("hello")]), "HELLO");
}

#[test]
fn test_to_upper_mixed() {
    assert_eq!(
        expect_str("to_upper", vec![s("Hello World")]),
        "HELLO WORLD"
    );
}

#[test]
fn test_to_upper_empty() {
    assert_eq!(expect_str("to_upper", vec![s("")]), "");
}

#[test]
fn test_to_lower_basic() {
    assert_eq!(expect_str("to_lower", vec![s("HELLO")]), "hello");
}

#[test]
fn test_to_lower_mixed() {
    assert_eq!(
        expect_str("to_lower", vec![s("Hello World")]),
        "hello world"
    );
}

#[test]
fn test_to_lower_empty() {
    assert_eq!(expect_str("to_lower", vec![s("")]), "");
}

// ══════════════════════════════════════════════════════════════════════════════
// string.starts_with / string.ends_with
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_starts_with_true() {
    assert!(expect_bool(
        "starts_with",
        vec![s("hello world"), s("hello")]
    ));
}

#[test]
fn test_starts_with_false() {
    assert!(!expect_bool(
        "starts_with",
        vec![s("hello world"), s("world")]
    ));
}

#[test]
fn test_starts_with_empty_prefix() {
    assert!(expect_bool("starts_with", vec![s("hello"), s("")]));
}

#[test]
fn test_starts_with_full_match() {
    assert!(expect_bool("starts_with", vec![s("hello"), s("hello")]));
}

#[test]
fn test_ends_with_true() {
    assert!(expect_bool("ends_with", vec![s("hello world"), s("world")]));
}

#[test]
fn test_ends_with_false() {
    assert!(!expect_bool(
        "ends_with",
        vec![s("hello world"), s("hello")]
    ));
}

#[test]
fn test_ends_with_empty_suffix() {
    assert!(expect_bool("ends_with", vec![s("hello"), s("")]));
}

// ══════════════════════════════════════════════════════════════════════════════
// string.replace (first occurrence only)
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_replace_first() {
    assert_eq!(
        expect_str("replace", vec![s("aabaa"), s("a"), s("x")]),
        "xabaa"
    );
}

#[test]
fn test_replace_not_found() {
    assert_eq!(
        expect_str("replace", vec![s("hello"), s("xyz"), s("!")]),
        "hello"
    );
}

#[test]
fn test_replace_empty_old() {
    // Replacing empty string returns original
    assert_eq!(
        expect_str("replace", vec![s("hello"), s(""), s("x")]),
        "hello"
    );
}

#[test]
fn test_replace_with_empty() {
    assert_eq!(
        expect_str("replace", vec![s("hello world"), s("world"), s("")]),
        "hello "
    );
}

#[test]
fn test_replace_longer() {
    assert_eq!(
        expect_str("replace", vec![s("hello"), s("ell"), s("ELLO")]),
        "hELLOo"
    );
}

// ══════════════════════════════════════════════════════════════════════════════
// string.replace_all
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_replace_all_basic() {
    assert_eq!(
        expect_str("replace_all", vec![s("aabaa"), s("a"), s("x")]),
        "xxbxx"
    );
}

#[test]
fn test_replace_all_not_found() {
    assert_eq!(
        expect_str("replace_all", vec![s("hello"), s("xyz"), s("!")]),
        "hello"
    );
}

#[test]
fn test_replace_all_empty_old() {
    assert_eq!(
        expect_str("replace_all", vec![s("hello"), s(""), s("x")]),
        "hello"
    );
}

// ══════════════════════════════════════════════════════════════════════════════
// string.pad_start / string.pad_end
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_pad_start_basic() {
    assert_eq!(
        expect_str("pad_start", vec![s("42"), num(5.0), s("0")]),
        "00042"
    );
}

#[test]
fn test_pad_start_already_long() {
    assert_eq!(
        expect_str("pad_start", vec![s("hello"), num(3.0), s("x")]),
        "hello"
    );
}

#[test]
fn test_pad_start_exact_length() {
    assert_eq!(
        expect_str("pad_start", vec![s("hi"), num(2.0), s("x")]),
        "hi"
    );
}

#[test]
fn test_pad_start_multi_char_pad() {
    assert_eq!(
        expect_str("pad_start", vec![s("1"), num(5.0), s("ab")]),
        "abab1"
    );
}

#[test]
fn test_pad_start_empty_pad() {
    assert_eq!(
        expect_str("pad_start", vec![s("hi"), num(10.0), s("")]),
        "hi"
    );
}

#[test]
fn test_pad_end_basic() {
    assert_eq!(
        expect_str("pad_end", vec![s("hi"), num(5.0), s(".")]),
        "hi..."
    );
}

#[test]
fn test_pad_end_already_long() {
    assert_eq!(
        expect_str("pad_end", vec![s("hello"), num(3.0), s("x")]),
        "hello"
    );
}

#[test]
fn test_pad_end_multi_char_pad() {
    assert_eq!(
        expect_str("pad_end", vec![s("1"), num(5.0), s("ab")]),
        "1abab"
    );
}

// ══════════════════════════════════════════════════════════════════════════════
// string.repeat
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_repeat_basic() {
    assert_eq!(expect_str("repeat", vec![s("ab"), num(3.0)]), "ababab");
}

#[test]
fn test_repeat_zero() {
    assert_eq!(expect_str("repeat", vec![s("hello"), num(0.0)]), "");
}

#[test]
fn test_repeat_one() {
    assert_eq!(expect_str("repeat", vec![s("hello"), num(1.0)]), "hello");
}

#[test]
fn test_repeat_empty_string() {
    assert_eq!(expect_str("repeat", vec![s(""), num(5.0)]), "");
}

#[test]
fn test_repeat_negative_error() {
    let err = call("repeat", vec![s("x"), num(-1.0)]).unwrap_err();
    assert!(matches!(err, StdlibError::RuntimeError(_)));
}

#[test]
fn test_repeat_fractional_error() {
    let err = call("repeat", vec![s("x"), num(2.5)]).unwrap_err();
    assert!(matches!(err, StdlibError::RuntimeError(_)));
}

// ══════════════════════════════════════════════════════════════════════════════
// string.join
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_join_basic() {
    let items = Value::List(vec![s("a"), s("b"), s("c")]);
    assert_eq!(expect_str("join", vec![items, s(", ")]), "a, b, c");
}

#[test]
fn test_join_empty_list() {
    let items = Value::List(vec![]);
    assert_eq!(expect_str("join", vec![items, s(", ")]), "");
}

#[test]
fn test_join_single_item() {
    let items = Value::List(vec![s("only")]);
    assert_eq!(expect_str("join", vec![items, s(", ")]), "only");
}

#[test]
fn test_join_empty_separator() {
    let items = Value::List(vec![s("a"), s("b"), s("c")]);
    assert_eq!(expect_str("join", vec![items, s("")]), "abc");
}

#[test]
fn test_join_non_string_items_error() {
    let items = Value::List(vec![s("a"), num(42.0)]);
    let err = call("join", vec![items, s(", ")]).unwrap_err();
    assert!(matches!(err, StdlibError::TypeMismatch { .. }));
}

// ══════════════════════════════════════════════════════════════════════════════
// string.format
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_format_basic() {
    let mut fields = BTreeMap::new();
    fields.insert("name".to_string(), s("Alice"));
    let record = Value::Record {
        type_name: None,
        fields,
    };
    assert_eq!(
        expect_str("format", vec![s("Hello, {name}!"), record]),
        "Hello, Alice!"
    );
}

#[test]
fn test_format_multiple_placeholders() {
    let mut fields = BTreeMap::new();
    fields.insert("first".to_string(), s("Jane"));
    fields.insert("last".to_string(), s("Doe"));
    let record = Value::Record {
        type_name: None,
        fields,
    };
    assert_eq!(
        expect_str("format", vec![s("{first} {last}"), record]),
        "Jane Doe"
    );
}

#[test]
fn test_format_number_value() {
    let mut fields = BTreeMap::new();
    fields.insert("count".to_string(), num(42.0));
    let record = Value::Record {
        type_name: None,
        fields,
    };
    assert_eq!(
        expect_str("format", vec![s("Count: {count}"), record]),
        "Count: 42"
    );
}

#[test]
fn test_format_missing_placeholder() {
    let fields = BTreeMap::new();
    let record = Value::Record {
        type_name: None,
        fields,
    };
    assert_eq!(
        expect_str("format", vec![s("Hello, {name}!"), record]),
        "Hello, {name}!"
    );
}

#[test]
fn test_format_no_placeholders() {
    let fields = BTreeMap::new();
    let record = Value::Record {
        type_name: None,
        fields,
    };
    assert_eq!(expect_str("format", vec![s("Hello!"), record]), "Hello!");
}

#[test]
fn test_format_repeated_placeholder() {
    let mut fields = BTreeMap::new();
    fields.insert("x".to_string(), s("!"));
    let record = Value::Record {
        type_name: None,
        fields,
    };
    assert_eq!(expect_str("format", vec![s("{x}{x}{x}"), record]), "!!!");
}

#[test]
fn test_format_wrong_type() {
    let err = call("format", vec![s("template"), num(1.0)]).unwrap_err();
    assert!(matches!(err, StdlibError::TypeMismatch { .. }));
}

// ══════════════════════════════════════════════════════════════════════════════
// string.from
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_from_number() {
    assert_eq!(expect_str("from", vec![num(42.0)]), "42");
}

#[test]
fn test_from_number_decimal() {
    assert_eq!(expect_str("from", vec![num(3.14)]), "3.14");
}

#[test]
fn test_from_string() {
    assert_eq!(expect_str("from", vec![s("hello")]), "hello");
}

#[test]
fn test_from_bool() {
    assert_eq!(expect_str("from", vec![Value::Bool(true)]), "true");
    assert_eq!(expect_str("from", vec![Value::Bool(false)]), "false");
}

#[test]
fn test_from_nil() {
    assert_eq!(expect_str("from", vec![Value::Nil]), "nil");
}

#[test]
fn test_from_list() {
    let list = Value::List(vec![num(1.0), num(2.0), num(3.0)]);
    assert_eq!(expect_str("from", vec![list]), "[1, 2, 3]");
}

#[test]
fn test_from_wrong_arg_count() {
    let err = call("from", vec![]).unwrap_err();
    assert!(matches!(err, StdlibError::WrongArgCount { .. }));
}

// ══════════════════════════════════════════════════════════════════════════════
// string.is_empty
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_is_empty_true() {
    assert!(expect_bool("is_empty", vec![s("")]));
}

#[test]
fn test_is_empty_false() {
    assert!(!expect_bool("is_empty", vec![s("hello")]));
}

#[test]
fn test_is_empty_whitespace_is_not_empty() {
    assert!(!expect_bool("is_empty", vec![s(" ")]));
}

// ══════════════════════════════════════════════════════════════════════════════
// string.index_of
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_index_of_found() {
    assert_eq!(
        expect_num("index_of", vec![s("hello world"), s("world")]),
        6.0
    );
}

#[test]
fn test_index_of_not_found() {
    assert_eq!(expect_num("index_of", vec![s("hello"), s("xyz")]), -1.0);
}

#[test]
fn test_index_of_at_start() {
    assert_eq!(expect_num("index_of", vec![s("hello"), s("hel")]), 0.0);
}

#[test]
fn test_index_of_empty_sub() {
    assert_eq!(expect_num("index_of", vec![s("hello"), s("")]), 0.0);
}

#[test]
fn test_index_of_first_occurrence() {
    assert_eq!(expect_num("index_of", vec![s("abcabc"), s("abc")]), 0.0);
}

#[test]
fn test_index_of_unicode() {
    // "café" — 'é' is at character index 3
    assert_eq!(expect_num("index_of", vec![s("café"), s("é")]), 3.0);
}

// ══════════════════════════════════════════════════════════════════════════════
// Unicode / multi-byte edge cases
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_unicode_slice_emoji() {
    // "Hi 😀!" — emoji at char index 3
    assert_eq!(
        expect_str("slice", vec![s("Hi 😀!"), num(3.0), num(4.0)]),
        "😀"
    );
}

#[test]
fn test_unicode_to_upper() {
    assert_eq!(expect_str("to_upper", vec![s("café")]), "CAFÉ");
}

#[test]
fn test_unicode_to_lower() {
    assert_eq!(expect_str("to_lower", vec![s("CAFÉ")]), "café");
}

#[test]
fn test_unicode_contains() {
    assert!(expect_bool("contains", vec![s("日本語"), s("本")]));
}

#[test]
fn test_unicode_split() {
    let result = call_ok("split", vec![s("a·b·c"), s("·")]);
    assert_eq!(result, Value::List(vec![s("a"), s("b"), s("c")]));
}

// ══════════════════════════════════════════════════════════════════════════════
// Type error coverage
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_type_errors() {
    // All functions that expect strings should reject numbers
    let err = call("trim", vec![num(1.0)]).unwrap_err();
    assert!(matches!(err, StdlibError::TypeMismatch { .. }));

    let err = call("to_upper", vec![num(1.0)]).unwrap_err();
    assert!(matches!(err, StdlibError::TypeMismatch { .. }));

    let err = call("to_lower", vec![Value::Bool(true)]).unwrap_err();
    assert!(matches!(err, StdlibError::TypeMismatch { .. }));

    let err = call("is_empty", vec![Value::Nil]).unwrap_err();
    assert!(matches!(err, StdlibError::TypeMismatch { .. }));
}

#[test]
fn test_arg_count_errors() {
    let err = call("length", vec![]).unwrap_err();
    assert!(matches!(err, StdlibError::WrongArgCount { .. }));

    let err = call("concat", vec![s("a")]).unwrap_err();
    assert!(matches!(err, StdlibError::WrongArgCount { .. }));

    let err = call("slice", vec![s("a"), num(0.0)]).unwrap_err();
    assert!(matches!(err, StdlibError::WrongArgCount { .. }));

    let err = call("pad_start", vec![s("a")]).unwrap_err();
    assert!(matches!(err, StdlibError::WrongArgCount { .. }));

    let err = call("join", vec![Value::List(vec![])]).unwrap_err();
    assert!(matches!(err, StdlibError::WrongArgCount { .. }));
}

// ══════════════════════════════════════════════════════════════════════════════
// Determinism proof — 100-iteration test
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_determinism_100_iterations() {
    let m = string_mod();

    let mut fields = BTreeMap::new();
    fields.insert("name".to_string(), s("World"));
    let rec = Value::Record {
        type_name: None,
        fields,
    };
    let items = Value::List(vec![s("a"), s("b"), s("c")]);

    let ref_length = m.call("length", vec![s("hello")]).unwrap();
    let ref_concat = m.call("concat", vec![s("a"), s("b")]).unwrap();
    let ref_contains = m.call("contains", vec![s("abc"), s("b")]).unwrap();
    let ref_slice = m
        .call("slice", vec![s("hello"), num(1.0), num(4.0)])
        .unwrap();
    let ref_trim = m.call("trim", vec![s("  hi  ")]).unwrap();
    let ref_split = m.call("split", vec![s("a,b"), s(",")]).unwrap();
    let ref_upper = m.call("to_upper", vec![s("hello")]).unwrap();
    let ref_lower = m.call("to_lower", vec![s("HELLO")]).unwrap();
    let ref_starts = m.call("starts_with", vec![s("hello"), s("he")]).unwrap();
    let ref_ends = m.call("ends_with", vec![s("hello"), s("lo")]).unwrap();
    let ref_replace = m.call("replace", vec![s("aab"), s("a"), s("x")]).unwrap();
    let ref_replace_all = m
        .call("replace_all", vec![s("aab"), s("a"), s("x")])
        .unwrap();
    let ref_pad_start = m.call("pad_start", vec![s("1"), num(3.0), s("0")]).unwrap();
    let ref_pad_end = m.call("pad_end", vec![s("1"), num(3.0), s("0")]).unwrap();
    let ref_repeat = m.call("repeat", vec![s("ab"), num(2.0)]).unwrap();
    let ref_join = m.call("join", vec![items.clone(), s(",")]).unwrap();
    let ref_format = m.call("format", vec![s("Hi {name}"), rec.clone()]).unwrap();
    let ref_from = m.call("from", vec![num(42.0)]).unwrap();
    let ref_empty = m.call("is_empty", vec![s("")]).unwrap();
    let ref_index = m.call("index_of", vec![s("hello"), s("ll")]).unwrap();

    for i in 0..100 {
        assert_eq!(
            m.call("length", vec![s("hello")]).unwrap(),
            ref_length,
            "length iter {i}"
        );
        assert_eq!(
            m.call("concat", vec![s("a"), s("b")]).unwrap(),
            ref_concat,
            "concat iter {i}"
        );
        assert_eq!(
            m.call("contains", vec![s("abc"), s("b")]).unwrap(),
            ref_contains,
            "contains iter {i}"
        );
        assert_eq!(
            m.call("slice", vec![s("hello"), num(1.0), num(4.0)])
                .unwrap(),
            ref_slice,
            "slice iter {i}"
        );
        assert_eq!(
            m.call("trim", vec![s("  hi  ")]).unwrap(),
            ref_trim,
            "trim iter {i}"
        );
        assert_eq!(
            m.call("split", vec![s("a,b"), s(",")]).unwrap(),
            ref_split,
            "split iter {i}"
        );
        assert_eq!(
            m.call("to_upper", vec![s("hello")]).unwrap(),
            ref_upper,
            "to_upper iter {i}"
        );
        assert_eq!(
            m.call("to_lower", vec![s("HELLO")]).unwrap(),
            ref_lower,
            "to_lower iter {i}"
        );
        assert_eq!(
            m.call("starts_with", vec![s("hello"), s("he")]).unwrap(),
            ref_starts,
            "starts_with iter {i}"
        );
        assert_eq!(
            m.call("ends_with", vec![s("hello"), s("lo")]).unwrap(),
            ref_ends,
            "ends_with iter {i}"
        );
        assert_eq!(
            m.call("replace", vec![s("aab"), s("a"), s("x")]).unwrap(),
            ref_replace,
            "replace iter {i}"
        );
        assert_eq!(
            m.call("replace_all", vec![s("aab"), s("a"), s("x")])
                .unwrap(),
            ref_replace_all,
            "replace_all iter {i}"
        );
        assert_eq!(
            m.call("pad_start", vec![s("1"), num(3.0), s("0")]).unwrap(),
            ref_pad_start,
            "pad_start iter {i}"
        );
        assert_eq!(
            m.call("pad_end", vec![s("1"), num(3.0), s("0")]).unwrap(),
            ref_pad_end,
            "pad_end iter {i}"
        );
        assert_eq!(
            m.call("repeat", vec![s("ab"), num(2.0)]).unwrap(),
            ref_repeat,
            "repeat iter {i}"
        );
        assert_eq!(
            m.call("join", vec![items.clone(), s(",")]).unwrap(),
            ref_join,
            "join iter {i}"
        );
        assert_eq!(
            m.call("format", vec![s("Hi {name}"), rec.clone()]).unwrap(),
            ref_format,
            "format iter {i}"
        );
        assert_eq!(
            m.call("from", vec![num(42.0)]).unwrap(),
            ref_from,
            "from iter {i}"
        );
        assert_eq!(
            m.call("is_empty", vec![s("")]).unwrap(),
            ref_empty,
            "is_empty iter {i}"
        );
        assert_eq!(
            m.call("index_of", vec![s("hello"), s("ll")]).unwrap(),
            ref_index,
            "index_of iter {i}"
        );
    }
}
