//! Tests for the `list` module — 31 functions.
//!
//! Each function gets:
//! - Normal-case tests (1–3)
//! - Edge-case tests (empty list, boundary indices, etc.)
//! - Wrong-type / wrong-arg-count error tests
//!
//! Higher-order functions also test callback behaviour.

use pepl_stdlib::modules::list::ListModule;
use pepl_stdlib::{StdlibError, StdlibFn, StdlibModule, Value};
use std::collections::BTreeMap;

// ── Helpers ───────────────────────────────────────────────────────────────────

fn list() -> ListModule {
    ListModule::new()
}

fn num(n: f64) -> Value {
    Value::Number(n)
}

fn s(val: &str) -> Value {
    Value::String(val.to_string())
}

fn b(val: bool) -> Value {
    Value::Bool(val)
}

fn lst(items: Vec<Value>) -> Value {
    Value::List(items)
}

fn call(func: &str, args: Vec<Value>) -> Result<Value, StdlibError> {
    list().call(func, args)
}

fn call_ok(func: &str, args: Vec<Value>) -> Value {
    call(func, args).unwrap_or_else(|e| panic!("list.{func} failed: {e}"))
}

/// Create a simple predicate function value for testing.
fn pred_fn(f: impl Fn(Vec<Value>) -> Result<Value, StdlibError> + Send + Sync + 'static) -> Value {
    Value::Function(StdlibFn::new(f))
}

/// Predicate: is the value > threshold?
fn gt(threshold: f64) -> Value {
    pred_fn(move |args| {
        let n = args[0].as_number().unwrap();
        Ok(Value::Bool(n > threshold))
    })
}

/// Predicate: is the value even?
fn is_even() -> Value {
    pred_fn(|args| {
        let n = args[0].as_number().unwrap();
        Ok(Value::Bool(n as i64 % 2 == 0))
    })
}

/// Mapper: double the number.
fn double() -> Value {
    pred_fn(|args| {
        let n = args[0].as_number().unwrap();
        Ok(Value::Number(n * 2.0))
    })
}

/// Mapper: convert number to string.
fn to_string_fn() -> Value {
    pred_fn(|args| Ok(Value::String(format!("{}", args[0]))))
}

/// Reducer: sum accumulator.
fn sum_reducer() -> Value {
    pred_fn(|args| {
        let a = args[0].as_number().unwrap();
        let b = args[1].as_number().unwrap();
        Ok(Value::Number(a + b))
    })
}

/// Comparator: ascending numeric.
fn cmp_asc() -> Value {
    pred_fn(|args| {
        let a = args[0].as_number().unwrap();
        let b = args[1].as_number().unwrap();
        Ok(Value::Number(a - b))
    })
}

/// Comparator: descending numeric.
fn cmp_desc() -> Value {
    pred_fn(|args| {
        let a = args[0].as_number().unwrap();
        let b = args[1].as_number().unwrap();
        Ok(Value::Number(b - a))
    })
}

// ═══════════════════════════════════════════════════════════════════════════════
// Construction
// ═══════════════════════════════════════════════════════════════════════════════

// ── list.empty ────────────────────────────────────────────────────────────────

#[test]
fn empty_returns_empty_list() {
    assert_eq!(call_ok("empty", vec![]), lst(vec![]));
}

#[test]
fn empty_wrong_args() {
    assert!(call("empty", vec![num(1.0)]).is_err());
}

// ── list.of ───────────────────────────────────────────────────────────────────

#[test]
fn of_no_args() {
    assert_eq!(call_ok("of", vec![]), lst(vec![]));
}

#[test]
fn of_single() {
    assert_eq!(call_ok("of", vec![num(1.0)]), lst(vec![num(1.0)]));
}

#[test]
fn of_multiple() {
    assert_eq!(
        call_ok("of", vec![num(1.0), s("hello"), b(true)]),
        lst(vec![num(1.0), s("hello"), b(true)])
    );
}

// ── list.repeat ───────────────────────────────────────────────────────────────

