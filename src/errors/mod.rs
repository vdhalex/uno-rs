use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InputError {
    #[error("Hand played not same as last hand color")]
    WrongColorCard,
    #[error("Incorrect input type: {0}")]
    IncorrectInput(String),
    #[error("You don't have the card you inputted")]
    YouDontHaveThisCard,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("input error: {0}")]
    Input(#[from] InputError),
}
