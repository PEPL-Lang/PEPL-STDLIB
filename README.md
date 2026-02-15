# pepl-stdlib

[![crates.io](https://img.shields.io/crates/v/pepl-stdlib.svg)](https://crates.io/crates/pepl-stdlib)
[![docs.rs](https://docs.rs/pepl-stdlib/badge.svg)](https://docs.rs/pepl-stdlib)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

The PEPL standard library — deterministic, pure functions for PEPL programs.

**Status:** Phase 7 complete (all 9 pure modules + 4 capability modules, 100 functions + 2 constants). See [ROADMAP.md](ROADMAP.md) for progress.

## Pure Modules

| Module | Functions | Status |
|--------|-----------|--------|
| `core` | 4 (log, assert, type_of, capability) | ✅ Done |
| `math` | 10 + 2 constants (PI, E) | ✅ Done |
| `string` | 20 (length, concat, contains, slice, trim, split, etc.) | ✅ Done |
| `list` | 34 (construction, access, modification, higher-order, query) | ✅ Done |
| `record` | 5 (get, set, has, keys, values) | ✅ Done |
| `time` | 5 (now, format, diff, day_of_week, start_of_day) | ✅ Done |
| `convert` | 5 (to_string, to_number, parse_int, parse_float, to_bool) | ✅ Done |
| `json` | 2 (parse, stringify) | ✅ Done |
| `timer` | 4 (start, start_once, stop, stop_all) | ✅ Done |

## Capability Modules

| Module | Functions | cap_id | Status |
|--------|-----------|--------|--------|
| `http` | 5 (get, post, put, patch, delete) | 1 | ✅ Done |
| `storage` | 4 (get, set, delete, keys) | 2 | ✅ Done |
| `location` | 1 (current) | 3 | ✅ Done |
| `notifications` | 1 (send) | 4 | ✅ Done |

## Tests

512 tests:
- core: 75
- math: 85
- string: 109
- list: 117
- record + time + convert + json + timer: 64
- capability (http + storage + location + notifications): 50
- integration: 1

## Key Design Choices

- **Deterministic:** No floating-point surprises — NaN traps, "0.5 rounds up"
- **Unicode-correct:** String indexing by Unicode grapheme clusters
- **Gas metering:** `CallContext` tracks compute budget
- **Value enum:** Supports Number, String, Bool, Nil, List, Record, SumVariant, Function

## Build

```bash
source "$HOME/.cargo/env"
cargo build
cargo test
cargo clippy -- -D warnings
```

## Cross-Repo Coordination

Part of the PEPL project alongside [`pepl`](https://github.com/PEPL-Lang/PEPL) (compiler) and [`pepl-ui`](https://github.com/PEPL-Lang/PEPL-UI) (UI components).

## License

MIT
