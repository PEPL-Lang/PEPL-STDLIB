use std::collections::BTreeMap;
use std::fmt;

/// Runtime value in PEPL.
///
/// All PEPL values are immutable — operations that "modify" a value return a
/// new value instead. [`BTreeMap`] is used for records to guarantee
/// deterministic iteration order (a core PEPL invariant).
///
/// # Type names
///
/// [`Value::type_name`] returns the string used by `core.type_of()`:
/// `"number"`, `"string"`, `"bool"`, `"nil"`, `"list"`, `"record"` (or the
/// declared type name for named records/sum variants), `"color"`, `"result"`.
#[derive(Debug, Clone)]
pub enum Value {
    /// 64-bit IEEE 754 floating-point number.
    ///
    /// NaN is prevented from entering state — operations that would produce
    /// NaN trap instead.
    Number(f64),

    /// UTF-8 string.
    String(String),

    /// Boolean value.
    Bool(bool),

    /// The absence of a value.
    Nil,

    /// Ordered collection of values.
    List(Vec<Value>),

    /// Named fields with values. Uses [`BTreeMap`] for deterministic ordering.
    ///
    /// `type_name` is `Some("Todo")` for named record types (`type Todo = { ... }`),
    /// `None` for anonymous inline records (`{ x: 1, y: 2 }`).
    Record {
        type_name: Option<String>,
        fields: BTreeMap<String, Value>,
    },

    /// RGBA color value. Each component is in the range 0.0–1.0.
    Color {
        r: f64,
        g: f64,
        b: f64,
        a: f64,
    },

    /// Result type for fallible operations (`Ok` or `Err`).
    Result(Box<ResultValue>),

    /// Sum type variant (e.g., `Shape.Circle(5, 10)`).
    ///
    /// `type_name` is the declaring sum type (e.g., `"Shape"`).
    /// `variant` is the variant name (e.g., `"Circle"`).
    /// `fields` holds positional values — empty for unit variants like `Active`.
    SumVariant {
        type_name: String,
        variant: String,
        fields: Vec<Value>,
    },
}

/// The two variants of a PEPL `Result` value.
#[derive(Debug, Clone)]
pub enum ResultValue {
    Ok(Value),
    Err(Value),
}

// ── Equality ──────────────────────────────────────────────────────────────────
//
// Structural equality per execution-semantics.md:
//   - number:  IEEE 754 (NaN != NaN) — handled by f64 partial_eq
//   - string:  byte-for-byte UTF-8
//   - bool:    value equality
//   - nil:     nil == nil
//   - list:    same length + element-by-element
//   - record:  recursive field-by-field
//   - color:   RGBA value comparison
//   - result:  same variant + same inner value
//   - record:  structural (type_name ignored — type checker ensures compatibility)
//   - sum:     nominal (type_name + variant + fields must all match)
//   - Note: Functions/lambdas live in EvalValue (pepl-eval), not here

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b, // IEEE 754: NaN != NaN
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Nil, Value::Nil) => true,
            (Value::List(a), Value::List(b)) => a == b,
            // Structural equality for records — type_name is metadata, not identity
            (Value::Record { fields: a, .. }, Value::Record { fields: b, .. }) => a == b,
            (Value::Color { r: r1, g: g1, b: b1, a: a1 },
             Value::Color { r: r2, g: g2, b: b2, a: a2 }) => {
                r1 == r2 && g1 == g2 && b1 == b2 && a1 == a2
            }
            (Value::Result(a), Value::Result(b)) => a == b,
            // Nominal equality for sum variants — same type + variant + fields
            (Value::SumVariant { type_name: t1, variant: v1, fields: f1 },
             Value::SumVariant { type_name: t2, variant: v2, fields: f2 }) => {
                t1 == t2 && v1 == v2 && f1 == f2
            }
            _ => false, // different variants are never equal
        }
    }
}

impl PartialEq for ResultValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ResultValue::Ok(a), ResultValue::Ok(b)) => a == b,
            (ResultValue::Err(a), ResultValue::Err(b)) => a == b,
            _ => false,
        }
    }
}

