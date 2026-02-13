//! The `list` module — 32 functions (31 spec + 5 extensions + `some` alias).
//!
//! All operations are **immutable** — they return new lists, never mutate.
//!
//! ## Construction (4)
//! | Function       | Signature                                    |
//! |----------------|----------------------------------------------|
//! | `list.empty`   | `() -> list`                                 |
//! | `list.of`      | `(...items) -> list` (variadic)               |
//! | `list.repeat`  | `(value, count: number) -> list`             |
//! | `list.range`   | `(start: number, end: number) -> list`       |
//!
//! ## Access (5)
//! | Function         | Signature                                  |
//! |------------------|--------------------------------------------|
//! | `list.length`    | `(items: list) -> number`                  |
//! | `list.get`       | `(items: list, index: number) -> any\|nil` |
//! | `list.first`     | `(items: list) -> any\|nil`                |
//! | `list.last`      | `(items: list) -> any\|nil`                |
//! | `list.index_of`  | `(items: list, value) -> number`           |
//!
//! ## Modification (10)
//! | Function         | Signature                                            |
//! |------------------|------------------------------------------------------|
//! | `list.append`    | `(items: list, value) -> list`                       |
//! | `list.prepend`   | `(items: list, value) -> list`                       |
//! | `list.insert`    | `(items: list, index: number, value) -> list`        |
//! | `list.remove`    | `(items: list, index: number) -> list`               |
//! | `list.update`    | `(items: list, index: number, value) -> list`        |
//! | `list.slice`     | `(items: list, start: number, end: number) -> list`  |
//! | `list.concat`    | `(a: list, b: list) -> list`                         |
//! | `list.reverse`   | `(items: list) -> list`                              |
//! | `list.flatten`   | `(items: list) -> list`                              |
//! | `list.unique`    | `(items: list) -> list`                              |
//!
//! ## Higher-Order (9)
//! | Function           | Signature                                               |
//! |--------------------|---------------------------------------------------------|
//! | `list.map`         | `(items: list, f: fn(any) -> any) -> list`              |
//! | `list.filter`      | `(items: list, pred: fn(any) -> bool) -> list`          |
//! | `list.reduce`      | `(items: list, init, f: fn(acc, item) -> acc) -> any`   |
//! | `list.find`        | `(items: list, pred: fn(any) -> bool) -> any\|nil`      |
//! | `list.find_index`  | `(items: list, pred: fn(any) -> bool) -> number`        |
//! | `list.every`       | `(items: list, pred: fn(any) -> bool) -> bool`          |
//! | `list.any`         | `(items: list, pred: fn(any) -> bool) -> bool`          |
//! | `list.sort`        | `(items: list, cmp: fn(a, b) -> number) -> list`        |
//! | `list.count`       | `(items: list, pred: fn(any) -> bool) -> number`        |
//!
//! ## Query (4) — also non-higher-order
//! | Function         | Signature                                  |
//! |------------------|--------------------------------------------|
//! | `list.contains`  | `(items: list, value) -> bool`             |
//! | `list.zip`       | `(a: list, b: list) -> list`               |
//! | `list.take`      | `(items: list, n: number) -> list`         |
//! | `list.drop`      | `(items: list, n: number) -> list`         |

use crate::error::StdlibError;
use crate::module::StdlibModule;
use crate::value::Value;

/// The `list` stdlib module.
pub struct ListModule;

impl ListModule {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ListModule {
    fn default() -> Self {
        Self::new()
    }
}

impl StdlibModule for ListModule {
    fn name(&self) -> &'static str {
        "list"
    }

    fn has_function(&self, function: &str) -> bool {
        matches!(
            function,
            // Construction
            "empty" | "of" | "repeat" | "range"
            // Access
            | "length" | "get" | "first" | "last" | "index_of"
            // Modification
            | "append" | "prepend" | "insert" | "remove" | "update" | "set"
            | "slice" | "concat" | "reverse" | "flatten" | "unique"
            // Higher-order
            | "map" | "filter" | "reduce" | "find" | "find_index"
            | "every" | "any" | "some" | "sort" | "count"
            // Query
            | "contains" | "zip" | "take" | "drop"
        )
    }

