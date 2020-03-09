use super::gameState;
use std::io::{stderr, stdin, stdout, BufRead, BufReader, Write};
use std::string::ToString;
use crate::player::unoplayer::{unoPlayer, unoCard, CardType, ColorType};
use crate::errors::{Error, InputError};
use crate::player::gamePlayer; // write to the CLI interface

lazy_static! {
    static ref NUM_CODE_MAP: HashMap<u8, CardType> = {
        let mut m = HashMap::new();
        m.insert(10, CardType::Skipcard);
        m.insert(11, CardType::Reversecard);
        m.insert(12, CardType::Draw2card);
        m.insert(13, CardType::Wildcard);
        m.insert(14, CardType::Wildcard4);
        m
    }
}

pub struct unoState {
    deck: Vec<u8>,
    players: [unoPlayer; 4],
    player_lens: [isize; 4],
    lastCard: Option<unoCard>,
    isActive: bool,
}

impl gameState for unoState {
    fn new(cards: Vec<String>) -> Self {
        let mut deck = Vec::new();
        for ii in 1..109 {
            deck.push(ii as u8);
        }
        return unoState {
            deck: deck,
            players: [unoPlayer::new(); 4],
            player_lens: [0, 0, 0, 0],
            lastCard: None,
            isActive: true,
        };
    }

    fn begin_play(
        &mut self,
        input: impl BufRead,
        mut output: impl Write,
        mut error: impl Write,
    ) -> Result<(), Error>{
        self.shuffle();
        // assign 6 cards to each player
        // set last card to be top of the deck

        for i in 0..5 {
            // change this to convert u8s to unoCards
            let mut temp_cards = Vec::new();
            for ii in self.deck[0..6] {
                temp_cards.push(convert_num_to_card(ii));
            }
            self.players[i].add_cards(&temp_cards);
            self.deck.drain(0..6);
        }
        self.lastCard = Some(convert_num_to_card(self.deck.pop().unwrap()));

        let mut tryAgain = false;
        let mut pos = 0;
        print_instructions(pos, &self.lastCard.unwrap(), self.players[pos].show_cards(), output);
        for line in input.lines() {
           match check_input(convert_card_to_num(line?.as_str()), self.lastCard.unwrap()) {
               Ok(num) => {
                   self.lastCard = num;
                   self.players[pos].removeCard(num);
                   self.player_lens[pos] -= 1;
                   tryAgain = false;
               }
               Err(err) => {
                   writeln!(error, "Error: {}", err)?;
                   tryAgain = true;
               }
           };
            if tryAgain {
                pos += 1;
                if pos >= 5 {
                    pos %= 5;
                }
            }
            print_instructions(pos, &self.lastCard.unwrap(), self.players[pos].show_cards(), output);
        }
        // begin input for the game
        Ok(())
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

fn check_input(input: &str, last_card: unoCard) -> Result<(), Error> {
    if input.len() > 2 {
        InputError::IncorrectInput(input.to_string())
    }

    if input.contains(" ") {
        // should return an action struct --> update new last card with it
        let temp_vec = input.split_ascii_whitespace();
        if temp_vec.len() > 2 {
            InputError::IncorrectInput(input.to_string())
        }

        if temp_vec[0] != "W" || temp_vec[0] != "W4" {
            InputError::IncorrectInput(input.to_string())
        }

        if temp_vec[1] != "R" || temp_vec[1] != "G" || temp_vec[1] != "Y" || temp_vec[1] != "B" {
            InputError::IncorrectInput(input.to_string())
        }
    }



    Ok(())
}

fn print_instructions(player_num: u8, lastCard: &unoCard, player_cards: &[unoCard], mut output: impl Write) {
    writeln!(output, "Player {}'s turn", pos+1)?;
    writeln!(output, "Current Last Card {}", convert_card_to_string(lastCard))?;
    write!(output, "Player {} current cards: ", pos+1)?;
    let card_len = player_cards.len();
    for ii in card_len {
        if ii < card_len -1 {
            write!(output, "{} ", convert_card_to_string(player_cards[ii]))?;
        }
        else {
            writeln!(output, "{}", convert_card_to_string(player_cards[ii]))?;
        }
    }
}

fn convert_num_to_card(num: u8) -> unoCard {
    let cardt;
    let colort;
    let key = num%15;
    if key >= 10 {
        cardt = NUM_CODE_MAP[key];
    } else {
        cardt = CardType::Number(key as isize);
    }
    if key <= 12 {
        colort = match num/22 {
            0 => ColorType::Red,
            1 => ColorType::Green,
            2 => ColorType::Blue,
            3 => ColorType::Yellow,
            _ => ColorType::None,
        };
    } else {
        colort = ColorType::None;
    }

    unoCard{
        inst: cardt,
        color: Some(colort),
    }
}

fn convert_card_to_string(ucard: &unoCard) -> String {
    let color = match *ucard.color {
        Some(col) => {
            match col {
                ColorType::Red => "R",
                ColorType::Green => "G",
                ColorType::Yellow => "Y",
                ColorType::Blue => "B",
                _ => "",
            }
        },
        None => "",
    };
    let mut card = match ucard.card {
        CardType::Number(n) => n.to_string(),
        CardType::Skipcard => "S".to_string(),
        CardType::Reversecard => "R".to_string(),
        CardType::Draw2card => "D2".to_string(),
        CardType::Wildcard => "W".to_string(),
        CardType::Wildcard4 => "W4".to_string(),
        _ => "",
    };
    card.push_str(color);
    card
}