// ── Display ───────────────────────────────────────────────────────────────────
//
// Used by `core.log`, `convert.to_string`, and `string.from`.

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => {
                // Print integers without decimal point
                if n.fract() == 0.0 && n.is_finite() {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{n}")
                }
            }
            Value::String(s) => write!(f, "{s}"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::Nil => write!(f, "nil"),
            Value::List(items) => {
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    // Strings inside lists/records get quoted
                    match item {
                        Value::String(s) => write!(f, "\"{s}\"")?,
                        other => write!(f, "{other}")?,
                    }
                }
                write!(f, "]")
            }
            Value::Record { type_name, fields } => {
                if let Some(name) = type_name {
                    write!(f, "{name}")?;
                }
                write!(f, "{{")?;
                for (i, (key, val)) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    match val {
                        Value::String(s) => write!(f, "{key}: \"{s}\"")?,
                        other => write!(f, "{key}: {other}")?,
                    }
                }
                write!(f, "}}")
            }
            Value::SumVariant { variant, fields, .. } => {
                write!(f, "{variant}")?;
                if !fields.is_empty() {
                    write!(f, "(")?;
                    for (i, val) in fields.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{val}")?;
                    }
                    write!(f, ")")?;
                }
                Ok(())
            }
            Value::Color { r, g, b, a } => {
                write!(f, "color({r}, {g}, {b}, {a})")
            }
            Value::Result(res) => match res.as_ref() {
                ResultValue::Ok(v) => write!(f, "Ok({v})"),
                ResultValue::Err(v) => write!(f, "Err({v})"),
            },
        }
    }
}

// ── Constructors & Helpers ────────────────────────────────────────────────────

impl Value {
    /// Returns the PEPL type name as used by `core.type_of()`.
    pub fn type_name(&self) -> &str {
        match self {
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Bool(_) => "bool",
            Value::Nil => "nil",
            Value::List(_) => "list",
            Value::Record { type_name: Some(name), .. } => name.as_str(),
            Value::Record { type_name: None, .. } => "record",
            Value::Color { .. } => "color",
            Value::Result(_) => "result",
            Value::SumVariant { type_name, .. } => type_name.as_str(),
        }
    }

    /// Returns `true` if this value is truthy.
    ///
    /// Truthiness rules (per `convert.to_bool`):
    /// - `false`, `nil`, `0`, `""` → falsy
    /// - everything else → truthy
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(false) => false,
            Value::Nil => false,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            _ => true, // List, Record, Color, Result, SumVariant are truthy
        }
    }

    /// Convenience: wrap in `Ok` result.
    pub fn ok(self) -> Value {
        Value::Result(Box::new(ResultValue::Ok(self)))
    }

    /// Convenience: wrap in `Err` result.
    pub fn err(self) -> Value {
        Value::Result(Box::new(ResultValue::Err(self)))
    }

    /// Create an anonymous record (no type name).
    pub fn record(fields: BTreeMap<String, Value>) -> Value {
        Value::Record { type_name: None, fields }
    }

    /// Create a named record (e.g., `type Todo = { ... }`).
    pub fn named_record(type_name: impl Into<String>, fields: BTreeMap<String, Value>) -> Value {
        Value::Record { type_name: Some(type_name.into()), fields }
    }

    /// Create a unit sum variant (no payload fields).
    pub fn unit_variant(type_name: impl Into<String>, variant: impl Into<String>) -> Value {
        Value::SumVariant {
            type_name: type_name.into(),
            variant: variant.into(),
            fields: vec![],
        }
    }

    /// Create a sum variant with positional fields.
    pub fn sum_variant(
        type_name: impl Into<String>,
        variant: impl Into<String>,
        fields: Vec<Value>,
    ) -> Value {
        Value::SumVariant {
            type_name: type_name.into(),
            variant: variant.into(),
            fields,
        }
    }

    /// Try to extract a number, returning `None` if not a `Number`.
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// Try to extract a string reference, returning `None` if not a `String`.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    /// Try to extract a bool, returning `None` if not a `Bool`.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Try to extract a list reference, returning `None` if not a `List`.
    pub fn as_list(&self) -> Option<&[Value]> {
        match self {
            Value::List(l) => Some(l),
            _ => None,
        }
    }

    /// Try to extract a record reference, returning `None` if not a `Record`.
    pub fn as_record(&self) -> Option<&BTreeMap<String, Value>> {
        match self {
            Value::Record { fields, .. } => Some(fields),
            _ => None,
        }
    }

    /// Try to extract sum variant info: `(type_name, variant, fields)`.
    pub fn as_variant(&self) -> Option<(&str, &str, &[Value])> {
        match self {
            Value::SumVariant { type_name, variant, fields } => {
                Some((type_name, variant, fields))
            }
            _ => None,
        }
    }

    /// Returns the declared type name for named records and sum variants.
    /// Returns `None` for anonymous records and all other value types.
    pub fn declared_type_name(&self) -> Option<&str> {
        match self {
            Value::Record { type_name: Some(name), .. } => Some(name),
            Value::SumVariant { type_name, .. } => Some(type_name),
            _ => None,
        }
    }
}

// ── From impls ────────────────────────────────────────────────────────────────

impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Value::Number(n)
    }
}

impl From<i64> for Value {
    fn from(n: i64) -> Self {
        Value::Number(n as f64)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(s.to_string())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<BTreeMap<String, Value>> for Value {
    fn from(fields: BTreeMap<String, Value>) -> Self {
        Value::Record { type_name: None, fields }
    }
}
