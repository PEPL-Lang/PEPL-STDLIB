//! `http` capability module — HTTP request functions (host-delegated).
//!
//! Functions: get, post, put, patch, delete.
//! All HTTP operations are host-delegated — the runtime host performs actual
//! requests via `env.host_call(cap_id=1, fn_id, payload)`. This module
//! validates arguments and returns `CapabilityCall` errors to signal the
//! caller to route the call to the host.
//!
//! # Cap ID / Fn ID Mapping
//!
//! | fn_id | Function |
//! |-------|----------|
//! | 1     | get      |
//! | 2     | post     |
//! | 3     | put      |
//! | 4     | patch    |
//! | 5     | delete   |

use crate::capability::{CAP_HTTP, HTTP_DELETE, HTTP_GET, HTTP_PATCH, HTTP_POST, HTTP_PUT};
use crate::error::StdlibError;
use crate::module::StdlibModule;
use crate::value::Value;

/// The `http` capability module.
pub struct HttpModule;

impl HttpModule {
    pub fn new() -> Self {
        Self
    }
}

impl Default for HttpModule {
    fn default() -> Self {
        Self::new()
    }
}

impl StdlibModule for HttpModule {
    fn name(&self) -> &'static str {
        "http"
    }

    fn has_function(&self, function: &str) -> bool {
        matches!(function, "get" | "post" | "put" | "patch" | "delete")
    }

    fn call(&self, function: &str, args: Vec<Value>) -> Result<Value, StdlibError> {
        match function {
            "get" => self.get(args),
            "post" => self.post(args),
            "put" => self.put(args),
            "patch" => self.patch(args),
            "delete" => self.delete(args),
            _ => Err(StdlibError::unknown_function("http", function)),
        }
    }
}

impl HttpModule {
    /// `http.get(url: string, options?: HttpOptions) -> Result<HttpResponse, HttpError>`
    ///
    /// Validates: 1 or 2 args, first must be string.
    /// Returns `CapabilityCall` with cap_id=1, fn_id=1.
    fn get(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.is_empty() || args.len() > 2 {
            return Err(StdlibError::wrong_args("http.get", 1, args.len()));
        }
        validate_string("http.get", &args[0], 1)?;
        Err(StdlibError::capability_call(
            "http", "get", CAP_HTTP, HTTP_GET, args,
        ))
    }

    /// `http.post(url: string, body: string, options?: HttpOptions) -> Result<HttpResponse, HttpError>`
    ///
    /// Validates: 2 or 3 args, first two must be strings.
    /// Returns `CapabilityCall` with cap_id=1, fn_id=2.
    fn post(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() < 2 || args.len() > 3 {
            return Err(StdlibError::wrong_args("http.post", 2, args.len()));
        }
        validate_string("http.post", &args[0], 1)?;
        validate_string("http.post", &args[1], 2)?;
        Err(StdlibError::capability_call(
            "http", "post", CAP_HTTP, HTTP_POST, args,
        ))
    }

    /// `http.put(url: string, body: string, options?: HttpOptions) -> Result<HttpResponse, HttpError>`
    ///
    /// Validates: 2 or 3 args, first two must be strings.
    /// Returns `CapabilityCall` with cap_id=1, fn_id=3.
    fn put(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() < 2 || args.len() > 3 {
            return Err(StdlibError::wrong_args("http.put", 2, args.len()));
        }
        validate_string("http.put", &args[0], 1)?;
        validate_string("http.put", &args[1], 2)?;
        Err(StdlibError::capability_call(
            "http", "put", CAP_HTTP, HTTP_PUT, args,
        ))
    }

    /// `http.patch(url: string, body: string, options?: HttpOptions) -> Result<HttpResponse, HttpError>`
    ///
    /// Validates: 2 or 3 args, first two must be strings.
    /// Returns `CapabilityCall` with cap_id=1, fn_id=4.
    fn patch(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() < 2 || args.len() > 3 {
            return Err(StdlibError::wrong_args("http.patch", 2, args.len()));
        }
        validate_string("http.patch", &args[0], 1)?;
        validate_string("http.patch", &args[1], 2)?;
        Err(StdlibError::capability_call(
            "http", "patch", CAP_HTTP, HTTP_PATCH, args,
        ))
    }

    /// `http.delete(url: string, options?: HttpOptions) -> Result<HttpResponse, HttpError>`
    ///
    /// Validates: 1 or 2 args, first must be string.
    /// Returns `CapabilityCall` with cap_id=1, fn_id=5.
    fn delete(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.is_empty() || args.len() > 2 {
            return Err(StdlibError::wrong_args("http.delete", 1, args.len()));
        }
        validate_string("http.delete", &args[0], 1)?;
        Err(StdlibError::capability_call(
            "http",
            "delete",
            CAP_HTTP,
            HTTP_DELETE,
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
