use super::gameState;
use std::io::{stderr, stdin, stdout, BufRead, BufReader, Write};
use std::string::ToString; // write to the CLI interface

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
        return unoState {
            deck: [1..108].to_vec(),
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
            self.players[i].add_cards(&self.deck[0..6]);
            self.deck.drain(0..6);
        }
        self.lastCard = convert_num_to_card(self.deck.pop(0));

        writeln!(output, "Player 1 Begins the game!")?;
        writeln!(output, "Current Last Card {}", convert_card_to_string(self.lastCard))?;
        write!(output, "Player 1 current cards: ")?;
        for ii in self.players[0].cards.len() {
            if ii < self.players[0].cards.len() -1 {
                write!(output, "{} ", self.players[0].cards[ii])?;
            }
            else {
                writeln!(output, "{}", self.players[0].cards[ii])?;
            }
        }
        let mut tryAgain = false;
        let mut pos = 0;
        for line in input.lines() {
           match convert_card_to_num(line?.as_str()) {
               Ok(num) => {
                   self.lastCard = num;
                   self.players[pos].removeCard(num);
                   tryAgain = false;
               }
               Err() => {
                   writeln!(error, "Error: {}", err)?;
                   tryAgain = true;
               }
           };
            if tryAgain {
                pos += 1;
                if pos >= 5 {
                    pos %= 5;
                }
                writeln!(output, "Player {}'s turn!", pos+1)?;
                writeln!(output, "Current Last Card {}", convert_card_to_string(self.lastCard))?;
                write!(output, "Player {} current cards: ", pos)?;
                for ii in self.players[pos].cards.len() {
                    if ii < self.players[pos].cards.len() -1 {
                        write!(output, "{} ", self.players[pos+1].cards[ii])?;
                    }
                    else {
                        writeln!(output, "{}", self.players[pos+1].cards[ii])?;
                    }
                }
            }
            else {
                writeln!(output, "Player {} try again!", pos)?;
                writeln!(output, "Current Last Card {}", convert_card_to_string(self.lastCard))?;
                write!(output, "Player {} current cards: ", pos)?;
                for ii in self.players[pos].cards.len() {
                    if ii < self.players[pos].cards.len() -1 {
                        write!(output, "{} ", self.players[pos+1].cards[ii])?;
                    }
                    else {
                        writeln!(output, "{}", self.players[pos+1].cards[ii])?;
                    }
                }
            }
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

fn convert_num_to_card(num: u8) -> unoCard {
    let cardt;
    let colort;
    let key = num%15;
    if key >= 10 {
        cardt = NUM_CODE_MAP[key];
    } else {
        cardt = CardType::Number(key);
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
        card: cardt,
        color: colort,
    }
}

fn convert_card_to_string(ucard: &unoCard) -> String {
    let color = match ucard.color {
        ColorType::Red => " Red",
        ColorType::Green => " Green",
        ColorType::Yellow => " Yellow",
        ColorType::Blue => " Blue",
        _ => "",
    };
    let mut card = match ucard.card {
        CardType::Number(n) => n.to_string(),
        CardType::Skipcard => "Skip".to_string(),
        CardType::Reversecard => "Reverse".to_string(),
        CardType::Draw2card => "Draw 2".to_string(),
        CardType::Wildcard => "Wildcard".to_string(),
        CardType::Wildcard4 => "Wildcard + 4".to_string(),
        _ => "",
    };
    card.push_str(color);
    card
}