#[test]
fn repeat_basic() {
    assert_eq!(
        call_ok("repeat", vec![s("x"), num(3.0)]),
        lst(vec![s("x"), s("x"), s("x")])
    );
}

#[test]
fn repeat_zero() {
    assert_eq!(call_ok("repeat", vec![num(1.0), num(0.0)]), lst(vec![]));
}

#[test]
fn repeat_negative_count() {
    assert!(call("repeat", vec![num(1.0), num(-1.0)]).is_err());
}

#[test]
fn repeat_non_integer_count() {
    assert!(call("repeat", vec![num(1.0), num(2.5)]).is_err());
}

#[test]
fn repeat_wrong_args() {
    assert!(call("repeat", vec![num(1.0)]).is_err());
}

// ── list.range ────────────────────────────────────────────────────────────────

#[test]
fn range_basic() {
    assert_eq!(
        call_ok("range", vec![num(1.0), num(4.0)]),
        lst(vec![num(1.0), num(2.0), num(3.0)])
    );
}

#[test]
fn range_same_start_end() {
    assert_eq!(call_ok("range", vec![num(5.0), num(5.0)]), lst(vec![]));
}

#[test]
fn range_end_less_than_start() {
    assert_eq!(call_ok("range", vec![num(5.0), num(3.0)]), lst(vec![]));
}

#[test]
fn range_negative() {
    assert_eq!(
        call_ok("range", vec![num(-2.0), num(1.0)]),
        lst(vec![num(-2.0), num(-1.0), num(0.0)])
    );
}

#[test]
fn range_non_integer() {
    assert!(call("range", vec![num(1.5), num(3.0)]).is_err());
}

#[test]
fn range_wrong_type() {
    assert!(call("range", vec![s("a"), num(3.0)]).is_err());
}

// ═══════════════════════════════════════════════════════════════════════════════
// Access
// ═══════════════════════════════════════════════════════════════════════════════

// ── list.length ───────────────────────────────────────────────────────────────

#[test]
fn length_empty() {
    assert_eq!(call_ok("length", vec![lst(vec![])]), num(0.0));
}

#[test]
fn length_nonempty() {
    assert_eq!(
        call_ok("length", vec![lst(vec![num(1.0), num(2.0), num(3.0)])]),
        num(3.0)
    );
}

#[test]
fn length_wrong_type() {
    assert!(call("length", vec![num(5.0)]).is_err());
}

// ── list.get ──────────────────────────────────────────────────────────────────

#[test]
fn get_valid_index() {
    let items = lst(vec![s("a"), s("b"), s("c")]);
    assert_eq!(call_ok("get", vec![items, num(1.0)]), s("b"));
}

#[test]
fn get_first_index() {
    let items = lst(vec![num(10.0), num(20.0)]);
    assert_eq!(call_ok("get", vec![items, num(0.0)]), num(10.0));
}

#[test]
fn get_out_of_bounds() {
    let items = lst(vec![num(1.0)]);
    assert_eq!(call_ok("get", vec![items, num(5.0)]), Value::Nil);
}

#[test]
fn get_negative_index() {
    let items = lst(vec![num(1.0)]);
    assert_eq!(call_ok("get", vec![items, num(-1.0)]), Value::Nil);
}

#[test]
fn get_empty_list() {
    assert_eq!(call_ok("get", vec![lst(vec![]), num(0.0)]), Value::Nil);
}

// ── list.first ────────────────────────────────────────────────────────────────

#[test]
fn first_nonempty() {
    assert_eq!(
        call_ok("first", vec![lst(vec![s("a"), s("b")])]),
        s("a")
    );
}

#[test]
fn first_empty() {
    assert_eq!(call_ok("first", vec![lst(vec![])]), Value::Nil);
}

// ── list.last ─────────────────────────────────────────────────────────────────

#[test]
fn last_nonempty() {
    assert_eq!(
        call_ok("last", vec![lst(vec![num(1.0), num(2.0), num(3.0)])]),
        num(3.0)
    );
}

