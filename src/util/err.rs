#[cfg(has_not_matches)]
macro_rules! my_matches {
    ($expression:expr, $( $pattern:pat )|+ $( if $guard: expr )? $(,)?) => {
        match $expression {
            $( $pattern )|+ $( if $guard )? => true,
            _ => false
        }
    }
}
#[cfg(not(has_not_matches))]
macro_rules! my_matches {
    ($expression:expr, $( $pattern:pat )|+ $( if $guard: expr )? $(,)?) => {
        matches!($expression, $( $pattern )|+ $( if $guard )?)
    }
}

pub trait BrokenPipeError {
    fn is_broken_pipe(&self) -> bool;
}

impl BrokenPipeError for anyhow::Error {
    fn is_broken_pipe(&self) -> bool {
        my_matches!(self.downcast_ref::<std::io::Error>(),
            Some(ref ioe) if ioe.kind() == std::io::ErrorKind::BrokenPipe)
    }
}

impl<T> BrokenPipeError for anyhow::Result<T> {
    fn is_broken_pipe(&self) -> bool {
        match self {
            Err(ref err) => err.is_broken_pipe(),
            _ => false,
        }
    }
}
