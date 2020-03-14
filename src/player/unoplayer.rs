use super::GamePlayer;

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
    Number(usize),
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

    pub(crate) fn update_color(&mut self, color: ColorType) {
        self.color = Some(color);
    }

    pub(crate) fn update_card(&mut self, card: CardType) {
        self.inst = card;
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
            self.len += 1;
        }
    }

    fn show_cards(&mut self) -> &mut [UnoCard] {
        &mut self.cards
    }

    fn remove_card(&mut self, card: &UnoCard) -> bool {
        for ii in 0..self.len {
            if self.cards[ii] == *card {
                self.cards.remove(ii);
                self.len -= 1;
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod test_remove_card {
    use crate::player::unoplayer::{CardType, ColorType, UnoCard, UnoPlayer};
    use crate::player::GamePlayer;

    #[test]
    fn basic_test() {
        let mut player = UnoPlayer::new();
        let mut cards = Vec::new();
        cards.push(UnoCard::new(ColorType::Red, CardType::Number(2)));
        cards.push(UnoCard::new(ColorType::Green, CardType::Number(4)));
        player.add_cards(&mut cards);
        player.remove_card(&cards[0]);
        assert_eq!(player.show_cards().len(), 1);
        assert_eq!(player.show_cards(), &mut cards[1..]);
    }
}
