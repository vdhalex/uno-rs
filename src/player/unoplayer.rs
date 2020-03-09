use super::gamePlayer;
use rand::prelude::*;

#[derive(Debug, PartialEq)]
pub enum ColorType {
    Red,
    Yellow,
    Blue,
    Green,
    None,
}

#[derive(Debug, PartialEq)]
pub enum CardType {
    Number(isize),
    Skipcard,
    Reversecard,
    Draw2card,
    Wildcard,
    Wildcard4,
}

pub struct unoCard {
    pub(crate) color: Option<ColorType>,
    inst: CardType,
}

pub struct unoPlayer {
    cards: Vec<unoCard>,
    len: usize,
}

impl UnoCard {
    fn new(color: ColorType, inst: CardType) -> Self {
        Self { color, inst }
    }
}

impl gamePlayer for unoPlayer {
    fn new() -> Self {
        unoPlayer{
            cards: Vec::new(),
            len: 0,
        }
    }

    fn add_cards(&mut self, cards: &[unoCard]) {
        for val in cards {
            self.cards.push(**val);
            self.len += 1;
        }
    }

    fn show_cards(&mut self) -> &[unoCard] {
        &self.cards
    }
}