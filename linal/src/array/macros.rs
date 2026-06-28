#[macro_export]
macro_rules! impl_array_op {
    ($trait:ident, $method:ident, $slice_fn:ident) => {
        impl<T: ArrayElement + SliceArith> $trait for Array<T> {
            type Output = Self;
            fn $method(self, rhs: Array<T>) -> Self::Output {
                assert_eq!(self.data.len(), rhs.data.len(), "array length mismatch");
                let mut data = vec![T::default(); self.data.len()];
                T::$slice_fn(&self.data, &rhs.data, &mut data);
                let nulls = self.nulls.union(&rhs.nulls);
                Self { data, nulls }
            }
        }

        impl<T: ArrayElement + SliceArith> $trait<&Array<T>> for &Array<T> {
            type Output = Array<T>;
            fn $method(self, rhs: &Array<T>) -> Self::Output {
                assert_eq!(self.data.len(), rhs.data.len(), "array length mismatch");
                let mut data = vec![T::default(); self.data.len()];
                T::$slice_fn(&self.data, &rhs.data, &mut data);
                let nulls = self.nulls.union(&rhs.nulls);
                Array { data, nulls }
            }
        }
    };
}
