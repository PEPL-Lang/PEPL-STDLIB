//! `location` capability module — GPS/location access (host-delegated).
//!
//! Functions: current.
//! Location access is host-delegated — the runtime host reads actual device
//! sensors via `env.host_call(cap_id=3, fn_id=1, payload)`. This module
//! validates arguments and returns a `CapabilityCall` error to signal the
//! caller to route the call to the host.
//!
//! # Cap ID / Fn ID Mapping
//!
//! | fn_id | Function |
//! |-------|----------|
//! | 1     | current  |

use crate::capability::{CAP_LOCATION, LOCATION_CURRENT};
use crate::error::StdlibError;
use crate::module::StdlibModule;
use crate::value::Value;

/// The `location` capability module.
pub struct LocationModule;

impl LocationModule {
    pub fn new() -> Self {
        Self
    }
}

impl Default for LocationModule {
    fn default() -> Self {
        Self::new()
    }
}

impl StdlibModule for LocationModule {
    fn name(&self) -> &'static str {
        "location"
    }

    fn has_function(&self, function: &str) -> bool {
        matches!(function, "current")
    }

    fn call(&self, function: &str, args: Vec<Value>) -> Result<Value, StdlibError> {
        match function {
            "current" => self.current(args),
            _ => Err(StdlibError::unknown_function("location", function)),
        }
    }
}

impl LocationModule {
    /// `location.current() -> Result<{ lat: number, lon: number }, LocationError>`
    ///
    /// Validates: no args.
    /// Returns `CapabilityCall` with cap_id=3, fn_id=1.
    fn current(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if !args.is_empty() {
            return Err(StdlibError::wrong_args("location.current", 0, args.len()));
        }
        Err(StdlibError::capability_call(
            "location",
            "current",
            CAP_LOCATION,
            LOCATION_CURRENT,
            args,
        ))
    }
}
