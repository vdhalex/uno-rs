use super::GamePlayer;
use rand::prelude::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ColorType {
    Red,
    Yellow,
    Blue,
    Green,
    None,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CardType {
    Number(isize),
    Skipcard,
    Reversecard,
    Draw2card,
    Wildcard,
    Wildcard4,
    None,
}

#[derive(Debug, PartialEq)]
pub struct UnoCard {
    pub(crate) color: Option<ColorType>,
    pub(crate) inst: CardType,
}

#[derive(Debug)]
pub struct UnoPlayer {
    cards: Vec<UnoCard>,
    len: usize,
}

impl UnoCard {
    pub fn new(color: ColorType, inst: CardType) -> Self {
        UnoCard {
            inst: inst,
            color: Some(color),
        }
    }

    pub(crate) fn get_color(&self) -> Option<ColorType> {
        self.color
    }

    pub(crate) fn get_card(&self) -> CardType {
        self.inst
    }

    fn clone(&mut self) -> UnoCard {
        UnoCard {
            inst: self.inst,
            color: self.color,
        }
    }
}

impl GamePlayer for UnoPlayer {
    fn new() -> Self {
        UnoPlayer {
            cards: Vec::new(),
            len: 0,
        }
    }

    fn add_cards(&mut self, cards: &mut Vec<UnoCard>) {
        for ii in 0..cards.len() {
            self.cards.push(cards[ii].clone());
        }
    }

    fn show_cards(&self) -> &[UnoCard] {
        &self.cards
    }

    fn remove_card(&mut self, card: &UnoCard) {
        for ii in 0..self.len {
            if self.cards[ii] == *card {
                self.cards.remove(ii);
                break;
            }
        }
    }
}