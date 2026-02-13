# ROADMAP — pepl-stdlib (Standard Library)

> 88 Phase 0 functions across 9 core modules + 4 capability modules.
> All core functions are pure, deterministic, and execute in < 1ms.
> Written in Rust, compiled alongside the compiler.

> See `ORCHESTRATION.md` for cross-repo sequencing.

---

## Phase 1: Project Scaffolding & Core Module

### 1.1 Cargo Project Setup
- [x] Create Cargo library crate `pepl-stdlib`
- [x] Configure dependencies: `thiserror`, `serde`, `serde_json`
- [x] Define `StdlibModule` trait (module name, function lookup, call dispatch)
- [x] Define `Value` enum (Number, String, Bool, Nil, List, Record, Color, Result, SumVariant)\n- [x] Record variant carries optional `type_name` for named record types\n- [x] SumVariant carries `type_name`, `variant`, positional `fields`
- [x] Define `StdlibError` type
- [x] Workspace-level `cargo build` succeeds

### 1.2 `core` Module (4 functions)
- [x] `core.log(value: any) -> nil` — debug output, no-op in production
- [x] `core.assert(condition: bool, message?: string) -> nil` — trap if false
- [x] `core.type_of(value: any) -> string` — returns type name
- [x] `core.capability(name: string) -> bool` — checks optional capability availability
- [x] Unit tests for all 4 functions (normal, edge, error cases)
- [x] 100-iteration determinism test

> **API FREEZE:** After Phase 1, the `Value` enum's public variants and the `StdlibModule` trait signature are **frozen**. All subsequent phases (math, string, list, etc.) add modules — they do not change `Value` or `StdlibModule`. This stability is critical because `pepl-eval` (C6) depends on these types.\n>\n> **Note:** `Function`, `ActionRef`, and `Surface` variants are NOT in `Value` — they live in `EvalValue` (pepl-eval), keeping the stdlib free of AST/evaluator dependencies.

---

## Phase 2: `math` Module

### 2.1 Math Functions (10 functions + 2 constants)
- [x] `math.abs(a: number) -> number`
- [x] `math.min(a: number, b: number) -> number`
- [x] `math.max(a: number, b: number) -> number`
- [x] `math.floor(a: number) -> number`
- [x] `math.ceil(a: number) -> number`
- [x] `math.round(a: number) -> number` — 0.5 rounds up
- [x] `math.round_to(a: number, decimals: number) -> number`
- [x] `math.pow(base: number, exp: number) -> number`
- [x] `math.clamp(value: number, min: number, max: number) -> number`
- [x] `math.sqrt(a: number) -> number` — trap on negative input (NaN prevention)
- [x] `math.PI` constant (3.14159265358979...)
- [x] `math.E` constant (2.71828182845904...)
- [x] NaN prevention: all operations that would produce NaN trap instead
- [x] Unit tests for all functions (normal, edge, NaN cases)
- [x] 100-iteration determinism test

---

## Phase 3: `string` Module

### 3.1 String Functions (20 functions)
- [x] `string.length(s) -> number`
- [x] `string.concat(a, b) -> string`
- [x] `string.contains(haystack, needle) -> bool`
- [x] `string.slice(s, start, end) -> string`
- [x] `string.trim(s) -> string`
- [x] `string.split(s, delimiter) -> list<string>`
- [x] `string.to_upper(s) -> string`
- [x] `string.to_lower(s) -> string`
- [x] `string.starts_with(s, prefix) -> bool`
- [x] `string.ends_with(s, suffix) -> bool`
- [x] `string.replace(s, old, new) -> string` — first occurrence only
- [x] `string.replace_all(s, old, new) -> string`
- [x] `string.pad_start(s, length, pad) -> string`
- [x] `string.pad_end(s, length, pad) -> string`
- [x] `string.repeat(s, count) -> string`
- [x] `string.join(items: list<string>, separator) -> string`
- [x] `string.format(template, values: record) -> string` — `{key}` placeholders
- [x] `string.from(value: any) -> string`
- [x] `string.is_empty(s) -> bool`
- [x] `string.index_of(s, sub) -> number` — returns -1 if not found
- [x] Unit tests for all functions (empty strings, Unicode, multi-byte)
- [x] 100-iteration determinism test

