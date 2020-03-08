use super::gamePlayer;

pub struct unoPlayer {
    cards: Vec<UnoCard>,
    len: usize,
}

impl gamePlayer for unoPlayer {
    fn new(given_cards: &str) -> Self {

    }

    fn add_cards(&mut self, cards: &[UnoCard]) {

    }

    fn play_move(cards_to_play: &str) -> Vec<UnoCard>{
        // return cards_to_play
    }
}