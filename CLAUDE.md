# Langust

Rust workspace for data structures, SIMD-accelerated numerics, and (planned) ML primitives.

## Workspace Crates

| Crate | Path | Purpose |
|-------|------|---------|
| `linal` | `linal/` | Core library — arrays, series, dataframes, SIMD ops, IO |
| `monumel` | `monumel/` | ML crate (placeholder, not yet implemented) |
| `exmpls` | `exmpls/` | Example binaries consuming `linal` |

## Architecture

### linal modules

```
linal/src/
├── array/          # Columnar data primitives
│   ├── mod.rs
│   └── null.rs     # NullBuffer — bit-packed null tracking (1=null, 0=valid)
├── series/         # Series type (not yet implemented)
├── dataframe/      # DataFrame type (not yet implemented)
├── io/             # IO utilities (not yet implemented)
├── simd/           # SIMD-accelerated arithmetic (crate-internal)
│   ├── f32.rs      # f32 SIMD: AVX2 (x86_64), NEON (aarch64), scalar fallback
│   ├── traits.rs   # SimdOps, ArithmeticOperation
│   └── errors.rs   # SimdError, SimdResult
├── traits.rs       # LinAlg trait (matmul, dot, transpose — placeholder)
└── lib.rs
```

### Key design decisions

- **NullBuffer** uses bit-packing (1 bit per element, `Vec<u8>` backing). Bit 1 = null, bit 0 = valid.
- **SIMD** dispatches at compile time via `#[cfg(target_arch)]` + `#[target_feature]`. Processes 8 floats/iteration (AVX2) or 4 (NEON), scalar tail handles remainder.
- `simd` module is `pub(crate)` — internal only, not part of public API.
- `linal` allows `unsafe_code` (needed for SIMD intrinsics). Other crates use workspace lint defaults (`unsafe_code = "warn"`).
- Edition 2024 across all crates.

## Build & Test

```bash
cargo build              # Build all crates
cargo test               # Run all tests
cargo test -p linal      # Test linal only
cargo clippy             # Lint (workspace warns on clippy::all)
```

Tests use `-- --nocapture` for debug output where needed.

## CI

GitHub Actions (`.github/workflows/build.yml`): builds + tests + clippy on `stable`, `beta`, `nightly` toolchains. Runs on every push and PR.

## Linker Config

`.cargo/config.toml` configures LLD linker for faster linking on Linux/Windows. macOS LLD config is commented out — uncomment if `brew install llvm` is set up.

## Conventions

- No external dependencies in `linal` (zero deps).
- Tests live in-file as `#[cfg(test)]` modules.
- SIMD implementations follow pattern: arch-specific dispatch function → scalar tail fallback.
- `NullBuffer` constructors: `From<Vec<bool>>` (true=null), `From<Vec<u8>>` (0=null), `FromIterator<bool>`, `FromIterator<u8>`.

## Status

Work in progress. Implemented so far:
- `NullBuffer` — complete with full test coverage
- `simd::f32` — complete with full test coverage (add/sub/mul/div, in-place variants)
- Series, DataFrame, IO, LinAlg traits — stubs only
- `monumel` — not started
