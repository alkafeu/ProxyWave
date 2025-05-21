use thiserror::Error;

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Engine already running")]
    AlreadyRunning,

    #[error("Engine not running")]
    NotRunning,

    #[error("Unexpected: {0}")]
    Other(String),
} 