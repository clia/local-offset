//! Custom error type.

use time::error::ComponentRange;
use tz::TzError;

/// Define error type.
#[derive(Debug)]
pub enum LocalOffsetError {
    TzError(TzError),
    ComponentRange(ComponentRange),
}

impl std::error::Error for LocalOffsetError {}

impl From<TzError> for LocalOffsetError {
    fn from(err: TzError) -> Self {
        Self::TzError(err)
    }
}

impl From<ComponentRange> for LocalOffsetError {
    fn from(value: ComponentRange) -> Self {
        Self::ComponentRange(value)
    }
}

impl std::fmt::Display for LocalOffsetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LocalOffsetError::TzError(err) => write!(f, "{}", err),
            LocalOffsetError::ComponentRange(err) => write!(f, "{}", err),
        }
    }
}
