//! Tests for capability modules: http, storage, location, notifications.
//!
//! Each capability module validates arguments and returns `CapabilityCall` errors.
//! Tests verify:
//! - Correct cap_id/fn_id in returned errors
//! - Argument validation (arity, types)
//! - Unknown function handling
//! - 100-iteration determinism

use pepl_stdlib::capability::{
    self, CAP_HTTP, CAP_LOCATION, CAP_NOTIFICATIONS, CAP_STORAGE, HTTP_DELETE, HTTP_GET,
    HTTP_PATCH, HTTP_POST, HTTP_PUT, LOCATION_CURRENT, NOTIFICATIONS_SEND, STORAGE_DELETE,
    STORAGE_GET, STORAGE_KEYS, STORAGE_SET,
};
use pepl_stdlib::modules::http::HttpModule;
use pepl_stdlib::modules::location::LocationModule;
use pepl_stdlib::modules::notifications::NotificationsModule;
use pepl_stdlib::modules::storage::StorageModule;
use pepl_stdlib::StdlibError;
use pepl_stdlib::StdlibModule;
use pepl_stdlib::Value;
use std::collections::BTreeMap;

// ── Helper ───────────────────────────────────────────────────────────────────

/// Extract cap_id and fn_id from a CapabilityCall error.
fn extract_cap_call(err: &StdlibError) -> (u32, u32) {
    match err {
        StdlibError::CapabilityCall { cap_id, fn_id, .. } => (*cap_id, *fn_id),
        other => panic!("Expected CapabilityCall, got: {other}"),
    }
}

