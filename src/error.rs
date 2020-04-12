use std::io::ErrorKind;

#[derive(Debug)]
pub enum Error {
    NullException,
    FileNotFound,
    PermissionDenied,
    UnknownIOError,
    ParseFailed,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        match e.kind() {
            ErrorKind::NotFound => Error::FileNotFound,
            ErrorKind::PermissionDenied => Error::PermissionDenied,
            _ => Error::UnknownIOError,
        }
    }
}

impl From<std::option::NoneError> for Error {
    fn from(_: std::option::NoneError) -> Self {
        Error::NullException
    }
}
