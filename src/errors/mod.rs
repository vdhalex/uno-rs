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
    #[error("Hand played not same as last hand color")]
    WrongColorCard,
    #[error("Incorrect input type: {0}")]
    IncorrectInput(String),
    #[error("Color not same as last card on hand")]
    ColorMismatch,
    #[error("You don't have the card you inputted")]
    YouDontHaveThisCard,
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
