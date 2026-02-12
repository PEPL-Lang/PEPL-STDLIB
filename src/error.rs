use thiserror::Error;

/// Errors that can occur when calling stdlib functions.
#[derive(Debug, Clone, Error)]
pub enum StdlibError {
    /// Wrong number of arguments passed to a function.
    #[error("{function}: expected {expected} argument(s), got {got}")]
    WrongArgCount {
        function: String,
        expected: usize,
        got: usize,
    },

    /// Argument has the wrong type.
    #[error("{function}: argument {position} expected {expected}, got {got}")]
    TypeMismatch {
        function: String,
        position: usize,
        expected: String,
        got: String,
    },

    /// `core.assert` failed.
    #[error("Assertion failed: {message}")]
    AssertionFailed { message: String },

    /// Unknown function in module.
    #[error("Unknown function: {module}.{function}")]
    UnknownFunction { module: String, function: String },

    /// Generic runtime error (e.g., NaN would be produced, division by zero).
    #[error("{0}")]
    RuntimeError(String),
}

impl StdlibError {
    /// Create a `WrongArgCount` error.
    pub fn wrong_args(function: &str, expected: usize, got: usize) -> Self {
        Self::WrongArgCount {
            function: function.to_string(),
            expected,
            got,
        }
    }

    /// Create a `TypeMismatch` error.
    pub fn type_mismatch(
        function: &str,
        position: usize,
        expected: &str,
        got: &str,
    ) -> Self {
        Self::TypeMismatch {
            function: function.to_string(),
            position,
            expected: expected.to_string(),
            got: got.to_string(),
        }
    }

    /// Create an `UnknownFunction` error.
    pub fn unknown_function(module: &str, function: &str) -> Self {
        Self::UnknownFunction {
            module: module.to_string(),
            function: function.to_string(),
        }
    }
}