    fn call(&self, function: &str, args: Vec<Value>) -> Result<Value, StdlibError> {
        match function {
            // Construction
            "empty" => self.empty(args),
            "of" => self.of(args),
            "repeat" => self.repeat(args),
            "range" => self.range(args),
            // Access
            "length" => self.length(args),
            "get" => self.get(args),
            "first" => self.first(args),
            "last" => self.last(args),
            "index_of" => self.index_of(args),
            // Modification
            "append" => self.append(args),
            "prepend" => self.prepend(args),
            "insert" => self.insert(args),
            "remove" => self.remove(args),
            "update" | "set" => self.update(args),
            "slice" => self.slice(args),
            "concat" => self.concat(args),
            "reverse" => self.reverse(args),
            "flatten" => self.flatten(args),
            "unique" => self.unique(args),
            // Higher-order
            "map" => self.map(args),
            "filter" => self.filter(args),
            "reduce" => self.reduce(args),
            "find" => self.find(args),
            "find_index" => self.find_index(args),
            "every" => self.every(args),
            "any" | "some" => self.any(args),
            "sort" => self.sort(args),
            "count" => self.count(args),
            // Query
            "contains" => self.contains(args),
            "zip" => self.zip(args),
            "take" => self.take(args),
            "drop" => self.drop_fn(args),
            _ => Err(StdlibError::unknown_function("list", function)),
        }
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Extract a single list argument.
fn expect_list(fn_name: &str, args: &[Value]) -> Result<Vec<Value>, StdlibError> {
    if args.len() != 1 {
        return Err(StdlibError::wrong_args(fn_name, 1, args.len()));
    }
    match &args[0] {
        Value::List(items) => Ok(items.clone()),
        other => Err(StdlibError::type_mismatch(fn_name, 1, "list", other.type_name())),
    }
}

/// Extract a list from the first argument (multi-arg functions).
fn extract_list(fn_name: &str, val: &Value) -> Result<Vec<Value>, StdlibError> {
    match val {
        Value::List(items) => Ok(items.clone()),
        other => Err(StdlibError::type_mismatch(fn_name, 1, "list", other.type_name())),
    }
}

/// Extract an integer index from a Value, checking it's a whole number.
fn extract_index(fn_name: &str, val: &Value, position: usize) -> Result<i64, StdlibError> {
    match val {
        Value::Number(n) => {
            if n.fract() != 0.0 || !n.is_finite() {
                return Err(StdlibError::RuntimeError(format!(
                    "{fn_name}: index must be a whole number, got {n}"
                )));
            }
            Ok(*n as i64)
        }
        other => Err(StdlibError::type_mismatch(fn_name, position, "number", other.type_name())),
    }
}

/// Extract a number argument at a given position.
fn extract_number(fn_name: &str, val: &Value, position: usize) -> Result<f64, StdlibError> {
    match val {
        Value::Number(n) => Ok(*n),
        other => Err(StdlibError::type_mismatch(fn_name, position, "number", other.type_name())),
    }
}

/// Extract a function argument at a given position.
fn extract_function(
    fn_name: &str,
    val: &Value,
    position: usize,
) -> Result<crate::value::StdlibFn, StdlibError> {
    match val {
        Value::Function(f) => Ok(f.clone()),
        other => Err(StdlibError::type_mismatch(
            fn_name,
            position,
            "function",
            other.type_name(),
        )),
    }
}

// ── Construction ──────────────────────────────────────────────────────────────

impl ListModule {
    /// `list.empty() -> list` — returns an empty list.
    fn empty(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if !args.is_empty() {
            return Err(StdlibError::wrong_args("list.empty", 0, args.len()));
        }
        Ok(Value::List(vec![]))
    }

    /// `list.of(...items) -> list` — creates a list from all arguments (variadic).
    fn of(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        Ok(Value::List(args))
    }

    /// `list.repeat(value, count) -> list` — creates a list of `count` copies of `value`.
    fn repeat(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 2 {
            return Err(StdlibError::wrong_args("list.repeat", 2, args.len()));
        }
        let count = extract_number("list.repeat", &args[1], 2)?;
        if count.fract() != 0.0 || !count.is_finite() || count < 0.0 {
            return Err(StdlibError::RuntimeError(
                "list.repeat: count must be a non-negative integer".to_string(),
            ));
        }
        let count = count as usize;
        let item = args[0].clone();
        Ok(Value::List(vec![item; count]))
    }

    /// `list.range(start, end) -> list<number>` — start inclusive, end exclusive.
    fn range(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 2 {
            return Err(StdlibError::wrong_args("list.range", 2, args.len()));
        }
        let start = extract_number("list.range", &args[0], 1)?;
        let end = extract_number("list.range", &args[1], 2)?;
        if start.fract() != 0.0 || end.fract() != 0.0 || !start.is_finite() || !end.is_finite() {
            return Err(StdlibError::RuntimeError(
                "list.range: start and end must be integers".to_string(),
            ));
        }
        let start = start as i64;
        let end = end as i64;
        if end < start {
            return Ok(Value::List(vec![]));
        }
        // Safety limit: prevent absurdly large ranges
        let len = (end - start) as usize;
        if len > 10_000_000 {
            return Err(StdlibError::RuntimeError(
                "list.range: range too large (max 10,000,000 elements)".to_string(),
            ));
        }
        let items: Vec<Value> = (start..end).map(|i| Value::Number(i as f64)).collect();
        Ok(Value::List(items))
    }

    // ── Access ────────────────────────────────────────────────────────────────

    /// `list.length(items) -> number`
    fn length(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        let items = expect_list("list.length", &args)?;
        Ok(Value::Number(items.len() as f64))
    }

    /// `list.get(items, index) -> any|nil` — returns nil on out-of-bounds.
    fn get(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 2 {
            return Err(StdlibError::wrong_args("list.get", 2, args.len()));
        }
        let items = extract_list("list.get", &args[0])?;
        let index = extract_index("list.get", &args[1], 2)?;
        if index < 0 || index as usize >= items.len() {
            Ok(Value::Nil)
        } else {
            Ok(items[index as usize].clone())
        }
    }

    /// `list.first(items) -> any|nil` — returns nil for empty list.
    fn first(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        let items = expect_list("list.first", &args)?;
        Ok(items.first().cloned().unwrap_or(Value::Nil))
    }

    /// `list.last(items) -> any|nil` — returns nil for empty list.
    fn last(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        let items = expect_list("list.last", &args)?;
        Ok(items.last().cloned().unwrap_or(Value::Nil))
    }

    /// `list.index_of(items, value) -> number` — returns -1 if not found.
    fn index_of(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 2 {
            return Err(StdlibError::wrong_args("list.index_of", 2, args.len()));
        }
        let items = extract_list("list.index_of", &args[0])?;
        let needle = &args[1];
        let index = items
            .iter()
            .position(|v| v == needle)
            .map(|i| i as f64)
            .unwrap_or(-1.0);
        Ok(Value::Number(index))
    }

    // ── Modification ──────────────────────────────────────────────────────────

    /// `list.append(items, value) -> list` — adds to end.
    fn append(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 2 {
            return Err(StdlibError::wrong_args("list.append", 2, args.len()));
        }
        let mut items = extract_list("list.append", &args[0])?;
        items.push(args[1].clone());
        Ok(Value::List(items))
    }

    /// `list.prepend(items, value) -> list` — adds to start.
    fn prepend(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 2 {
            return Err(StdlibError::wrong_args("list.prepend", 2, args.len()));
        }
        let mut items = extract_list("list.prepend", &args[0])?;
        items.insert(0, args[1].clone());
        Ok(Value::List(items))
    }

    /// `list.insert(items, index, value) -> list` — inserts at index.
    fn insert(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 3 {
            return Err(StdlibError::wrong_args("list.insert", 3, args.len()));
        }
        let mut items = extract_list("list.insert", &args[0])?;
        let index = extract_index("list.insert", &args[1], 2)?;
        if index < 0 || index as usize > items.len() {
            return Err(StdlibError::RuntimeError(format!(
                "list.insert: index {} out of bounds for list of length {}",
                index,
                items.len()
            )));
        }
        items.insert(index as usize, args[2].clone());
        Ok(Value::List(items))
    }

    /// `list.remove(items, index) -> list` — removes element at index.
    fn remove(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 2 {
            return Err(StdlibError::wrong_args("list.remove", 2, args.len()));
        }
        let mut items = extract_list("list.remove", &args[0])?;
        let index = extract_index("list.remove", &args[1], 2)?;
        if index < 0 || index as usize >= items.len() {
            return Err(StdlibError::RuntimeError(format!(
                "list.remove: index {} out of bounds for list of length {}",
                index,
                items.len()
            )));
        }
        items.remove(index as usize);
        Ok(Value::List(items))
    }

    /// `list.update(items, index, value) -> list` — replaces element at index.
    fn update(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 3 {
            return Err(StdlibError::wrong_args("list.update", 3, args.len()));
        }
        let mut items = extract_list("list.update", &args[0])?;
        let index = extract_index("list.update", &args[1], 2)?;
        if index < 0 || index as usize >= items.len() {
            return Err(StdlibError::RuntimeError(format!(
                "list.update: index {} out of bounds for list of length {}",
                index,
                items.len()
            )));
        }
        items[index as usize] = args[2].clone();
        Ok(Value::List(items))
    }

    /// `list.slice(items, start, end) -> list` — start inclusive, end exclusive.
    fn slice(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 3 {
            return Err(StdlibError::wrong_args("list.slice", 3, args.len()));
        }
        let items = extract_list("list.slice", &args[0])?;
        let start = extract_index("list.slice", &args[1], 2)?;
        let end = extract_index("list.slice", &args[2], 3)?;
        let len = items.len() as i64;
        // Clamp to bounds
        let start = start.clamp(0, len) as usize;
        let end = end.clamp(0, len) as usize;
        if start >= end {
            return Ok(Value::List(vec![]));
        }
        Ok(Value::List(items[start..end].to_vec()))
    }

    /// `list.concat(a, b) -> list` — concatenates two lists.
    fn concat(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 2 {
            return Err(StdlibError::wrong_args("list.concat", 2, args.len()));
        }
        let mut a = extract_list("list.concat", &args[0])?;
        let b = match &args[1] {
            Value::List(items) => items.clone(),
            other => {
                return Err(StdlibError::type_mismatch(
                    "list.concat",
                    2,
                    "list",
                    other.type_name(),
                ))
            }
        };
        a.extend(b);
        Ok(Value::List(a))
    }

    /// `list.reverse(items) -> list`
    fn reverse(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        let mut items = expect_list("list.reverse", &args)?;
        items.reverse();
        Ok(Value::List(items))
    }

    /// `list.flatten(items) -> list` — flattens one level of nesting.
    fn flatten(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        let items = expect_list("list.flatten", &args)?;
        let mut result = Vec::new();
        for item in items {
            match item {
                Value::List(inner) => result.extend(inner),
                other => result.push(other),
            }
        }
        Ok(Value::List(result))
    }

    /// `list.unique(items) -> list` — removes duplicates, preserving first occurrence.
    fn unique(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        let items = expect_list("list.unique", &args)?;
        let mut seen = Vec::new();
        let mut result = Vec::new();
        for item in items {
            if !seen.contains(&item) {
                seen.push(item.clone());
                result.push(item);
            }
        }
        Ok(Value::List(result))
    }

    // ── Higher-Order ──────────────────────────────────────────────────────────

    /// `list.map(items, f) -> list` — applies f to each element.
    fn map(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 2 {
            return Err(StdlibError::wrong_args("list.map", 2, args.len()));
        }
        let items = extract_list("list.map", &args[0])?;
        let f = extract_function("list.map", &args[1], 2)?;
        let mut result = Vec::with_capacity(items.len());
        for item in items {
            result.push(f.call(vec![item])?);
        }
        Ok(Value::List(result))
    }

    /// `list.filter(items, predicate) -> list`
    fn filter(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 2 {
            return Err(StdlibError::wrong_args("list.filter", 2, args.len()));
        }
        let items = extract_list("list.filter", &args[0])?;
        let pred = extract_function("list.filter", &args[1], 2)?;
        let mut result = Vec::new();
        for item in items {
            let keep = pred.call(vec![item.clone()])?;
            if keep.is_truthy() {
                result.push(item);
            }
        }
        Ok(Value::List(result))
    }

    /// `list.reduce(items, initial, f) -> any`
    fn reduce(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 3 {
            return Err(StdlibError::wrong_args("list.reduce", 3, args.len()));
        }
        let items = extract_list("list.reduce", &args[0])?;
        let mut acc = args[1].clone();
        let f = extract_function("list.reduce", &args[2], 3)?;
        for item in items {
            acc = f.call(vec![acc, item])?;
        }
        Ok(acc)
    }

    /// `list.find(items, predicate) -> any|nil` — returns first match or nil.
    fn find(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 2 {
            return Err(StdlibError::wrong_args("list.find", 2, args.len()));
        }
        let items = extract_list("list.find", &args[0])?;
        let pred = extract_function("list.find", &args[1], 2)?;
        for item in items {
            let matches = pred.call(vec![item.clone()])?;
            if matches.is_truthy() {
                return Ok(item);
            }
        }
        Ok(Value::Nil)
    }

    /// `list.find_index(items, predicate) -> number` — returns -1 if not found.
    fn find_index(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 2 {
            return Err(StdlibError::wrong_args("list.find_index", 2, args.len()));
        }
        let items = extract_list("list.find_index", &args[0])?;
        let pred = extract_function("list.find_index", &args[1], 2)?;
        for (i, item) in items.into_iter().enumerate() {
            let matches = pred.call(vec![item])?;
            if matches.is_truthy() {
                return Ok(Value::Number(i as f64));
            }
        }
        Ok(Value::Number(-1.0))
    }

    /// `list.every(items, predicate) -> bool` — true if pred holds for all.
    fn every(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 2 {
            return Err(StdlibError::wrong_args("list.every", 2, args.len()));
        }
        let items = extract_list("list.every", &args[0])?;
        let pred = extract_function("list.every", &args[1], 2)?;
        for item in items {
            let result = pred.call(vec![item])?;
            if !result.is_truthy() {
                return Ok(Value::Bool(false));
            }
        }
        Ok(Value::Bool(true))
    }

    /// `list.any(items, predicate) -> bool` — true if pred holds for any element.
    /// Also available as `list.some` (backward-compat alias).
    fn any(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 2 {
            return Err(StdlibError::wrong_args("list.any", 2, args.len()));
        }
        let items = extract_list("list.any", &args[0])?;
        let pred = extract_function("list.any", &args[1], 2)?;
        for item in items {
            let result = pred.call(vec![item])?;
            if result.is_truthy() {
                return Ok(Value::Bool(true));
            }
        }
        Ok(Value::Bool(false))
    }

    /// `list.sort(items, compare) -> list` — stable sort using comparator.
    ///
    /// The comparator `fn(a, b) -> number` must return:
    /// - negative if a < b
    /// - zero if a == b
    /// - positive if a > b
    fn sort(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 2 {
            return Err(StdlibError::wrong_args("list.sort", 2, args.len()));
        }
        let mut items = extract_list("list.sort", &args[0])?;
        let cmp = extract_function("list.sort", &args[1], 2)?;

        // We need to propagate errors from the comparator, so we use a cell
        // to capture the first error that occurs during sorting.
        let mut sort_error: Option<StdlibError> = None;

        items.sort_by(|a, b| {
            if sort_error.is_some() {
                return std::cmp::Ordering::Equal;
            }
            match cmp.call(vec![a.clone(), b.clone()]) {
                Ok(Value::Number(n)) => {
                    if n < 0.0 {
                        std::cmp::Ordering::Less
                    } else if n > 0.0 {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Equal
                    }
                }
                Ok(other) => {
                    sort_error = Some(StdlibError::RuntimeError(format!(
                        "list.sort: comparator must return a number, got {}",
                        other.type_name()
                    )));
                    std::cmp::Ordering::Equal
                }
                Err(e) => {
                    sort_error = Some(e);
                    std::cmp::Ordering::Equal
                }
            }
        });

        if let Some(e) = sort_error {
            return Err(e);
        }
        Ok(Value::List(items))
    }

    /// `list.count(items, predicate) -> number` — counts elements matching pred.
    fn count(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 2 {
            return Err(StdlibError::wrong_args("list.count", 2, args.len()));
        }
        let items = extract_list("list.count", &args[0])?;
        let pred = extract_function("list.count", &args[1], 2)?;
        let mut n = 0usize;
        for item in items {
            let result = pred.call(vec![item])?;
            if result.is_truthy() {
                n += 1;
            }
        }
        Ok(Value::Number(n as f64))
    }

    // ── Query ─────────────────────────────────────────────────────────────────

    /// `list.contains(items, value) -> bool` — value equality check.
    fn contains(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 2 {
            return Err(StdlibError::wrong_args("list.contains", 2, args.len()));
        }
        let items = extract_list("list.contains", &args[0])?;
        let needle = &args[1];
        Ok(Value::Bool(items.contains(needle)))
    }

    /// `list.zip(a, b) -> list` — pairs elements from two lists into records.
    ///
    /// Returns a list of `{ first, second }` records. If lists differ in length,
    /// stops at the shorter list.
    fn zip(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 2 {
            return Err(StdlibError::wrong_args("list.zip", 2, args.len()));
        }
        let a = extract_list("list.zip", &args[0])?;
        let b = match &args[1] {
            Value::List(items) => items.clone(),
            other => {
                return Err(StdlibError::type_mismatch(
                    "list.zip",
                    2,
                    "list",
                    other.type_name(),
                ))
            }
        };
        let result: Vec<Value> = a
            .into_iter()
            .zip(b)
            .map(|(first, second)| {
                let mut fields = std::collections::BTreeMap::new();
                fields.insert("first".to_string(), first);
                fields.insert("second".to_string(), second);
                Value::record(fields)
            })
            .collect();
        Ok(Value::List(result))
    }

    /// `list.take(items, n) -> list` — takes first n elements.
    fn take(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 2 {
            return Err(StdlibError::wrong_args("list.take", 2, args.len()));
        }
        let items = extract_list("list.take", &args[0])?;
        let n = extract_number("list.take", &args[1], 2)?;
        if n.fract() != 0.0 || !n.is_finite() || n < 0.0 {
            return Err(StdlibError::RuntimeError(
                "list.take: count must be a non-negative integer".to_string(),
            ));
        }
        let n = (n as usize).min(items.len());
        Ok(Value::List(items[..n].to_vec()))
    }

    /// `list.drop(items, n) -> list` — returns all elements after the first n.
    ///
    /// Spec: `list.drop(xs: list<T>, count: number) -> list<T>`
    /// If count >= length, returns empty list. If count <= 0, returns full list.
    fn drop_fn(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 2 {
            return Err(StdlibError::wrong_args("list.drop", 2, args.len()));
        }
        let items = extract_list("list.drop", &args[0])?;
        let n = extract_number("list.drop", &args[1], 2)?;
        if n.fract() != 0.0 || !n.is_finite() || n < 0.0 {
            return Err(StdlibError::RuntimeError(
                "list.drop: count must be a non-negative integer".to_string(),
            ));
        }
        let n = (n as usize).min(items.len());
        Ok(Value::List(items[n..].to_vec()))
    }
}
