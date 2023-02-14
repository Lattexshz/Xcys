use std::error;
use std::fmt;

#[derive(Clone)]
pub enum ErrorKind {
    Null,
}

impl ErrorKind {
    pub fn description(&self) -> &'static str {
        match self {
            ErrorKind::Null => "No command has been entered.",
        }
    }
}

pub struct CommandParseError {
    _error: _CommandParseError,
}

impl CommandParseError {
    pub fn new<E>(kind: ErrorKind, error: E) -> Self
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        CommandParseError {
            _error: _CommandParseError::Custom((kind, error.into())),
        }
    }

    pub fn simple(kind: ErrorKind) -> Self {
        CommandParseError {
            _error: _CommandParseError::Simple(kind),
        }
    }

    pub fn kind(&self) -> ErrorKind {
        match &self._error {
            _CommandParseError::Simple(s) => s.clone(),
            _CommandParseError::Custom(c) => c.0.clone(),
        }
    }
}

enum _CommandParseError {
    Simple(ErrorKind),
    Custom((ErrorKind, Box<dyn error::Error + Send + Sync>)),
}

impl fmt::Display for CommandParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self._error {
            _CommandParseError::Simple(s) => f.write_str(s.description()),
            _CommandParseError::Custom(c) => f.write_str(c.0.description()),
        }
    }
}

impl fmt::Debug for CommandParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}
