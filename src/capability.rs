//! Capability ID mappings for `env.host_call` dispatch.
//!
//! Each capability module has a unique `cap_id` and each function within
//! it has a unique `fn_id`. These constants are used by:
//! - The stdlib capability modules (to return `CapabilityCall` errors)
//! - The WASM code generator (to emit `env.host_call(cap_id, fn_id, ...)` instructions)

// ── Capability IDs ───────────────────────────────────────────────────────────

/// HTTP capability (get, post, put, patch, delete).
pub const CAP_HTTP: u32 = 1;

/// Persistent storage capability (get, set, delete, keys).
pub const CAP_STORAGE: u32 = 2;

/// Location/GPS capability (current).
pub const CAP_LOCATION: u32 = 3;

/// Push notifications capability (send).
pub const CAP_NOTIFICATIONS: u32 = 4;

/// Credential resolution (internal — PEPL code does not call directly).
pub const CAP_CREDENTIAL: u32 = 5;

// ── Function IDs: http ───────────────────────────────────────────────────────

pub const HTTP_GET: u32 = 1;
pub const HTTP_POST: u32 = 2;
pub const HTTP_PUT: u32 = 3;
pub const HTTP_PATCH: u32 = 4;
pub const HTTP_DELETE: u32 = 5;

// ── Function IDs: storage ────────────────────────────────────────────────────

pub const STORAGE_GET: u32 = 1;
pub const STORAGE_SET: u32 = 2;
pub const STORAGE_DELETE: u32 = 3;
pub const STORAGE_KEYS: u32 = 4;

// ── Function IDs: location ───────────────────────────────────────────────────

pub const LOCATION_CURRENT: u32 = 1;

// ── Function IDs: notifications ──────────────────────────────────────────────

pub const NOTIFICATIONS_SEND: u32 = 1;

// ── Function IDs: credential ─────────────────────────────────────────────────

pub const CREDENTIAL_GET: u32 = 1;

// ── Lookup ───────────────────────────────────────────────────────────────────

/// Resolve a capability module name + function name to `(cap_id, fn_id)`.
///
/// Returns `None` if the module/function combination is not a capability call.
///
/// # Example
/// ```
/// use pepl_stdlib::capability::resolve_ids;
/// assert_eq!(resolve_ids("http", "get"), Some((1, 1)));
/// assert_eq!(resolve_ids("storage", "keys"), Some((2, 4)));
/// assert_eq!(resolve_ids("math", "abs"), None);
/// ```
pub fn resolve_ids(module: &str, function: &str) -> Option<(u32, u32)> {
    match (module, function) {
        ("http", "get") => Some((CAP_HTTP, HTTP_GET)),
        ("http", "post") => Some((CAP_HTTP, HTTP_POST)),
        ("http", "put") => Some((CAP_HTTP, HTTP_PUT)),
        ("http", "patch") => Some((CAP_HTTP, HTTP_PATCH)),
        ("http", "delete") => Some((CAP_HTTP, HTTP_DELETE)),

        ("storage", "get") => Some((CAP_STORAGE, STORAGE_GET)),
        ("storage", "set") => Some((CAP_STORAGE, STORAGE_SET)),
        ("storage", "delete") => Some((CAP_STORAGE, STORAGE_DELETE)),
        ("storage", "keys") => Some((CAP_STORAGE, STORAGE_KEYS)),

        ("location", "current") => Some((CAP_LOCATION, LOCATION_CURRENT)),

        ("notifications", "send") => Some((CAP_NOTIFICATIONS, NOTIFICATIONS_SEND)),

        _ => None,
    }
}

/// Returns `true` if the given module name is a capability module.
pub fn is_capability_module(module: &str) -> bool {
    matches!(module, "http" | "storage" | "location" | "notifications")
}

/// Returns all capability module names.
pub fn capability_module_names() -> &'static [&'static str] {
    &["http", "storage", "location", "notifications"]
}
