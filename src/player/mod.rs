pub mod unoplayer;

pub trait gamePlayer {
    fn new() -> Self;
//    fn draw(&self, &mut Vec<u8>);
    fn add_cards(&mut self, cards: &[UnoCard]);
    fn play_move(cards_to_play: &str) -> Vec<UnoCard>;
}