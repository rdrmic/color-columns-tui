pub enum Error {
    Io(std::io::Error),
    ClockBeforeUnixEpoch,
    ParseIntEmpty,
    ParseIntInvalidDigit,
    ParseIntPosOverflow,
    Context(Box<(&'static str, Self)>),
}

impl std::error::Error for Error {}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => std::fmt::Display::fmt(e, f),
            Self::ClockBeforeUnixEpoch => {
                f.write_str("Failed to generate random seed from system time: system clock is set before year 1970: second time provided was later than self")
            }
            Self::ParseIntEmpty => f.write_str("cannot parse integer from empty string"),
            Self::ParseIntInvalidDigit => f.write_str("invalid digit found in string"),
            Self::ParseIntPosOverflow => f.write_str("number too large to fit in target type"),
            Self::Context(context) => {
                f.write_str(context.0)?;
                f.write_str(": ")?;
                std::fmt::Display::fmt(&context.1, f)
            }
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

// =============================================================================
// Context Trait
// =============================================================================
pub trait Context<T> {
    fn context(self, msg: &'static str) -> Result<T, Error>;

    // fn with_context<M, F>(self, f: F) -> Result<T, Error>
    // where
    //     M: Into<Cow<'static, str>>,
    //     F: FnOnce() -> M;
}

impl<T, E: Into<Error>> Context<T> for Result<T, E> {
    fn context(self, msg: &'static str) -> Result<T, Error> {
        match self {
            Ok(value) => Ok(value),
            Err(error) => Err(contextual_error(msg, error.into())),
        }
    }

    // fn with_context<M, F>(self, f: F) -> Result<T, Error>
    // where
    //     M: Into<Cow<'static, str>>,
    //     F: FnOnce() -> M,
    // {
    //     self.map_err(|e| Error::Context(f().into(), Box::new(e.into())))
    // }
}

#[cold]
#[inline(never)]
fn contextual_error(message: &'static str, source: Error) -> Error {
    Error::Context(Box::new((message, source)))
}

// =============================================================================
// TESTS
// =============================================================================
#[cfg(test)]
mod tests {
    use super::Error;

    #[test]
    fn clock_error_preserves_the_previous_full_diagnostic() {
        assert_eq!(
            Error::ClockBeforeUnixEpoch.to_string(),
            "Failed to generate random seed from system time: system clock is set before year 1970: second time provided was later than self"
        );
    }
}
