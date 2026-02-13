# pepl-stdlib

The PEPL standard library — deterministic, pure functions for PEPL programs.

**Status:** Phase 3 complete (core + math + string). See [ROADMAP.md](ROADMAP.md) for progress.

## Modules

| Module | Functions | Status |
|--------|-----------|--------|
| `core` | 4 (type_of, to_string, print, assert) | ✅ Done |
| `math` | 10 + 2 constants (PI, E) | ✅ Done |
| `string` | 20 (length, concat, contains, slice, trim, split, etc.) | ✅ Done |
| `list` | 31 (construction, transformation, higher-order) | Planned (Phase 4) |
| `record` | 5 | Planned (Phase 5) |
| `time` | 5 | Planned (Phase 5) |
| `convert` | 5 | Planned (Phase 5) |
| `json` | 2 | Planned (Phase 5) |
| `timer` | 4 (capability) | Planned (Phase 6) |

## Tests

269 tests:
- core: 75
- math: 85
- string: 109

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
