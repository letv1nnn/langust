use std::{error::Error, fmt::Display};

pub(crate) type SimdResult = Result<(), SimdError>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum SimdError {
    LengthMismatch { expected: usize, got: usize },
}

impl Display for SimdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LengthMismatch { expected, got } => {
                write!(f, "length mismatch: expected {}, got {}", expected, got)
            }
        }
    }
}

impl Error for SimdError {}

// utility functions
pub(crate) const fn check_for_length_mismatch<T>(a: &[T], b: &[T]) -> SimdResult {
    if a.len() != b.len() {
        return Err(SimdError::LengthMismatch {
            expected: a.len(),
            got: b.len(),
        });
    }
    Ok(())
}
