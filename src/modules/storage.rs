//! `storage` capability module — persistent key-value storage (host-delegated).
//!
//! Functions: get, set, delete, keys.
//! All storage operations are host-delegated — the runtime host manages actual
//! persistence via `env.host_call(cap_id=2, fn_id, payload)`. This module
//! validates arguments and returns `CapabilityCall` errors to signal the
//! caller to route the call to the host.
//!
//! # Cap ID / Fn ID Mapping
//!
//! | fn_id | Function |
//! |-------|----------|
//! | 1     | get      |
//! | 2     | set      |
//! | 3     | delete   |
//! | 4     | keys     |

use crate::capability::{CAP_STORAGE, STORAGE_DELETE, STORAGE_GET, STORAGE_KEYS, STORAGE_SET};
use crate::error::StdlibError;
use crate::module::StdlibModule;
use crate::value::Value;

/// The `storage` capability module.
pub struct StorageModule;

impl StorageModule {
    pub fn new() -> Self {
        Self
    }
}

impl Default for StorageModule {
    fn default() -> Self {
        Self::new()
    }
}

impl StdlibModule for StorageModule {
    fn name(&self) -> &'static str {
        "storage"
    }

    fn has_function(&self, function: &str) -> bool {
        matches!(function, "get" | "set" | "delete" | "keys")
    }

    fn call(&self, function: &str, args: Vec<Value>) -> Result<Value, StdlibError> {
        match function {
            "get" => self.get(args),
            "set" => self.set(args),
            "delete" => self.delete(args),
            "keys" => self.keys(args),
            _ => Err(StdlibError::unknown_function("storage", function)),
        }
    }
}

impl StorageModule {
    /// `storage.get(key: string) -> Result<string, StorageError>`
    ///
    /// Validates: exactly 1 arg, must be string.
    /// Returns `CapabilityCall` with cap_id=2, fn_id=1.
    fn get(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 1 {
            return Err(StdlibError::wrong_args("storage.get", 1, args.len()));
        }
        validate_string("storage.get", &args[0], 1)?;
        Err(StdlibError::capability_call(
            "storage",
            "get",
            CAP_STORAGE,
            STORAGE_GET,
            args,
        ))
    }

    /// `storage.set(key: string, value: string) -> Result<nil, StorageError>`
    ///
    /// Validates: exactly 2 args, both must be strings.
    /// Returns `CapabilityCall` with cap_id=2, fn_id=2.
    fn set(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 2 {
            return Err(StdlibError::wrong_args("storage.set", 2, args.len()));
        }
        validate_string("storage.set", &args[0], 1)?;
        validate_string("storage.set", &args[1], 2)?;
        Err(StdlibError::capability_call(
            "storage",
            "set",
            CAP_STORAGE,
            STORAGE_SET,
            args,
        ))
    }

    /// `storage.delete(key: string) -> Result<nil, StorageError>`
    ///
    /// Validates: exactly 1 arg, must be string.
    /// Returns `CapabilityCall` with cap_id=2, fn_id=3.
    fn delete(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 1 {
            return Err(StdlibError::wrong_args("storage.delete", 1, args.len()));
        }
        validate_string("storage.delete", &args[0], 1)?;
        Err(StdlibError::capability_call(
            "storage",
            "delete",
            CAP_STORAGE,
            STORAGE_DELETE,
            args,
        ))
    }

    /// `storage.keys() -> Result<list<string>, StorageError>`
    ///
    /// Validates: no args.
    /// Returns `CapabilityCall` with cap_id=2, fn_id=4.
    fn keys(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if !args.is_empty() {
            return Err(StdlibError::wrong_args("storage.keys", 0, args.len()));
        }
        Err(StdlibError::capability_call(
            "storage",
            "keys",
            CAP_STORAGE,
            STORAGE_KEYS,
            args,
        ))
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn validate_string(func: &str, val: &Value, pos: usize) -> Result<(), StdlibError> {
    match val {
        Value::String(_) => Ok(()),
        _ => Err(StdlibError::type_mismatch(
            func,
            pos,
            "string",
            val.type_name(),
        )),
    }
}
