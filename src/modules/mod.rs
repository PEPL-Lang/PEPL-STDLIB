//! Stdlib module implementations.
//!
//! Each PEPL stdlib module gets its own submodule here.
//! Pure modules execute locally. Capability modules validate arguments
//! and return `CapabilityCall` errors for host dispatch.

pub mod convert;
pub mod core;
pub mod http;
pub mod json;
pub mod list;
pub mod location;
pub mod math;
pub mod notifications;
pub mod record;
pub mod storage;
pub mod string;
pub mod time;
pub mod timer;
