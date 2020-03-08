use super::gamePlayer;
use rand::prelude::*;

#[derive(Debug, PartialEq)]
pub enum ColorType {
    Red,
    Yellow,
    Blue,
    Green,
}

#[derive(Debug, PartialEq)]
pub enum CardType {
    Number(isize),
    WildCard(String),
}

pub struct unoPlayer {
    cards: Vec<UnoCard>,
    len: usize,
}

pub struct UnoCard {
    color: Option<ColorType>,
    inst: CardType,
}

impl UnoCard {
    fn new(color: ColorType, inst: CardType) -> Self {
        Self { color, inst }
    }
}

impl gamePlayer for unoPlayer {
    fn new(cards: &str) -> Self {
        for card in cards.split(' ') {
            // not sure how we want to try and do this
        }
    }

    fn draw(&self, deck: &mut Vec<u8>) {
        self.cards.push(deck.pop());
    }

    fn add_cards(&mut self, cards: &[UnoCard]) {

    }

    fn play_move(cards_to_play: &str) -> Vec<UnoCard>{
        // return cards_to_play
    }
}