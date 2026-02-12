//! PEPL Standard Library
//!
//! 88 Phase 0 functions across 9 core modules + 4 capability modules.
//! All core functions are pure, deterministic, and execute in < 1ms.
//!
//! # Modules
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

mod error;
mod module;
mod value;

pub mod modules;

pub use error::StdlibError;
pub use module::StdlibModule;
pub use value::{ResultValue, Value};
