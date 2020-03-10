use super::gamePlayer;
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

pub struct unoCard {
    pub(crate) color: Option<ColorType>,
    inst: CardType,
}

pub struct unoPlayer {
    cards: Vec<unoCard>,
    len: usize,
}

impl unoCard {
    fn new(color: ColorType, inst: CardType) -> Self {
        unoCard{
            inst: inst,
            color: Some(color),
        }
    }

    fn get_color(&mut self) -> Option<ColorType> {
        self.color
    }

    fn get_card(&mut self) -> CardType {
        self.inst
    }

    fn clone(&mut self) -> unoCard {
        unoCard{
            inst: self.inst,
            color: self.color,
        }
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
        for ii in 0..cards.len() {
            self.cards.push(cards[ii].clone());
        }
    }

    fn show_cards(&mut self) -> &[unoCard] {
        &self.cards
    }

    fn remove_card(&mut self, card: unoCard) {
        for ii in 0..self.len {
            if self.cards[ii] == card {
                self.cards.remove(ii);
                break;
            }
        }
    }
}