
#[derive(Debug)]
pub enum ErrorLevel {
    Info,
    Warning,
    Error,
    Fatal
}

#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub level: ErrorLevel,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.level, self.message)
    }
}

impl Error {
    pub fn error(message: &str) -> Error {
        Error { message: message.to_owned(), level: ErrorLevel::Error }
    }
}