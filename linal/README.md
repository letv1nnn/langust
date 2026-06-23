# linal

Columnar array library with SIMD acceleration for ML. One array primitive serves both DataFrame columns and numerical compute. AVX2 (x86_64) / NEON (aarch64).

## ML Pipeline

```
CSV → DataFrame → wrangle → .to_contiguous() → LinAlg ops → DataFrame
```

## Build Order

| Phase | Module | Depends On |
|-------|--------|------------|
| 1 | NullBuffer | — |
| 2 | PrimitiveArray\<T\> | NullBuffer, SimdOps |
| 3 | BooleanArray, StringArray | NullBuffer |
| 4 | Series\<T\>, AnySeries | PrimitiveArray |
| 5 | DataFrame | Series |
| 6 | LinAlg trait | PrimitiveArray, SimdOps |
| 7 | IO | DataFrame |

## Types

| Type | SIMD | Use Case |
|------|------|----------|
| f32 | yes | Features, weights, predictions |
| f64 | no | Gradients, loss |

## Testing

Run all tests:
```
cargo test --release
```

Run all tests with output:
```
cargo test --release -- --nocapture
```

## Examples

There is a single example for now, which is `simple`.
```sh
cargo run --example simple
```