/// Assert a call returns CapabilityCall with the expected IDs.
fn assert_capability_call(
    module: &dyn StdlibModule,
    function: &str,
    args: Vec<Value>,
    expected_cap: u32,
    expected_fn: u32,
) {
    let result = module.call(function, args);
    assert!(
        result.is_err(),
        "{}.{function} should return Err",
        module.name()
    );
    let err = result.unwrap_err();
    let (cap, fid) = extract_cap_call(&err);
    assert_eq!(
        cap,
        expected_cap,
        "cap_id mismatch for {}.{function}",
        module.name()
    );
    assert_eq!(
        fid,
        expected_fn,
        "fn_id mismatch for {}.{function}",
        module.name()
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// HTTP MODULE TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn http_module_name() {
    assert_eq!(HttpModule::new().name(), "http");
}

#[test]
fn http_has_function() {
    let m = HttpModule::new();
    assert!(m.has_function("get"));
    assert!(m.has_function("post"));
    assert!(m.has_function("put"));
    assert!(m.has_function("patch"));
    assert!(m.has_function("delete"));
    assert!(!m.has_function("head"));
    assert!(!m.has_function("options"));
}

#[test]
fn http_get_returns_capability_call() {
    let m = HttpModule::new();
    assert_capability_call(
        &m,
        "get",
        vec![Value::String("https://example.com".into())],
        CAP_HTTP,
        HTTP_GET,
    );
}

#[test]
fn http_get_with_options() {
    let m = HttpModule::new();
    // get accepts 1 or 2 args (options is optional)
    let mut opts = BTreeMap::new();
    opts.insert("timeout".to_string(), Value::Number(5000.0));
    assert_capability_call(
        &m,
        "get",
        vec![
            Value::String("https://example.com".into()),
            Value::Record {
                type_name: None,
                fields: opts,
            },
        ],
        CAP_HTTP,
        HTTP_GET,
    );
}

#[test]
fn http_get_wrong_arg_count() {
    let m = HttpModule::new();
    // 0 args
    let err = m.call("get", vec![]).unwrap_err();
    assert!(matches!(err, StdlibError::WrongArgCount { .. }));
    // 3 args
    let err = m
        .call(
            "get",
            vec![
                Value::String("a".into()),
                Value::String("b".into()),
                Value::String("c".into()),
            ],
        )
        .unwrap_err();
    assert!(matches!(err, StdlibError::WrongArgCount { .. }));
}

#[test]
fn http_get_wrong_arg_type() {
    let m = HttpModule::new();
    let err = m.call("get", vec![Value::Number(42.0)]).unwrap_err();
    assert!(matches!(err, StdlibError::TypeMismatch { .. }));
}

#[test]
fn http_post_returns_capability_call() {
    let m = HttpModule::new();
    assert_capability_call(
        &m,
        "post",
        vec![
            Value::String("https://api.example.com".into()),
            Value::String("{\"key\": \"value\"}".into()),
        ],
        CAP_HTTP,
        HTTP_POST,
    );
}

#[test]
fn http_post_with_options() {
    let m = HttpModule::new();
    assert_capability_call(
        &m,
        "post",
        vec![
            Value::String("https://api.example.com".into()),
            Value::String("body".into()),
            Value::Record {
                type_name: None,
                fields: BTreeMap::new(),
            },
        ],
        CAP_HTTP,
        HTTP_POST,
    );
}

#[test]
fn http_post_wrong_arg_count() {
    let m = HttpModule::new();
    // 1 arg (needs 2+)
    let err = m
        .call("post", vec![Value::String("url".into())])
        .unwrap_err();
    assert!(matches!(err, StdlibError::WrongArgCount { .. }));
}

#[test]
fn http_post_wrong_arg_type() {
    let m = HttpModule::new();
    // first arg not string
    let err = m
        .call(
            "post",
            vec![Value::Number(1.0), Value::String("body".into())],
        )
        .unwrap_err();
    assert!(matches!(err, StdlibError::TypeMismatch { .. }));
    // second arg not string
    let err = m
        .call(
            "post",
            vec![Value::String("url".into()), Value::Number(1.0)],
        )
        .unwrap_err();
    assert!(matches!(err, StdlibError::TypeMismatch { .. }));
}

#[test]
fn http_put_returns_capability_call() {
    let m = HttpModule::new();
    assert_capability_call(
        &m,
        "put",
        vec![
            Value::String("https://api.example.com/1".into()),
            Value::String("body".into()),
        ],
        CAP_HTTP,
        HTTP_PUT,
    );
}

#[test]
fn http_patch_returns_capability_call() {
    let m = HttpModule::new();
    assert_capability_call(
        &m,
        "patch",
        vec![
            Value::String("https://api.example.com/1".into()),
            Value::String("body".into()),
        ],
        CAP_HTTP,
        HTTP_PATCH,
    );
}

#[test]
fn http_delete_returns_capability_call() {
    let m = HttpModule::new();
    assert_capability_call(
        &m,
        "delete",
        vec![Value::String("https://api.example.com/1".into())],
        CAP_HTTP,
        HTTP_DELETE,
    );
}

#[test]
fn http_unknown_function() {
    let m = HttpModule::new();
    let err = m.call("head", vec![]).unwrap_err();
    assert!(matches!(err, StdlibError::UnknownFunction { .. }));
}

#[test]
fn http_capability_call_preserves_args() {
    let m = HttpModule::new();
    let args = vec![Value::String("https://example.com".into())];
    let err = m.call("get", args).unwrap_err();
    match err {
        StdlibError::CapabilityCall { args, .. } => {
            assert_eq!(args.len(), 1);
            assert!(matches!(&args[0], Value::String(s) if s == "https://example.com"));
        }
        _ => panic!("Expected CapabilityCall"),
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// STORAGE MODULE TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn storage_module_name() {
    assert_eq!(StorageModule::new().name(), "storage");
}

#[test]
fn storage_has_function() {
    let m = StorageModule::new();
    assert!(m.has_function("get"));
    assert!(m.has_function("set"));
    assert!(m.has_function("delete"));
    assert!(m.has_function("keys"));
    assert!(!m.has_function("clear"));
    assert!(!m.has_function("remove"));
}

#[test]
fn storage_get_returns_capability_call() {
    let m = StorageModule::new();
    assert_capability_call(
        &m,
        "get",
        vec![Value::String("user_prefs".into())],
        CAP_STORAGE,
        STORAGE_GET,
    );
}

#[test]
fn storage_get_wrong_arg_count() {
    let m = StorageModule::new();
    let err = m.call("get", vec![]).unwrap_err();
    assert!(matches!(err, StdlibError::WrongArgCount { .. }));
    let err = m
        .call(
            "get",
            vec![Value::String("a".into()), Value::String("b".into())],
        )
        .unwrap_err();
    assert!(matches!(err, StdlibError::WrongArgCount { .. }));
}

#[test]
fn storage_get_wrong_arg_type() {
    let m = StorageModule::new();
    let err = m.call("get", vec![Value::Number(1.0)]).unwrap_err();
    assert!(matches!(err, StdlibError::TypeMismatch { .. }));
}

#[test]
fn storage_set_returns_capability_call() {
    let m = StorageModule::new();
    assert_capability_call(
        &m,
        "set",
        vec![Value::String("theme".into()), Value::String("dark".into())],
        CAP_STORAGE,
        STORAGE_SET,
    );
}

#[test]
fn storage_set_wrong_arg_count() {
    let m = StorageModule::new();
    let err = m.call("set", vec![Value::String("k".into())]).unwrap_err();
    assert!(matches!(err, StdlibError::WrongArgCount { .. }));
}

#[test]
fn storage_set_wrong_arg_type() {
    let m = StorageModule::new();
    let err = m
        .call("set", vec![Value::String("k".into()), Value::Number(1.0)])
        .unwrap_err();
    assert!(matches!(err, StdlibError::TypeMismatch { .. }));
}

#[test]
fn storage_delete_returns_capability_call() {
    let m = StorageModule::new();
    assert_capability_call(
        &m,
        "delete",
        vec![Value::String("old_key".into())],
        CAP_STORAGE,
        STORAGE_DELETE,
    );
}

#[test]
fn storage_delete_wrong_arg_count() {
    let m = StorageModule::new();
    let err = m.call("delete", vec![]).unwrap_err();
    assert!(matches!(err, StdlibError::WrongArgCount { .. }));
}

#[test]
fn storage_keys_returns_capability_call() {
    let m = StorageModule::new();
    assert_capability_call(&m, "keys", vec![], CAP_STORAGE, STORAGE_KEYS);
}

#[test]
fn storage_keys_wrong_arg_count() {
    let m = StorageModule::new();
    let err = m.call("keys", vec![Value::String("x".into())]).unwrap_err();
    assert!(matches!(err, StdlibError::WrongArgCount { .. }));
}

#[test]
fn storage_unknown_function() {
    let m = StorageModule::new();
    let err = m.call("clear", vec![]).unwrap_err();
    assert!(matches!(err, StdlibError::UnknownFunction { .. }));
}

// ═══════════════════════════════════════════════════════════════════════════
// LOCATION MODULE TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn location_module_name() {
    assert_eq!(LocationModule::new().name(), "location");
}

#[test]
fn location_has_function() {
    let m = LocationModule::new();
    assert!(m.has_function("current"));
    assert!(!m.has_function("watch"));
    assert!(!m.has_function("last"));
}

#[test]
fn location_current_returns_capability_call() {
    let m = LocationModule::new();
    assert_capability_call(&m, "current", vec![], CAP_LOCATION, LOCATION_CURRENT);
}

#[test]
fn location_current_wrong_arg_count() {
    let m = LocationModule::new();
    let err = m.call("current", vec![Value::Number(1.0)]).unwrap_err();
    assert!(matches!(err, StdlibError::WrongArgCount { .. }));
}

#[test]
fn location_unknown_function() {
    let m = LocationModule::new();
    let err = m.call("watch", vec![]).unwrap_err();
    assert!(matches!(err, StdlibError::UnknownFunction { .. }));
}

// ═══════════════════════════════════════════════════════════════════════════
// NOTIFICATIONS MODULE TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn notifications_module_name() {
    assert_eq!(NotificationsModule::new().name(), "notifications");
}

