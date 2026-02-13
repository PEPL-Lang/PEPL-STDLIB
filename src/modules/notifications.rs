//! `notifications` capability module — push notifications (host-delegated).
//!
//! Functions: send.
//! Notification delivery is host-delegated — the runtime host sends actual
//! notifications via `env.host_call(cap_id=4, fn_id=1, payload)`. This module
//! validates arguments and returns a `CapabilityCall` error to signal the
//! caller to route the call to the host.
//!
//! # Cap ID / Fn ID Mapping
//!
//! | fn_id | Function |
//! |-------|----------|
//! | 1     | send     |

use crate::capability::{CAP_NOTIFICATIONS, NOTIFICATIONS_SEND};
use crate::error::StdlibError;
use crate::module::StdlibModule;
use crate::value::Value;

/// The `notifications` capability module.
pub struct NotificationsModule;

impl NotificationsModule {
    pub fn new() -> Self {
        Self
    }
}

impl Default for NotificationsModule {
    fn default() -> Self {
        Self::new()
    }
}

impl StdlibModule for NotificationsModule {
    fn name(&self) -> &'static str {
        "notifications"
    }

    fn has_function(&self, function: &str) -> bool {
        matches!(function, "send")
    }

    fn call(&self, function: &str, args: Vec<Value>) -> Result<Value, StdlibError> {
        match function {
            "send" => self.send(args),
            _ => Err(StdlibError::unknown_function("notifications", function)),
        }
    }
}

impl NotificationsModule {
    /// `notifications.send(title: string, body: string) -> Result<nil, NotificationError>`
    ///
    /// Validates: exactly 2 args, both must be strings.
    /// Returns `CapabilityCall` with cap_id=4, fn_id=1.
    fn send(&self, args: Vec<Value>) -> Result<Value, StdlibError> {
        if args.len() != 2 {
            return Err(StdlibError::wrong_args("notifications.send", 2, args.len()));
        }
        validate_string("notifications.send", &args[0], 1)?;
        validate_string("notifications.send", &args[1], 2)?;
        Err(StdlibError::capability_call(
            "notifications",
            "send",
            CAP_NOTIFICATIONS,
            NOTIFICATIONS_SEND,
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
