use actix_web::{http::StatusCode, ResponseError};
use std::fmt;

/// error management in rust is so tedious, so let's try gather them all here
/// with a common one. Likely not the correct way but it's to avoid to
/// get lost with all the errors transformation and the needs of multiple
/// external crate that boilerplate.
/// At least will be ny own mess, will see how it goes

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ErrorCode {
    /// record related
    Parsing,
    UnknownKind,
    ValidationFailed,

    /// from dependencies, fail to properly call
    SendRequest,
    Payload,
    Connect,
    Ssl,

    /// from dependencies, successfuly called by returned status code error
    ErrorStatus(u16),
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl ErrorCode {
    pub fn description(&self) -> &'static str {
        match self {
            ErrorCode::Parsing => "fail to parse",
            ErrorCode::UnknownKind => "kind is not unknown",
            ErrorCode::ValidationFailed => "validation failed",
            ErrorCode::SendRequest => "dependency failure",
            ErrorCode::Payload => "dependency failure",
            ErrorCode::Connect => "dependency failure: cannot connect",
            ErrorCode::Ssl => "dependency failure",
            ErrorCode::ErrorStatus(_) => "dependency error",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Dependency {
    CoreStorage,
    Unknown,
}

impl fmt::Display for Dependency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Dependency::CoreStorage => write!(f, "core storage"),
            Dependency::Unknown => write!(f, "N/A"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ErrorCategory {
    Record,
    Dependency(Dependency),
}

/// basically we just want a code/category and a description ... may be a source to understand the origin
#[derive(Debug)]
pub struct WDMSError {
    pub category: ErrorCategory,
    pub code: ErrorCode,
    pub description: String,
    pub source: Option<Box<dyn std::error::Error>>,
}

impl WDMSError {
    pub fn from_record(
        code: ErrorCode,
        description: String,
        source: Option<Box<dyn std::error::Error>>,
    ) -> Self {
        Self {
            category: ErrorCategory::Record,
            code,
            description,
            source,
        }
    }

    pub fn from_validation(
        description: String,
        source: Option<Box<dyn std::error::Error>>,
    ) -> Self {
        Self {
            category: ErrorCategory::Record,
            code: ErrorCode::ValidationFailed,
            description,
            source,
        }
    }
}

impl From<serde_json::error::Error> for WDMSError {
    fn from(value: serde_json::error::Error) -> Self {
        WDMSError::from_record(ErrorCode::Parsing, value.to_string(), Some(Box::new(value)))
    }
}

impl fmt::Display for WDMSError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.category {
            ErrorCategory::Record => {
                write!(
                    f,
                    "Error on the record, {}: {}",
                    self.code, self.description
                )
            }
            ErrorCategory::Dependency(d) => {
                write!(f, "{} error: {}", d, self.description)
            }
        }
    }
}

impl std::error::Error for WDMSError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.source.as_ref() {
            None => None,
            Some(err) => Some(err.as_ref()),
        }
    }
}

/// so actix_web can construct an http response from WDMSError
impl ResponseError for WDMSError {
    /// Record category error will always produce a 422
    /// in case of dependency with error status, this one is directly forwarded
    /// any other case will produce a 500 internal server error
    fn status_code(&self) -> StatusCode {
        match self.category {
            ErrorCategory::Record => StatusCode::UNPROCESSABLE_ENTITY,
            ErrorCategory::Dependency(_) => match self.code {
                ErrorCode::ErrorStatus(c) => {
                    StatusCode::from_u16(c).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
                }
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
        }
    }
}
