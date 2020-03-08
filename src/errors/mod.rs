use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ArgsError {
    #[error("no graph file given")]
    TooFew,
    #[error("too many arguments given")]
    TooMany,
}

#[derive(Debug, Error)]
pub enum InputError {
    #[error("No path found")]
    NoPathFound,
    #[error("only 2 vertex for path, given: {0}")]
    IncorrectInput(usize),
    #[error("Unknown node: {0}")]
    UnknownNode(String),
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("bad command-line: {0}")]
    Args(#[from] ArgsError),
    #[error("input error: {0}")]
    Input(#[from] InputError),
}
