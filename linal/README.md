# linal

N-dimensional arrays with SIMD acceleration, heterogeneous dataframes with null support, and linear algebra operations for machine learning workloads. Built with AVX2/NEON intrinsics for performance on x86_64 and aarch64 architectures.

```
     linal/
     └── src/
         ├── types/          # DataType, Scalar, LinAlError
         ├── simd/           # f32/f64/i32/i64 AVX2/NEON ops
         ├── array/          # Array<T,D>, Array1/2/3
         ├── matrix/         # Matrix<T> using Array2<T>
         ├── series/         # Series<T>, AnySeries, NullBuffer
         ├── dataframe/      # DataFrame, select, filter, groupby, join
         └── io/             # CSV, Arrow (optional)
```

## Data Types

Core numeric types for ML:
- **f32** - Default float type, SIMD-optimized, memory efficient
- **f64** - High precision floats for gradient descent, loss computation
- **i32** - Labels, indices, small counts
- **i64** - Large counts, timestamps

Optional (can defer):
- **bool** - Boolean masks for filtering
- **String** - Categorical features, column names

## Core Structures

### Arrays (array/)
**`Array<T, D>`** - Generic N-dimensional contiguous array with row-major layout  
*Usecase*: Feature tensors, weight matrices, batched inputs  

**`Array1<T>`** - 1D vector  
*Usecase*: Single feature column, model predictions, loss values  

**`Array2<T>`** - 2D matrix  
*Usecase*: Feature matrix (samples × features), weight matrix, batch data  

**`Array3<T>`** - 3D tensor  
*Usecase*: Image batches (batch × height × width), time series windows  

**`Dimension` trait** - Shape and stride computation  
*Usecase*: Indexing, slicing, reshaping without data copy  

### Matrix (matrix/)
**`Matrix<T>`** - 2D matrix with linear algebra operations  
*Usecase*: Matrix multiplication, inverse, decompositions, gradient computation  

### SIMD (simd/)
**`SimdOps` trait** - Vectorized arithmetic operations  
*Usecase*: Fast element-wise ops on arrays (AVX2: 8×f32, NEON: 4×f32)  

### Series (series/)
**`Series<T>`** - 1D labeled array with null support  
*Usecase*: Single feature with missing values, target variable  

**`NullBuffer`** - Bitmap (1 bit per element) for tracking null values  
*Usecase*: Memory-efficient missing data representation  

**`AnySeries` enum** - Type-erased series for heterogeneous collections  
*Usecase*: DataFrame columns with different types  

### DataFrame (dataframe/)
**`DataFrame`** - Columnar 2D table with heterogeneous typed columns  
*Usecase*: Training data with mixed types, feature engineering, data loading  

### Type System (types/)
**`DataType` enum** - Runtime type information  
*Usecase*: Type checking, conversions, DataFrame column metadata  

**`Scalar` enum** - Type-erased single value  
*Usecase*: Scalar operations on heterogeneous data  

**`LinAlError`** - Unified error type  
*Usecase*: Shape mismatches, type errors, bounds checking
