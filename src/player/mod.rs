pub mod unoplayer;

#[derive(Debug, PartialEq)]
pub enum CardType {
    Number(isize),
    WildCard(String),
}

#[derive(Debug, PartialEq)]
pub enum ColorType {
    Red,
    Yellow,
    Blue,
    Green,
}

pub struct UnoCard {
    color: Option<ColorType>,
    inst: CardType,
}

pub trait gamePlayer {
    fn new(_: &str) -> Self;
    fn add_cards(&mut self, cards: &[UnoCard]);
    fn play_move(cards_to_play: &str) -> Vec<UnoCard>;
}