#[test]
fn notifications_has_function() {
    let m = NotificationsModule::new();
    assert!(m.has_function("send"));
    assert!(!m.has_function("schedule"));
    assert!(!m.has_function("cancel"));
}

#[test]
fn notifications_send_returns_capability_call() {
    let m = NotificationsModule::new();
    assert_capability_call(
        &m,
        "send",
        vec![
            Value::String("Reminder".into()),
            Value::String("Time to exercise!".into()),
        ],
        CAP_NOTIFICATIONS,
        NOTIFICATIONS_SEND,
    );
}

#[test]
fn notifications_send_wrong_arg_count() {
    let m = NotificationsModule::new();
    // 1 arg
    let err = m
        .call("send", vec![Value::String("title".into())])
        .unwrap_err();
    assert!(matches!(err, StdlibError::WrongArgCount { .. }));
    // 0 args
    let err = m.call("send", vec![]).unwrap_err();
    assert!(matches!(err, StdlibError::WrongArgCount { .. }));
}

#[test]
fn notifications_send_wrong_arg_type() {
    let m = NotificationsModule::new();
    // first arg not string
    let err = m
        .call(
            "send",
            vec![Value::Number(1.0), Value::String("body".into())],
        )
        .unwrap_err();
    assert!(matches!(err, StdlibError::TypeMismatch { .. }));
    // second arg not string
    let err = m
        .call(
            "send",
            vec![Value::String("title".into()), Value::Bool(true)],
        )
        .unwrap_err();
    assert!(matches!(err, StdlibError::TypeMismatch { .. }));
}

