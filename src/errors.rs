use std::borrow::Cow;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    SystemTime(std::time::SystemTimeError),
    ParseInt(std::num::ParseIntError),
    Context(Cow<'static, str>, Box<Self>),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "{e}"),
            Self::SystemTime(e) => write!(f, "{e}"),
            Self::ParseInt(e) => write!(f, "{e}"),
            Self::Context(msg, e) => write!(f, "{msg}: {e}"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::time::SystemTimeError> for Error {
    fn from(e: std::time::SystemTimeError) -> Self {
        Self::SystemTime(e)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::ParseInt(e)
    }
}

// ============================================================================
// Context Trait
// ============================================================================
pub trait Context<T> {
    fn context(self, msg: impl Into<Cow<'static, str>>) -> Result<T, Error>;

    fn with_context<M, F>(self, f: F) -> Result<T, Error>
    where
        M: Into<Cow<'static, str>>,
        F: FnOnce() -> M;
}

impl<T, E: Into<Error>> Context<T> for Result<T, E> {
    fn context(self, msg: impl Into<Cow<'static, str>>) -> Result<T, Error> {
        self.map_err(|e| Error::Context(msg.into(), Box::new(e.into())))
    }

    fn with_context<M, F>(self, f: F) -> Result<T, Error>
    where
        M: Into<Cow<'static, str>>,
        F: FnOnce() -> M,
    {
        self.map_err(|e| Error::Context(f().into(), Box::new(e.into())))
    }
}
