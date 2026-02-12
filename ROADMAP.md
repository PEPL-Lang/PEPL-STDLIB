# ROADMAP — pepl-stdlib (Standard Library)

> 88 Phase 0 functions across 9 core modules + 4 capability modules.
> All core functions are pure, deterministic, and execute in < 1ms.
> Written in Rust, compiled alongside the compiler.

---

## Phase 1: Project Scaffolding & Core Module

### 1.1 Cargo Project Setup
- [ ] Create Cargo library crate `pepl-stdlib`
- [ ] Configure dependencies: `thiserror`, `serde`, `serde_json`
- [ ] Define `StdlibModule` trait (module name, function lookup, call dispatch)
- [ ] Define `Value` enum (Number, String, Bool, Nil, List, Record, Color, Result)
- [ ] Define `StdlibError` type
- [ ] Workspace-level `cargo build` succeeds

### 1.2 `core` Module (4 functions)
- [ ] `core.log(value: any) -> nil` — debug output, no-op in production
- [ ] `core.assert(condition: bool, message?: string) -> nil` — trap if false
- [ ] `core.type_of(value: any) -> string` — returns type name
- [ ] `core.capability(name: string) -> bool` — checks optional capability availability
- [ ] Unit tests for all 4 functions (normal, edge, error cases)
- [ ] 100-iteration determinism test

---

## Phase 2: `math` Module

### 2.1 Math Functions (10 functions + 2 constants)
- [ ] `math.abs(a: number) -> number`
- [ ] `math.min(a: number, b: number) -> number`
- [ ] `math.max(a: number, b: number) -> number`
- [ ] `math.floor(a: number) -> number`
- [ ] `math.ceil(a: number) -> number`
- [ ] `math.round(a: number) -> number` — 0.5 rounds up
- [ ] `math.round_to(a: number, decimals: number) -> number`
- [ ] `math.pow(base: number, exp: number) -> number`
- [ ] `math.clamp(value: number, min: number, max: number) -> number`
- [ ] `math.sqrt(a: number) -> number` — trap on negative input (NaN prevention)
- [ ] `math.PI` constant (3.14159265358979...)
- [ ] `math.E` constant (2.71828182845904...)
- [ ] NaN prevention: all operations that would produce NaN trap instead
- [ ] Unit tests for all functions (normal, edge, NaN cases)
- [ ] 100-iteration determinism test

---

## Phase 3: `string` Module

### 3.1 String Functions (20 functions)
- [ ] `string.length(s) -> number`
- [ ] `string.concat(a, b) -> string`
- [ ] `string.contains(haystack, needle) -> bool`
- [ ] `string.slice(s, start, end) -> string`
- [ ] `string.trim(s) -> string`
- [ ] `string.split(s, delimiter) -> list<string>`
- [ ] `string.to_upper(s) -> string`
- [ ] `string.to_lower(s) -> string`
- [ ] `string.starts_with(s, prefix) -> bool`
- [ ] `string.ends_with(s, suffix) -> bool`
- [ ] `string.replace(s, old, new) -> string` — first occurrence only
- [ ] `string.replace_all(s, old, new) -> string`
- [ ] `string.pad_start(s, length, pad) -> string`
- [ ] `string.pad_end(s, length, pad) -> string`
- [ ] `string.repeat(s, count) -> string`
- [ ] `string.join(items: list<string>, separator) -> string`
- [ ] `string.format(template, values: record) -> string` — `{key}` placeholders
- [ ] `string.from(value: any) -> string`
- [ ] `string.is_empty(s) -> bool`
- [ ] `string.index_of(s, sub) -> number` — returns -1 if not found
- [ ] Unit tests for all functions (empty strings, Unicode, multi-byte)
- [ ] 100-iteration determinism test

---

## Phase 4: `list` Module

### 4.1 List Construction & Query (12 functions)
- [ ] `list.empty() -> list<T>`
- [ ] `list.of(...items) -> list<T>` — compiler-special-cased variadic
- [ ] `list.range(start, end) -> list<number>` — start inclusive, end exclusive
- [ ] `list.repeat(item, count) -> list<T>`
- [ ] `list.length(l) -> number`
- [ ] `list.is_empty(l) -> bool`
- [ ] `list.contains(l, item) -> bool`
- [ ] `list.index_of(l, item) -> number` — returns -1 if not found
- [ ] `list.first(l) -> T | nil`
- [ ] `list.last(l) -> T | nil`
- [ ] `list.get(l, index) -> T | nil` — returns nil on out-of-bounds
- [ ] Unit tests for all query functions