#[test]
fn last_empty() {
    assert_eq!(call_ok("last", vec![lst(vec![])]), Value::Nil);
}

// ── list.index_of ─────────────────────────────────────────────────────────────

#[test]
fn index_of_found() {
    let items = lst(vec![s("a"), s("b"), s("c")]);
    assert_eq!(call_ok("index_of", vec![items, s("b")]), num(1.0));
}

#[test]
fn index_of_not_found() {
    let items = lst(vec![num(1.0), num(2.0)]);
    assert_eq!(call_ok("index_of", vec![items, num(99.0)]), num(-1.0));
}

#[test]
fn index_of_empty() {
    assert_eq!(
        call_ok("index_of", vec![lst(vec![]), num(1.0)]),
        num(-1.0)
    );
}

#[test]
fn index_of_first_occurrence() {
    let items = lst(vec![s("a"), s("b"), s("a")]);
    assert_eq!(call_ok("index_of", vec![items, s("a")]), num(0.0));
}

// ═══════════════════════════════════════════════════════════════════════════════
// Modification
// ═══════════════════════════════════════════════════════════════════════════════

// ── list.append ───────────────────────────────────────────────────────────────

#[test]
fn append_basic() {
    assert_eq!(
        call_ok("append", vec![lst(vec![num(1.0)]), num(2.0)]),
        lst(vec![num(1.0), num(2.0)])
    );
}

#[test]
fn append_to_empty() {
    assert_eq!(
        call_ok("append", vec![lst(vec![]), s("x")]),
        lst(vec![s("x")])
    );
}

// ── list.prepend ──────────────────────────────────────────────────────────────

#[test]
fn prepend_basic() {
    assert_eq!(
        call_ok("prepend", vec![lst(vec![num(2.0)]), num(1.0)]),
        lst(vec![num(1.0), num(2.0)])
    );
}

#[test]
fn prepend_to_empty() {
    assert_eq!(
        call_ok("prepend", vec![lst(vec![]), s("x")]),
        lst(vec![s("x")])
    );
}

// ── list.insert ───────────────────────────────────────────────────────────────

#[test]
fn insert_middle() {
    let items = lst(vec![num(1.0), num(3.0)]);
    assert_eq!(
        call_ok("insert", vec![items, num(1.0), num(2.0)]),
        lst(vec![num(1.0), num(2.0), num(3.0)])
    );
}

#[test]
fn insert_at_start() {
    let items = lst(vec![num(2.0)]);
    assert_eq!(
        call_ok("insert", vec![items, num(0.0), num(1.0)]),
        lst(vec![num(1.0), num(2.0)])
    );
}

#[test]
fn insert_at_end() {
    let items = lst(vec![num(1.0)]);
    assert_eq!(
        call_ok("insert", vec![items, num(1.0), num(2.0)]),
        lst(vec![num(1.0), num(2.0)])
    );
}

#[test]
fn insert_out_of_bounds() {
    let items = lst(vec![num(1.0)]);
    assert!(call("insert", vec![items, num(5.0), num(2.0)]).is_err());
}

// ── list.remove ───────────────────────────────────────────────────────────────

#[test]
fn remove_first() {
    let items = lst(vec![num(1.0), num(2.0), num(3.0)]);
    assert_eq!(
        call_ok("remove", vec![items, num(0.0)]),
        lst(vec![num(2.0), num(3.0)])
    );
}

#[test]
fn remove_last() {
    let items = lst(vec![num(1.0), num(2.0)]);
    assert_eq!(
        call_ok("remove", vec![items, num(1.0)]),
        lst(vec![num(1.0)])
    );
}

#[test]
fn remove_out_of_bounds() {
    let items = lst(vec![num(1.0)]);
    assert!(call("remove", vec![items, num(5.0)]).is_err());
}

#[test]
fn remove_negative() {
    let items = lst(vec![num(1.0)]);
    assert!(call("remove", vec![items, num(-1.0)]).is_err());
}

