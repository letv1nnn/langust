use std::{error::Error, fmt::Display};

#[derive(Debug, Copy, Clone)]
pub enum MatrixError {
    ShapeMismatch,
}

impl Display for MatrixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatrixError::ShapeMismatch => write!(f, "matrix dimensions do not match"),
        }
    }
}

impl Error for MatrixError {}
