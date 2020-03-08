use super::gameState;
use std::io::{stderr, stdin, stdout, BufRead, BufReader, Write}; // write to the CLI interface

pub struct unoState {
    deck: Vec<u8>,
    players: [unoPlayer; 4],
    player_lens: [isize; 4],
    lastCard: Option<unoCard>,
    isActive: bool,
}

impl gameState for unoState {
    fn new() -> Self {
        // initialize all elements
    }

    fn get_move() {
        // thinking about implementing this a while loop till a winner is found
        // keeps moving forward till a skip or block card is seen
    }

    fn update_state() {
        // from the card(s) received from a player update the state
        // should write to CLI here each time someone writes
    }

    fn shuffle() {
        // shuffle current deck in hand
    }

    fn send_cards() {
        // when player has to draw cards --> send cards from the deck
    }

    fn check_winner() {
        // if any of the players have zero cards, end the game
    }
}