---

## Phase 4: `list` Module

> **Note:** The canonical set of 31 list functions is defined by the compiler's
> type-checker registrations (`pepl-compiler/src/stdlib.rs`). This phase
> implements exactly those 31 functions.

### 4.1 List Construction (4 functions)
- [x] `list.empty() -> list`
- [x] `list.of(...items) -> list` — variadic
- [x] `list.repeat(value, count) -> list`
- [x] `list.range(start, end) -> list<number>` — start inclusive, end exclusive

### 4.2 List Access (5 functions)
- [x] `list.length(items) -> number`
- [x] `list.get(items, index) -> any|nil` — returns nil on out-of-bounds
- [x] `list.first(items) -> any|nil`
- [x] `list.last(items) -> any|nil`
- [x] `list.index_of(items, value) -> number` — returns -1 if not found

### 4.3 List Modification (10 functions)
- [x] `list.append(items, value) -> list`
- [x] `list.prepend(items, value) -> list`
- [x] `list.insert(items, index, value) -> list`
- [x] `list.remove(items, index) -> list`
- [x] `list.update(items, index, value) -> list`
- [x] `list.slice(items, start, end) -> list`
- [x] `list.concat(a, b) -> list`
- [x] `list.reverse(items) -> list`
- [x] `list.flatten(items) -> list` — one level deep
- [x] `list.unique(items) -> list` — preserves first occurrence
- [x] All operations return NEW lists (immutable)

### 4.4 List Higher-Order (9 functions)
- [x] `list.map(items, f) -> list`
- [x] `list.filter(items, pred) -> list`
- [x] `list.reduce(items, init, f) -> any`
- [x] `list.find(items, pred) -> any|nil`
- [x] `list.find_index(items, pred) -> number` — returns -1 if not found
- [x] `list.every(items, pred) -> bool`
- [x] `list.some(items, pred) -> bool`
- [x] `list.sort(items, compare) -> list` — stable, deterministic
- [x] `list.count(items, pred) -> number`
- [x] Added `Value::Function(StdlibFn)` variant for callback support

### 4.5 List Query (3 functions)
- [x] `list.contains(items, value) -> bool`
- [x] `list.zip(a, b) -> list<{first, second}>` — stops at shorter list
- [x] `list.take(items, n) -> list`

### 4.6 Testing & Validation
- [x] Unit tests for all 31 functions (normal, edge, error cases)
- [x] Higher-order tests with real callback functions
- [x] Integration tests (chaining filter→map, range→reduce, etc.)
- [x] Sort stability & comparator error propagation tests
- [x] 100-iteration determinism test
- [x] `cargo clippy` clean
- [x] 117 list module tests, 386 total crate tests

---

## Phase 5: `record`, `time`, `convert`, `json`, `timer` Modules

### 5.1 `record` Module (5 functions)
- [x] `record.get(r, key) -> any` — returns Nil if key missing
- [x] `record.set(r, key, value) -> record` — immutable, returns new record
- [x] `record.has(r, key) -> bool`
- [x] `record.keys(r) -> list<string>` — deterministic BTreeMap order
- [x] `record.values(r) -> list<any>` — same deterministic order
- [x] Unit tests (12 tests)

