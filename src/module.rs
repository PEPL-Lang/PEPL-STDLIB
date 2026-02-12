use crate::error::StdlibError;
use crate::value::Value;

/// Trait implemented by each PEPL stdlib module.
///
/// Every module (core, math, string, list, etc.) implements this trait.
/// The evaluator dispatches `module.function(args...)` calls through it.
///
/// # Example
///
/// ```ignore
/// let core_mod = CoreModule::new();
/// let result = core_mod.call("type_of", vec![Value::Number(42.0)])?;
/// assert_eq!(result, Value::String("number".into()));
/// ```
pub trait StdlibModule {
    /// Module name as it appears in PEPL source (e.g., `"core"`, `"math"`).
    fn name(&self) -> &'static str;

    /// Check if a function exists in this module.
    fn has_function(&self, function: &str) -> bool;

    /// Call a function in this module with the given arguments.
    ///
    /// Returns `Err(StdlibError::UnknownFunction)` if the function doesn't exist.
    /// Returns `Err(StdlibError::WrongArgCount)` if argument count is wrong.
    /// Returns `Err(StdlibError::TypeMismatch)` if an argument has the wrong type.
    fn call(&self, function: &str, args: Vec<Value>) -> Result<Value, StdlibError>;
}
