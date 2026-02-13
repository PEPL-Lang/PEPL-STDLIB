# pepl-stdlib

The PEPL standard library — deterministic, pure functions for PEPL programs.

**Status:** Phase 5 complete (all 9 pure stdlib modules). See [ROADMAP.md](ROADMAP.md) for progress.

## Modules

| Module | Functions | Status |
|--------|-----------|--------|
| `core` | 4 (type_of, to_string, print, assert) | ✅ Done |
| `math` | 10 + 2 constants (PI, E) | ✅ Done |
| `string` | 20 (length, concat, contains, slice, trim, split, etc.) | ✅ Done |
| `list` | 31 (construction, access, modification, higher-order, query) | ✅ Done |
| `record` | 5 (get, set, has, keys, values) | ✅ Done |
| `time` | 5 (now, format, diff, day_of_week, start_of_day) | ✅ Done |
| `convert` | 5 (to_string, to_number, parse_int, parse_float, to_bool) | ✅ Done |
| `json` | 2 (parse, stringify) | ✅ Done |
| `timer` | 4 (start, start_once, stop, stop_all) | ✅ Done |

## Tests

450 tests:
- core: 75
- math: 85
- string: 109
- list: 117
- record + time + convert + json + timer: 64

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