### 5.2 `time` Module (5 functions)
- [x] `time.now() -> number` — deterministic stub (returns 0); host injects real value at runtime
- [x] `time.format(timestamp, pattern) -> string` — "YYYY-MM-DD", "HH:mm:ss", etc. via placeholder replacement
- [x] `time.diff(a, b) -> number` — difference in milliseconds (a - b)
- [x] `time.day_of_week(timestamp) -> number` — 0=Sunday through 6=Saturday
- [x] `time.start_of_day(timestamp) -> number` — truncate to midnight UTC
- [x] Unit tests with known timestamps (12 tests)
- [x] Civil calendar algorithm (Howard Hinnant's) — no external deps

### 5.3 `convert` Module (5 functions)
- [x] `convert.to_string(value: any) -> string` — always succeeds, uses Display impl
- [x] `convert.to_number(value: any) -> Result<number, string>` — String→parse, Bool→0/1, Number→identity
- [x] `convert.parse_int(s: string) -> Result<number, string>` — rejects floats
- [x] `convert.parse_float(s: string) -> Result<number, string>` — rejects NaN/Infinity
- [x] `convert.to_bool(value: any) -> bool` — truthiness (false/nil/0/"" are falsy)
- [x] Unit tests (20 tests)

### 5.4 `json` Module (2 functions)
- [x] `json.parse(s: string) -> Result<any, string>` — max depth: 32
- [x] `json.stringify(value: any) -> string` — handles all Value variants
- [x] JSON↔Value type mapping: null↔Nil, bool↔Bool, number↔Number, string↔String, array↔List, object↔Record
- [x] Roundtrip test (stringify→parse preserves structure)
- [x] Unit tests (13 tests)

### 5.5 `timer` Module (4 functions)
- [x] `timer.start(id, interval_ms) -> string` — host-delegated stub, returns timer ID
- [x] `timer.start_once(id, delay_ms) -> string` — one-shot, returns timer ID
- [x] `timer.stop(id) -> nil` — no-op if invalid
- [x] `timer.stop_all() -> nil`
- [x] Unit tests (7 tests)
- [x] 100-iteration determinism test for all Phase 5 modules
- [x] `cargo clippy` clean (library)
- [x] 64 Phase 5 tests, 450 total crate tests

---

## Phase 6: Capability Modules

### 6.1 `http` Module (5 functions)
- [ ] `http.get(url, options?) -> Result<HttpResponse, HttpError>`
- [ ] `http.post(url, body, options?) -> Result<HttpResponse, HttpError>`
- [ ] `http.put(url, body, options?) -> Result<HttpResponse, HttpError>`
- [ ] `http.patch(url, body, options?) -> Result<HttpResponse, HttpError>`
- [ ] `http.delete(url, options?) -> Result<HttpResponse, HttpError>`
- [ ] Define `HttpOptions`, `HttpResponse`, `HttpError` types
- [ ] All calls yield to host via `env.host_call` (cap_id=1)
- [ ] Unit tests with mocked host responses

### 6.2 `storage` Module (4 functions)
- [ ] `storage.get(key) -> Result<string, StorageError>`
- [ ] `storage.set(key, value) -> Result<nil, StorageError>`
- [ ] `storage.delete(key) -> Result<nil, StorageError>`
- [ ] `storage.keys() -> Result<list<string>, StorageError>`
- [ ] All calls yield to host via `env.host_call` (cap_id=2)
- [ ] Unit tests with mocked host responses

### 6.3 `location` Module (1 function)
- [ ] `location.current() -> Result<{lat: number, lon: number}, LocationError>`
- [ ] Yields to host via `env.host_call` (cap_id=3)
- [ ] Unit tests

### 6.4 `notifications` Module (1 function)
- [ ] `notifications.send(title, body) -> Result<nil, NotificationError>`
- [ ] Yields to host via `env.host_call` (cap_id=4)
- [ ] Unit tests

### 6.5 Final Validation
- [ ] All 88 Phase 0 functions implemented and tested
- [ ] Every function executes in < 1ms
- [ ] All error types match spec: HttpError, JsonError, StorageError, LocationError, NotificationError, ConvertError
- [ ] Full 100-iteration determinism test across all modules
- [ ] `cargo clippy -- -D warnings` clean
- [ ] README.md with module reference and architecture overview
