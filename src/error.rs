use std::fmt::{Display, Formatter};

/// Errors shared across the forensic recovery components.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoveryError {
    InvalidArgument(String),
    InvalidRange {
        offset: u64,
        length: u64,
    },
    OutOfBounds {
        offset: u64,
        length: u64,
        source_size: u64,
    },
    Io(String),
    Unsupported(String),
    IntegrityViolation(String),
}

impl Display for RecoveryError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidArgument(message) => {
                write!(formatter, "invalid argument: {message}")
            }
            Self::InvalidRange { offset, length } => {
                write!(formatter, "invalid range: offset={offset}, length={length}")
            }
            Self::OutOfBounds {
                offset,
                length,
                source_size,
            } => write!(
                formatter,
                "range out of bounds: offset={offset}, length={length}, source_size={source_size}"
            ),
            Self::Io(message) => write!(formatter, "I/O error: {message}"),
            Self::Unsupported(message) => {
                write!(formatter, "unsupported operation: {message}")
            }
            Self::IntegrityViolation(message) => {
                write!(formatter, "integrity violation: {message}")
            }
        }
    }
}

impl std::error::Error for RecoveryError {}

pub type RecoveryResult<T> = Result<T, RecoveryError>;
