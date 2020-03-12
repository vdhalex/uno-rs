use crate::errors::Error;
use crate::player::unoplayer::{CardType, ColorType};
use std::io::{BufRead, Write};

pub mod unostate;

pub trait GameState {
    fn new() -> Self;
    fn begin_play(
        &mut self,
        input: impl BufRead,
        output: impl Write,
        error: impl Write,
    ) -> Result<(), Error>;
    fn update_state(&mut self, color: &ColorType, card: Option<CardType>, pos: usize) -> bool;
    fn shuffle(&mut self);
    fn check_winner(&self) -> bool;
    fn to_xml(&self) -> String;
}