// ── list.update ───────────────────────────────────────────────────────────────

#[test]
fn update_basic() {
    let items = lst(vec![num(1.0), num(2.0), num(3.0)]);
    assert_eq!(
        call_ok("update", vec![items, num(1.0), num(99.0)]),
        lst(vec![num(1.0), num(99.0), num(3.0)])
    );
}

#[test]
fn update_out_of_bounds() {
    let items = lst(vec![num(1.0)]);
    assert!(call("update", vec![items, num(5.0), num(2.0)]).is_err());
}

// ── list.slice ────────────────────────────────────────────────────────────────

#[test]
fn slice_basic() {
    let items = lst(vec![num(1.0), num(2.0), num(3.0), num(4.0)]);
    assert_eq!(
        call_ok("slice", vec![items, num(1.0), num(3.0)]),
        lst(vec![num(2.0), num(3.0)])
    );
}

#[test]
fn slice_full() {
    let items = lst(vec![num(1.0), num(2.0)]);
    assert_eq!(
        call_ok("slice", vec![items.clone(), num(0.0), num(2.0)]),
        items
    );
}

#[test]
fn slice_empty_range() {
    let items = lst(vec![num(1.0), num(2.0)]);
    assert_eq!(
        call_ok("slice", vec![items, num(1.0), num(1.0)]),
        lst(vec![])
    );
}

#[test]
fn slice_clamped() {
    let items = lst(vec![num(1.0), num(2.0)]);
    assert_eq!(
        call_ok("slice", vec![items, num(-5.0), num(100.0)]),
        lst(vec![num(1.0), num(2.0)])
    );
}

// ── list.concat ───────────────────────────────────────────────────────────────

#[test]
fn concat_basic() {
    assert_eq!(
        call_ok(
            "concat",
            vec![lst(vec![num(1.0)]), lst(vec![num(2.0), num(3.0)])]
        ),
        lst(vec![num(1.0), num(2.0), num(3.0)])
    );
}

#[test]
fn concat_with_empty() {
    assert_eq!(
        call_ok("concat", vec![lst(vec![num(1.0)]), lst(vec![])]),
        lst(vec![num(1.0)])
    );
}

#[test]
fn concat_both_empty() {
    assert_eq!(
        call_ok("concat", vec![lst(vec![]), lst(vec![])]),
        lst(vec![])
    );
}

#[test]
fn concat_wrong_type() {
    assert!(call("concat", vec![lst(vec![]), num(1.0)]).is_err());
}

// ── list.reverse ──────────────────────────────────────────────────────────────

#[test]
fn reverse_basic() {
    assert_eq!(
        call_ok("reverse", vec![lst(vec![num(1.0), num(2.0), num(3.0)])]),
        lst(vec![num(3.0), num(2.0), num(1.0)])
    );
}

#[test]
fn reverse_empty() {
    assert_eq!(call_ok("reverse", vec![lst(vec![])]), lst(vec![]));
}

#[test]
fn reverse_single() {
    assert_eq!(
        call_ok("reverse", vec![lst(vec![num(1.0)])]),
        lst(vec![num(1.0)])
    );
}

// ── list.flatten ──────────────────────────────────────────────────────────────

#[test]
fn flatten_basic() {
    let nested = lst(vec![
        lst(vec![num(1.0), num(2.0)]),
        lst(vec![num(3.0)]),
        num(4.0),
    ]);
    assert_eq!(
        call_ok("flatten", vec![nested]),
        lst(vec![num(1.0), num(2.0), num(3.0), num(4.0)])
    );
}

#[test]
fn flatten_already_flat() {
    let flat = lst(vec![num(1.0), num(2.0)]);
    assert_eq!(
        call_ok("flatten", vec![flat.clone()]),
        flat
    );
}

#[test]
fn flatten_empty() {
    assert_eq!(call_ok("flatten", vec![lst(vec![])]), lst(vec![]));
}

