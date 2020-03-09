use crate::player::unoplayer::unoCard;

pub mod unoplayer;

pub trait gamePlayer {
    fn new() -> Self;
//    fn draw(&self, &mut Vec<u8>);
    fn add_cards(&mut self, cards: &[UnoCard]);
    fn show_cards(&mut self) -> &[unoCard];
}