### 4.2 List Transformation (12 functions)
- [ ] `list.append(l, item) -> list<T>`
- [ ] `list.prepend(l, item) -> list<T>`
- [ ] `list.set(l, index, item) -> list<T>`
- [ ] `list.remove(l, index) -> list<T>`
- [ ] `list.concat(a, b) -> list<T>`
- [ ] `list.reverse(l) -> list<T>`
- [ ] `list.sort(l, comparator) -> list<T>` — stable, deterministic sorting
- [ ] `list.sort_by(l, key) -> list<T>` — string keys: lexicographic, case-sensitive
- [ ] `list.unique(l) -> list<T>` — preserves first occurrence
- [ ] `list.slice(l, start, end) -> list<T>`
- [ ] `list.take(l, count) -> list<T>`
- [ ] `list.drop(l, count) -> list<T>`
- [ ] All operations return NEW lists (immutable)
- [ ] Unit tests for all transformation functions

### 4.3 List Higher-Order (7 functions)
- [ ] `list.map(l, fn) -> list<U>`
- [ ] `list.filter(l, fn) -> list<T>`
- [ ] `list.reduce(l, init, fn) -> U`
- [ ] `list.find(l, fn) -> T | nil`
- [ ] `list.flat_map(l, fn) -> list<U>`
- [ ] `list.any(l, fn) -> bool`
- [ ] `list.every(l, fn) -> bool`
- [ ] `list.count(l, fn) -> number`
- [ ] Unit tests for all higher-order functions
- [ ] Sort stability test: equal-key elements preserve order
- [ ] 100-iteration determinism test for entire list module

---

## Phase 5: `record`, `time`, `convert`, `json`, `timer` Modules

### 5.1 `record` Module (5 functions)
- [ ] `record.get(r, key) -> any`
- [ ] `record.set(r, key, value) -> record`
- [ ] `record.has(r, key) -> bool`
- [ ] `record.keys(r) -> list<string>`
- [ ] `record.values(r) -> list<any>`
- [ ] Unit tests

### 5.2 `time` Module (5 functions)
- [ ] `time.now() -> number` — host-provided timestamp (deterministic on replay)
- [ ] `time.format(timestamp, pattern) -> string` — "YYYY-MM-DD", "HH:mm", etc.
- [ ] `time.diff(a, b) -> number` — difference in milliseconds
- [ ] `time.day_of_week(timestamp) -> number` — 0=Sunday through 6=Saturday
- [ ] `time.start_of_day(timestamp) -> number`
- [ ] Unit tests (deterministic with injected timestamps)

### 5.3 `convert` Module (5 functions)
- [ ] `convert.to_string(value: any) -> string` — always succeeds
- [ ] `convert.to_number(value: any) -> Result<number, ConvertError>`
- [ ] `convert.parse_int(s: string) -> Result<number, ConvertError>`
- [ ] `convert.parse_float(s: string) -> Result<number, ConvertError>`
- [ ] `convert.to_bool(value: any) -> bool` — truthy conversion
- [ ] Unit tests (valid inputs, invalid inputs, Result handling)

### 5.4 `json` Module (2 functions)
- [ ] `json.parse(s: string) -> Result<any, JsonError>` — max depth: 32
- [ ] `json.stringify(value: any) -> string`
- [ ] Unit tests (valid JSON, invalid JSON, depth limit, type mapping)

### 5.5 `timer` Module (4 functions)
- [ ] `timer.start(action_name, interval_ms) -> string` — recurring timer
- [ ] `timer.start_once(action_name, delay_ms) -> string` — one-shot timer
- [ ] `timer.stop(timer_id) -> nil` — no-op if invalid
- [ ] `timer.stop_all() -> nil`
- [ ] Unit tests
- [ ] 100-iteration determinism test for all Phase 5 modules

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
