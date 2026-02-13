//! PEPL Standard Library
//!
//! 88 Phase 0 functions across 9 pure modules + 4 capability modules.
//! All pure functions are deterministic and execute in < 1ms.
//! Capability modules validate arguments and yield to the host via `CapabilityCall`.
//!
//! # Pure Modules
//!
//! | Module | Functions | Description |
//! |--------|-----------|-------------|
//! | `core` | 4 | Logging, assertions, type inspection, capability check |
//! | `math` | 10 + 2 constants | Arithmetic beyond basic operators |
//! | `string` | 20 | String manipulation |
//! | `list` | 31 | List construction, query, transformation, higher-order |
//! | `record` | 5 | Record field access and manipulation |
//! | `time` | 5 | Host-provided timestamps and formatting |
//! | `convert` | 5 | Type conversion (fallible and infallible) |
//! | `json` | 2 | JSON parse/stringify |
//! | `timer` | 4 | Recurring and one-shot timer scheduling |
//!
//! # Capability Modules
//!
//! | Module | Functions | cap_id | Description |
//! |--------|-----------|--------|-------------|
//! | `http` | 5 | 1 | HTTP requests (get, post, put, patch, delete) |
//! | `storage` | 4 | 2 | Persistent key-value storage (get, set, delete, keys) |
//! | `location` | 1 | 3 | GPS/location access (current) |
//! | `notifications` | 1 | 4 | Push notifications (send) |

mod error;
mod module;
mod value;

pub mod capability;
pub mod modules;

pub use error::StdlibError;
pub use module::StdlibModule;
pub use value::{ResultValue, StdlibFn, Value};
