pub trait BrokenPipeError {
    fn is_broken_pipe(&self) -> bool;
}

impl BrokenPipeError for anyhow::Error {
    fn is_broken_pipe(&self) -> bool {
        matches!(self.downcast_ref::<std::io::Error>(),
            Some(ioe) if ioe.kind() == std::io::ErrorKind::BrokenPipe)
    }
}

impl<T> BrokenPipeError for anyhow::Result<T> {
    fn is_broken_pipe(&self) -> bool {
        match self {
            Err(err) => err.is_broken_pipe(),
            _ => false,
        }
    }
}
