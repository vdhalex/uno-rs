use std::io::{BufRead, Write};
use crate::errors::Error;

pub mod unostate;

pub trait GameState {
    fn new() -> Self;
    fn begin_play(
        &mut self,
        input: impl BufRead,
        output: impl Write,
        error: impl Write) -> Result<(), Error>;
//    fn update_state();
    fn shuffle(&mut self);
    fn check_winner(&self) -> bool;
}