#[test]
fn flatten_only_one_level() {
    // [[[ 1 ]]] -> [[ 1 ]] (only one level flattened)
    let deeply_nested = lst(vec![lst(vec![lst(vec![num(1.0)])])]);
    assert_eq!(
        call_ok("flatten", vec![deeply_nested]),
        lst(vec![lst(vec![num(1.0)])])
    );
}

// ── list.unique ───────────────────────────────────────────────────────────────

#[test]
fn unique_basic() {
    let items = lst(vec![num(1.0), num(2.0), num(1.0), num(3.0), num(2.0)]);
    assert_eq!(
        call_ok("unique", vec![items]),
        lst(vec![num(1.0), num(2.0), num(3.0)])
    );
}

#[test]
fn unique_no_duplicates() {
    let items = lst(vec![num(1.0), num(2.0), num(3.0)]);
    assert_eq!(
        call_ok("unique", vec![items.clone()]),
        items
    );
}

#[test]
fn unique_empty() {
    assert_eq!(call_ok("unique", vec![lst(vec![])]), lst(vec![]));
}

#[test]
fn unique_preserves_order() {
    let items = lst(vec![s("c"), s("a"), s("b"), s("a"), s("c")]);
    assert_eq!(
        call_ok("unique", vec![items]),
        lst(vec![s("c"), s("a"), s("b")])
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// Higher-Order
// ═══════════════════════════════════════════════════════════════════════════════

// ── list.map ──────────────────────────────────────────────────────────────────

#[test]
fn map_double() {
    let items = lst(vec![num(1.0), num(2.0), num(3.0)]);
    assert_eq!(
        call_ok("map", vec![items, double()]),
        lst(vec![num(2.0), num(4.0), num(6.0)])
    );
}

#[test]
fn map_to_string() {
    let items = lst(vec![num(1.0), num(2.0)]);
    assert_eq!(
        call_ok("map", vec![items, to_string_fn()]),
        lst(vec![s("1"), s("2")])
    );
}

#[test]
fn map_empty() {
    assert_eq!(
        call_ok("map", vec![lst(vec![]), double()]),
        lst(vec![])
    );
}

#[test]
fn map_wrong_type_for_function() {
    let items = lst(vec![num(1.0)]);
    assert!(call("map", vec![items, num(1.0)]).is_err());
}

// ── list.filter ───────────────────────────────────────────────────────────────

#[test]
fn filter_even() {
    let items = lst(vec![num(1.0), num(2.0), num(3.0), num(4.0)]);
    assert_eq!(
        call_ok("filter", vec![items, is_even()]),
        lst(vec![num(2.0), num(4.0)])
    );
}

#[test]
fn filter_none_match() {
    let items = lst(vec![num(1.0), num(3.0), num(5.0)]);
    assert_eq!(
        call_ok("filter", vec![items, is_even()]),
        lst(vec![])
    );
}

#[test]
fn filter_all_match() {
    let items = lst(vec![num(2.0), num(4.0)]);
    assert_eq!(
        call_ok("filter", vec![items.clone(), is_even()]),
        items
    );
}

#[test]
fn filter_empty() {
    assert_eq!(
        call_ok("filter", vec![lst(vec![]), is_even()]),
        lst(vec![])
    );
}

// ── list.reduce ───────────────────────────────────────────────────────────────

#[test]
fn reduce_sum() {
    let items = lst(vec![num(1.0), num(2.0), num(3.0)]);
    assert_eq!(
        call_ok("reduce", vec![items, num(0.0), sum_reducer()]),
        num(6.0)
    );
}

#[test]
fn reduce_with_initial() {
    let items = lst(vec![num(1.0), num(2.0)]);
    assert_eq!(
        call_ok("reduce", vec![items, num(10.0), sum_reducer()]),
        num(13.0)
    );
}

#[test]
fn reduce_empty() {
    assert_eq!(
        call_ok("reduce", vec![lst(vec![]), num(42.0), sum_reducer()]),
        num(42.0) // returns initial value
    );
}

#[test]
fn reduce_string_concat() {
    let concat_fn = pred_fn(|args| {
        let a = args[0].as_str().unwrap().to_string();
        let b = args[1].as_str().unwrap().to_string();
        Ok(Value::String(format!("{a}{b}")))
    });
    let items = lst(vec![s("a"), s("b"), s("c")]);
    assert_eq!(
        call_ok("reduce", vec![items, s(""), concat_fn]),
        s("abc")
    );
}

// ── list.find ─────────────────────────────────────────────────────────────────

#[test]
fn find_found() {
    let items = lst(vec![num(1.0), num(2.0), num(3.0)]);
    assert_eq!(call_ok("find", vec![items, gt(1.5)]), num(2.0));
}

#[test]
fn find_not_found() {
    let items = lst(vec![num(1.0), num(2.0)]);
    assert_eq!(call_ok("find", vec![items, gt(10.0)]), Value::Nil);
}

#[test]
fn find_empty() {
    assert_eq!(call_ok("find", vec![lst(vec![]), gt(0.0)]), Value::Nil);
}

// ── list.find_index ───────────────────────────────────────────────────────────

#[test]
fn find_index_found() {
    let items = lst(vec![num(1.0), num(2.0), num(3.0)]);
    assert_eq!(call_ok("find_index", vec![items, gt(1.5)]), num(1.0));
}

#[test]
fn find_index_not_found() {
    let items = lst(vec![num(1.0)]);
    assert_eq!(call_ok("find_index", vec![items, gt(10.0)]), num(-1.0));
}

// ── list.every ────────────────────────────────────────────────────────────────

#[test]
fn every_all_match() {
    let items = lst(vec![num(2.0), num(4.0), num(6.0)]);
    assert_eq!(call_ok("every", vec![items, is_even()]), b(true));
}

#[test]
fn every_some_dont() {
    let items = lst(vec![num(2.0), num(3.0), num(4.0)]);
    assert_eq!(call_ok("every", vec![items, is_even()]), b(false));
}

#[test]
fn every_empty() {
    // vacuously true
    assert_eq!(call_ok("every", vec![lst(vec![]), is_even()]), b(true));
}

// ── list.some ─────────────────────────────────────────────────────────────────

#[test]
fn some_one_matches() {
    let items = lst(vec![num(1.0), num(2.0), num(3.0)]);
    assert_eq!(call_ok("some", vec![items, is_even()]), b(true));
}

#[test]
fn some_none_match() {
    let items = lst(vec![num(1.0), num(3.0), num(5.0)]);
    assert_eq!(call_ok("some", vec![items, is_even()]), b(false));
}

#[test]
fn some_empty() {
    assert_eq!(call_ok("some", vec![lst(vec![]), is_even()]), b(false));
}

// ── list.sort ─────────────────────────────────────────────────────────────────

#[test]
fn sort_ascending() {
    let items = lst(vec![num(3.0), num(1.0), num(2.0)]);
    assert_eq!(
        call_ok("sort", vec![items, cmp_asc()]),
        lst(vec![num(1.0), num(2.0), num(3.0)])
    );
}

#[test]
fn sort_descending() {
    let items = lst(vec![num(1.0), num(3.0), num(2.0)]);
    assert_eq!(
        call_ok("sort", vec![items, cmp_desc()]),
        lst(vec![num(3.0), num(2.0), num(1.0)])
    );
}

#[test]
fn sort_already_sorted() {
    let items = lst(vec![num(1.0), num(2.0), num(3.0)]);
    assert_eq!(
        call_ok("sort", vec![items.clone(), cmp_asc()]),
        items
    );
}

#[test]
fn sort_empty() {
    assert_eq!(
        call_ok("sort", vec![lst(vec![]), cmp_asc()]),
        lst(vec![])
    );
}

#[test]
fn sort_single() {
    assert_eq!(
        call_ok("sort", vec![lst(vec![num(1.0)]), cmp_asc()]),
        lst(vec![num(1.0)])
    );
}

#[test]
fn sort_comparator_error() {
    let bad_cmp = pred_fn(|_| Err(StdlibError::RuntimeError("boom".to_string())));
    let items = lst(vec![num(2.0), num(1.0)]);
    assert!(call("sort", vec![items, bad_cmp]).is_err());
}

// ── list.count ────────────────────────────────────────────────────────────────

#[test]
fn count_basic() {
    let items = lst(vec![num(1.0), num(2.0), num(3.0), num(4.0)]);
    assert_eq!(call_ok("count", vec![items, is_even()]), num(2.0));
}

#[test]
fn count_none() {
    let items = lst(vec![num(1.0), num(3.0)]);
    assert_eq!(call_ok("count", vec![items, is_even()]), num(0.0));
}

#[test]
fn count_empty() {
    assert_eq!(call_ok("count", vec![lst(vec![]), is_even()]), num(0.0));
}

// ═══════════════════════════════════════════════════════════════════════════════
// Query
// ═══════════════════════════════════════════════════════════════════════════════

// ── list.contains ─────────────────────────────────────────────────────────────

#[test]
fn contains_found() {
    let items = lst(vec![num(1.0), num(2.0), num(3.0)]);
    assert_eq!(call_ok("contains", vec![items, num(2.0)]), b(true));
}

#[test]
fn contains_not_found() {
    let items = lst(vec![num(1.0), num(2.0)]);
    assert_eq!(call_ok("contains", vec![items, num(99.0)]), b(false));
}

#[test]
fn contains_empty() {
    assert_eq!(
        call_ok("contains", vec![lst(vec![]), num(1.0)]),
        b(false)
    );
}

#[test]
fn contains_string() {
    let items = lst(vec![s("hello"), s("world")]);
    assert_eq!(call_ok("contains", vec![items, s("world")]), b(true));
}

// ── list.zip ──────────────────────────────────────────────────────────────────

#[test]
fn zip_same_length() {
    let a = lst(vec![num(1.0), num(2.0)]);
    let b_list = lst(vec![s("a"), s("b")]);
    let result = call_ok("zip", vec![a, b_list]);
    if let Value::List(items) = &result {
        assert_eq!(items.len(), 2);
        // Each item is a { first, second } record
        let mut expected1 = BTreeMap::new();
        expected1.insert("first".to_string(), num(1.0));
        expected1.insert("second".to_string(), s("a"));
        assert_eq!(items[0], Value::record(expected1));

        let mut expected2 = BTreeMap::new();
        expected2.insert("first".to_string(), num(2.0));
        expected2.insert("second".to_string(), s("b"));
        assert_eq!(items[1], Value::record(expected2));
    } else {
        panic!("expected list, got {result:?}");
    }
}

#[test]
fn zip_different_lengths() {
    let a = lst(vec![num(1.0), num(2.0), num(3.0)]);
    let b_list = lst(vec![s("x")]);
    let result = call_ok("zip", vec![a, b_list]);
    if let Value::List(items) = &result {
        assert_eq!(items.len(), 1); // shorter list wins
    } else {
        panic!("expected list");
    }
}

#[test]
fn zip_empty() {
    assert_eq!(
        call_ok("zip", vec![lst(vec![]), lst(vec![num(1.0)])]),
        lst(vec![])
    );
}

#[test]
fn zip_wrong_type() {
    assert!(call("zip", vec![lst(vec![]), num(1.0)]).is_err());
}

// ── list.take ─────────────────────────────────────────────────────────────────

#[test]
fn take_basic() {
    let items = lst(vec![num(1.0), num(2.0), num(3.0)]);
    assert_eq!(
        call_ok("take", vec![items, num(2.0)]),
        lst(vec![num(1.0), num(2.0)])
    );
}

#[test]
fn take_more_than_length() {
    let items = lst(vec![num(1.0)]);
    assert_eq!(
        call_ok("take", vec![items, num(100.0)]),
        lst(vec![num(1.0)])
    );
}

#[test]
fn take_zero() {
    let items = lst(vec![num(1.0), num(2.0)]);
    assert_eq!(call_ok("take", vec![items, num(0.0)]), lst(vec![]));
}

#[test]
fn take_negative() {
    let items = lst(vec![num(1.0)]);
    assert!(call("take", vec![items, num(-1.0)]).is_err());
}

// ═══════════════════════════════════════════════════════════════════════════════
// Module trait
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn module_name() {
    assert_eq!(list().name(), "list");
}

#[test]
fn has_all_31_functions() {
    let m = list();
    let functions = [
        "empty", "of", "repeat", "range",
        "length", "get", "first", "last", "index_of",
        "append", "prepend", "insert", "remove", "update",
        "slice", "concat", "reverse", "flatten", "unique",
        "map", "filter", "reduce", "find", "find_index",
        "every", "some", "sort", "count",
        "contains", "zip", "take",
    ];
    for f in &functions {
        assert!(m.has_function(f), "missing function: {f}");
    }
    assert_eq!(functions.len(), 31);
}

#[test]
fn unknown_function() {
    assert!(call("nonexistent", vec![]).is_err());
}

// ═══════════════════════════════════════════════════════════════════════════════
// Integration / Chaining
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn chain_filter_then_map() {
    // list.filter([1,2,3,4], is_even) then list.map(result, double)
    let items = lst(vec![num(1.0), num(2.0), num(3.0), num(4.0)]);
    let filtered = call_ok("filter", vec![items, is_even()]);
    let mapped = call_ok("map", vec![filtered, double()]);
    assert_eq!(mapped, lst(vec![num(4.0), num(8.0)]));
}

#[test]
fn chain_range_then_reduce() {
    // list.range(1, 6) then list.reduce(result, 0, sum)
    let range = call_ok("range", vec![num(1.0), num(6.0)]);
    let sum = call_ok("reduce", vec![range, num(0.0), sum_reducer()]);
    assert_eq!(sum, num(15.0)); // 1+2+3+4+5
}

#[test]
fn chain_concat_sort_take() {
    let a = lst(vec![num(5.0), num(1.0)]);
    let b_list = lst(vec![num(3.0), num(2.0)]);
    let concatenated = call_ok("concat", vec![a, b_list]);
    let sorted = call_ok("sort", vec![concatenated, cmp_asc()]);
    let taken = call_ok("take", vec![sorted, num(3.0)]);
    assert_eq!(taken, lst(vec![num(1.0), num(2.0), num(3.0)]));
}

#[test]
fn chain_repeat_flatten_unique() {
    let inner = lst(vec![num(1.0), num(2.0)]);
    let repeated = call_ok("repeat", vec![inner, num(3.0)]);
    let flat = call_ok("flatten", vec![repeated]);
    let uniq = call_ok("unique", vec![flat]);
    assert_eq!(uniq, lst(vec![num(1.0), num(2.0)]));
}

// ═══════════════════════════════════════════════════════════════════════════════
// Determinism (100 iterations)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn determinism_100_iterations() {
    let m = list();
    let items = lst(vec![num(3.0), num(1.0), num(4.0), num(1.0), num(5.0)]);

    for _ in 0..100 {
        assert_eq!(
            m.call("sort", vec![items.clone(), cmp_asc()]).unwrap(),
            lst(vec![num(1.0), num(1.0), num(3.0), num(4.0), num(5.0)])
        );
        assert_eq!(
            m.call("unique", vec![items.clone()]).unwrap(),
            lst(vec![num(3.0), num(1.0), num(4.0), num(5.0)])
        );
        assert_eq!(
            m.call("reverse", vec![items.clone()]).unwrap(),
            lst(vec![num(5.0), num(1.0), num(4.0), num(1.0), num(3.0)])
        );
        assert_eq!(
            m.call("filter", vec![items.clone(), gt(2.5)]).unwrap(),
            lst(vec![num(3.0), num(4.0), num(5.0)])
        );
    }
}
