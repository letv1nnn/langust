# linal

Columnar array library with SIMD acceleration for ML. One array primitive serves both DataFrame columns and numerical compute. AVX2 (x86_64) / NEON (aarch64).

## Architecture

```
┌─────────────────────────────────────────────────┐                                                   
│                     IO                          │                                                   
│              CSV, Arrow IPC                     │                                                   
├─────────────────────────────────────────────────┤                                                   
│                  DataFrame                      │                                                   
│       select, filter, groupby, join             │                                                   
├──────────────────────┬──────────────────────────┤                                                   
│       Series<T>      │      LinAlg trait        │                                                   
│   AnySeries enum     │  matmul, dot, transpose  │                                                   
├──────────────────────┴──────────────────────────┤                                                   
│              Typed Arrays                       │                                                   
│              PrimitiveArray<T>                  │                                                   
├─────────────────────────────────────────────────┤                                                   
│                SIMD Engine                      │                                                   
│     SimdOps trait — f32/f64/i32/i64             │                                                   
│         AVX2 (x86_64) / NEON (aarch64)          │                                                   
└─────────────────────────────────────────────────┘  
```

**LinAlg trait** — matmul, dot, transpose — implemented for anything exposing `&[f32]` + shape. No standalone Matrix type.

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
