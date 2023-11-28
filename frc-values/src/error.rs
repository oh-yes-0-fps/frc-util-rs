use thiserror::Error;

use crate::FrcType;

#[derive(Debug, Clone)]
pub enum CastErrorReason {
    Type,
    Overflow,
    Underflow
}


#[derive(Debug, Clone, Error)]
pub enum FrcValueError {
    #[error("Could not cast {0} variant to {1} type ({2:?})")]
    InvalidCast(FrcType, &'static str, CastErrorReason),
    #[error("Could not represent the casted data as an FrcValue")]
    UnrepresentableCast
}