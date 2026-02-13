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

    /// Capability call — cannot be executed locally, must be routed to host.
    /// The caller should use `cap_id` and `fn_id` for `env.host_call` dispatch.
    #[error("{module}.{function}: capability call requires host (cap_id={cap_id}, fn_id={fn_id})")]
    CapabilityCall {
        module: String,
        function: String,
        cap_id: u32,
        fn_id: u32,
        args: Vec<crate::value::Value>,
    },
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
    pub fn type_mismatch(function: &str, position: usize, expected: &str, got: &str) -> Self {
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

    /// Create a `CapabilityCall` error — signals that this call must be routed to the host.
    pub fn capability_call(
        module: &str,
        function: &str,
        cap_id: u32,
        fn_id: u32,
        args: Vec<crate::value::Value>,
    ) -> Self {
        Self::CapabilityCall {
            module: module.to_string(),
            function: function.to_string(),
            cap_id,
            fn_id,
            args,
        }
    }
}