#[test]
fn notifications_unknown_function() {
    let m = NotificationsModule::new();
    let err = m.call("schedule", vec![]).unwrap_err();
    assert!(matches!(err, StdlibError::UnknownFunction { .. }));
}

#[test]
fn notifications_preserves_args() {
    let m = NotificationsModule::new();
    let err = m
        .call(
            "send",
            vec![Value::String("Hello".into()), Value::String("World".into())],
        )
        .unwrap_err();
    match err {
        StdlibError::CapabilityCall { args, .. } => {
            assert_eq!(args.len(), 2);
            assert!(matches!(&args[0], Value::String(s) if s == "Hello"));
            assert!(matches!(&args[1], Value::String(s) if s == "World"));
        }
        _ => panic!("Expected CapabilityCall"),
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// CAPABILITY ID MAPPING TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn resolve_ids_http() {
    assert_eq!(capability::resolve_ids("http", "get"), Some((1, 1)));
    assert_eq!(capability::resolve_ids("http", "post"), Some((1, 2)));
    assert_eq!(capability::resolve_ids("http", "put"), Some((1, 3)));
    assert_eq!(capability::resolve_ids("http", "patch"), Some((1, 4)));
    assert_eq!(capability::resolve_ids("http", "delete"), Some((1, 5)));
}

#[test]
fn resolve_ids_storage() {
    assert_eq!(capability::resolve_ids("storage", "get"), Some((2, 1)));
    assert_eq!(capability::resolve_ids("storage", "set"), Some((2, 2)));
    assert_eq!(capability::resolve_ids("storage", "delete"), Some((2, 3)));
    assert_eq!(capability::resolve_ids("storage", "keys"), Some((2, 4)));
}

#[test]
fn resolve_ids_location() {
    assert_eq!(capability::resolve_ids("location", "current"), Some((3, 1)));
}

#[test]
fn resolve_ids_notifications() {
    assert_eq!(
        capability::resolve_ids("notifications", "send"),
        Some((4, 1))
    );
}

#[test]
fn resolve_ids_unknown() {
    assert_eq!(capability::resolve_ids("math", "abs"), None);
    assert_eq!(capability::resolve_ids("http", "head"), None);
    assert_eq!(capability::resolve_ids("foo", "bar"), None);
}

#[test]
fn is_capability_module_check() {
    assert!(capability::is_capability_module("http"));
    assert!(capability::is_capability_module("storage"));
    assert!(capability::is_capability_module("location"));
    assert!(capability::is_capability_module("notifications"));
    assert!(!capability::is_capability_module("math"));
    assert!(!capability::is_capability_module("core"));
    assert!(!capability::is_capability_module("timer"));
}

#[test]
fn capability_module_names_complete() {
    let names = capability::capability_module_names();
    assert_eq!(names.len(), 4);
    assert!(names.contains(&"http"));
    assert!(names.contains(&"storage"));
    assert!(names.contains(&"location"));
    assert!(names.contains(&"notifications"));
}

// ═══════════════════════════════════════════════════════════════════════════
// DETERMINISM TEST
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn capability_modules_deterministic_100_iterations() {
    // Each capability module call should produce identical CapabilityCall errors
    // across 100 iterations (argument validation + error construction is deterministic).
    let http = HttpModule::new();
    let storage = StorageModule::new();
    let location = LocationModule::new();
    let notifications = NotificationsModule::new();

    let http_args = || vec![Value::String("https://example.com".into())];
    let storage_args = || vec![Value::String("key".into())];
    let notif_args = || vec![Value::String("title".into()), Value::String("body".into())];

    // Capture reference errors
    let ref_http = format!("{}", http.call("get", http_args()).unwrap_err());
    let ref_storage = format!("{}", storage.call("get", storage_args()).unwrap_err());
    let ref_location = format!("{}", location.call("current", vec![]).unwrap_err());
    let ref_notif = format!("{}", notifications.call("send", notif_args()).unwrap_err());

    for i in 0..100 {
        assert_eq!(
            format!("{}", http.call("get", http_args()).unwrap_err()),
            ref_http,
            "http.get not deterministic at iteration {i}"
        );
        assert_eq!(
            format!("{}", storage.call("get", storage_args()).unwrap_err()),
            ref_storage,
            "storage.get not deterministic at iteration {i}"
        );
        assert_eq!(
            format!("{}", location.call("current", vec![]).unwrap_err()),
            ref_location,
            "location.current not deterministic at iteration {i}"
        );
        assert_eq!(
            format!("{}", notifications.call("send", notif_args()).unwrap_err()),
            ref_notif,
            "notifications.send not deterministic at iteration {i}"
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ERROR TYPE MATCHING
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn capability_call_error_display_includes_ids() {
    let m = HttpModule::new();
    let err = m
        .call("get", vec![Value::String("url".into())])
        .unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("cap_id=1"), "Should include cap_id: {msg}");
    assert!(msg.contains("fn_id=1"), "Should include fn_id: {msg}");
    assert!(
        msg.contains("http.get"),
        "Should include module.function: {msg}"
    );
}

#[test]
fn all_capability_functions_return_capability_call_error() {
    // Exhaustive: every function in every capability module returns CapabilityCall
    let http = HttpModule::new();
    let storage = StorageModule::new();
    let location = LocationModule::new();
    let notifications = NotificationsModule::new();

    let s = || Value::String("x".into());

    let calls: Vec<(&dyn StdlibModule, &str, Vec<Value>)> = vec![
        (&http, "get", vec![s()]),
        (&http, "post", vec![s(), s()]),
        (&http, "put", vec![s(), s()]),
        (&http, "patch", vec![s(), s()]),
        (&http, "delete", vec![s()]),
        (&storage, "get", vec![s()]),
        (&storage, "set", vec![s(), s()]),
        (&storage, "delete", vec![s()]),
        (&storage, "keys", vec![]),
        (&location, "current", vec![]),
        (&notifications, "send", vec![s(), s()]),
    ];

    for (module, func, args) in calls {
        let result = module.call(func, args);
        assert!(result.is_err(), "{}.{func} should be Err", module.name());
        assert!(
            matches!(result.unwrap_err(), StdlibError::CapabilityCall { .. }),
            "{}.{func} should be CapabilityCall",
            module.name()
        );
    }
}
