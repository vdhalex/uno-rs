extern crate rand;

use rand::Rng;
use super::GameState;
use std::io::{stderr, stdin, stdout, BufRead, BufReader, Write};
use std::string::ToString;
use crate::player::unoplayer::{UnoPlayer, UnoCard, CardType, ColorType};
use crate::errors::{Error, InputError};
use crate::player::GamePlayer; // write to the CLI interface
use std::collections::HashMap;

lazy_static! {
    static ref NUM_CODE_MAP: HashMap<u8, CardType> = {
        let mut m = HashMap::new();
        m.insert(10, CardType::Skipcard);
        m.insert(11, CardType::Reversecard);
        m.insert(12, CardType::Draw2card);
        m.insert(13, CardType::Wildcard);
        m.insert(14, CardType::Wildcard4);
        m
    };
}

pub struct UnoState {
    deck: Vec<u8>,
    players: [UnoPlayer; 4],
    player_lens: [usize; 4],
    last_card: Option<UnoCard>,
    is_active: bool,
}

impl GameState for UnoState {
    fn new() -> Self {
        let mut deck = Vec::new();
        for ii in 1..109 {
            deck.push(ii as u8);
        }
        return UnoState {
            deck: deck,
            players: [UnoPlayer::new(), UnoPlayer::new(), UnoPlayer::new(), UnoPlayer::new()],
            player_lens: [0, 0, 0, 0],
            last_card: None,
            is_active: true,
        };
    }

    fn begin_play(
        &mut self,
        input: impl BufRead,
        mut output: impl Write,
        mut error: impl Write,
    ) -> Result<(), Error>{

        intro_message(&mut output);

        for ii in 1..109 {
            println!("{:?} -> {:?}", ii, convert_num_to_card(ii));
        }


        self.shuffle();
        // assign 6 cards to each player
        // set last card to be top of the deck

        for i in 0..4 {
            // change this to convert u8s to unoCards
            let mut temp_cards = Vec::new();
            for ii in self.deck[0..6].to_vec() {
                temp_cards.push(convert_num_to_card(ii));
            }
            self.players[i].add_cards(&mut temp_cards);
            self.deck.drain(0..6);
        }
        self.last_card = Some(convert_num_to_card(self.deck.pop().unwrap()));

        let mut try_again = false;
        let mut pos: usize = 0;
        let mut delta = 1;
        print_instructions(pos, self.last_card.as_ref().unwrap(), self.players[pos].show_cards(), &mut output);
        for line in input.lines() {
            println!("Checking input: {:?}", line);
           match check_input(line?.as_str(), self.last_card.as_ref().unwrap()) {
               Ok((card, color, action)) => {
                   self.last_card = Some(card);
                   self.players[pos].remove_card(self.last_card.as_ref().unwrap());
                   self.player_lens[pos] -= 1;
                   match action {
                       Some(CardType::Skipcard) => pos = update_position(pos, delta, 2),
                       Some(CardType::Reversecard) => {
                           pos = update_position(pos, -1, 0);
                           delta = -1;
                       },
                       Some(CardType::Draw2card) => {
                           let cards = self.deck[0..2].to_vec();
                           pos = update_position(pos, delta, 0);
                           self.players[pos].add_cards(
                           &mut cards.iter().map(|c| convert_num_to_card(*c)).collect());
                       },
                       _ => pos = update_position(pos, delta, 0),
                   };
                   try_again = false;
               }
               Err(err) => {
                   writeln!(error, "Error: {}", err)?;
                   println!("Try again is true");
                   try_again = true;
               }
           };
            if try_again {
                writeln!(output, "Player {} goes again!", pos+1)?;
            }
            if self.check_winner() {
                break;
            }

            // ERROR IS HERE
            // OUTPUT HAS USE OF MOVED VALUE ERROR
            print_instructions(pos, self.last_card.as_ref().unwrap(), self.players[pos].show_cards(), &mut output);
        }
        // begin input for the game
        Ok(())
    }

//    fn update_state() {
//        // from the card(s) received from a player update the state
//        // should write to CLI here each time someone writes
//    }

    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        let len = self.deck.len();
        for _ii in 0..1000 {
            let from = rng.gen_range(0, len);
            let to = rng.gen_range(0, len);
            let temp = self.deck[from];
            self.deck[from] = self.deck[to];
            self.deck[to] = temp;
        }
    }

    fn check_winner(&self) -> bool {
        for ii in &self.player_lens {
            if *ii == 0 {
                return true
            }
        }
        false
    }
}

fn update_position(pos: usize, delta: i8, skip: usize) -> usize {
    let res = (pos as i8 +(skip as i8 +1)*delta)%4;
    res.abs() as usize
}

fn check_input(input: &str, last_card: &UnoCard) -> Result<(UnoCard, ColorType, Option<CardType>), InputError> {
    if input.len() > 2 {
        return Err(InputError::IncorrectInput(input.to_string()));
    }
    // for result we should return the last card, a new color, and the action
    if input.contains(" ") {
        // should return an action struct --> update new last card with it
        let temp_vec: Vec<_> = input.split_ascii_whitespace().collect();
        if temp_vec.len() > 2 {
            return Err(InputError::IncorrectInput(input.to_string()));
        }

        if temp_vec[0] != "W" || temp_vec[0] != "W4" {
            return Err(InputError::IncorrectInput(input.to_string()));
        }

        if temp_vec[1] != "R" || temp_vec[1] != "G" || temp_vec[1] != "Y" || temp_vec[1] != "B" {
            return Err(InputError::IncorrectInput(input.to_string()));
        }


        let colort = match temp_vec[1] {
            "R" => ColorType::Red,
            "G" => ColorType::Green,
            "B" => ColorType::Blue,
            "Y" => ColorType::Yellow,
            _  => ColorType::None,
        };

        let cardt = match temp_vec[0] {
            "W" => CardType::Wildcard,
            "W4" => CardType::Wildcard4,
            _ => CardType::None,
        };
        return Ok((UnoCard {
            inst: cardt,
            color: Some(colort)
        }, colort, Some(cardt)));

    }

//    let one: usize = 1;
//    let zero: usize = 0;

    let cardt = match input.chars().next() {
        Some(num) if num.is_numeric() => CardType::Number(num as isize),
        Some(letter) => match letter {
            'S' => CardType::Skipcard,
            'R' => CardType::Reversecard,
            'D' => CardType::Draw2card,
             _  => CardType::None,
        },
        _ => CardType::None,
    };

    let colort = match input.chars().next().unwrap() {
        'R' => ColorType::Red,
        'G' => ColorType::Green,
        'B' => ColorType::Blue,
        'Y' => ColorType::Yellow,
         _  => ColorType::None,
    };

    if cardt == CardType::None || colort == ColorType::None {
        return Err(InputError::IncorrectInput(input.to_string()));
    }

    Ok((UnoCard {
        inst: cardt,
        color: Some(colort),
    }, colort, Some(cardt)))
}

fn print_instructions(player_num: usize, last_card: &UnoCard, player_cards: &[UnoCard], mut output: impl Write) {
    writeln!(output, "Player {}'s turn", player_num+1).unwrap();
    writeln!(output, "Current Last Card {}", convert_card_to_string(last_card)).unwrap();
    writeln!(output, "Player {} current cards: ", player_num+1).unwrap();
    let card_len = player_cards.len();
    for ii in 0..card_len {
        if ii < card_len-1 {
            write!(output, "{} ", convert_card_to_string(&player_cards[ii])).unwrap();
        }
        else {
            writeln!(output, "{}", convert_card_to_string(&player_cards[ii])).unwrap();
        }
    }
}

fn convert_num_to_card(num: u8) -> UnoCard {
    let cardt;
    let colort;
    let key = num%15;

    if key >= 10 {
        cardt = NUM_CODE_MAP[&key];
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

    UnoCard {
        inst: cardt,
        color: Some(colort),
    }
}

fn convert_card_to_string(ucard: &UnoCard) -> String {
    let color = match ucard.get_color() {
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
    let mut card = match ucard.get_card() {
        CardType::Number(n) => n.to_string(),
        CardType::Skipcard => "S".to_string(),
        CardType::Reversecard => "R".to_string(),
        CardType::Draw2card => "D".to_string(),
        CardType::Wildcard => "W".to_string(),
        CardType::Wildcard4 => "W4".to_string(),
        _ => "".to_string(),
    };
    card.push_str(color);
    card
}

fn intro_message(mut output: impl Write) {
    writeln!(output, "\nWelcome to UNO-rs\n").unwrap();

    writeln!(output, "Card Representation:\n \
                      \tJust like in regular UNO, there are 6 different types of cards\n \
                      \t1) Number cards: A number followed one of letter representing a color (R(red), B(blue), G(green), Y(yellow))\n \
                      \t2) Skip cards: An S followed by one of letter representing a color\n \
                      \t3) Reverse cards: An R followed by one of letter representing a color\n \
                      \t4) Draw 2 cards: A D followed by one of letter representing a color\n \
                      \t5) Wild cards: The letter W\n \
                      \t6) Wild Draw 4 cards: A W4\n").unwrap();

    writeln!(output, "\nInstructions: ").unwrap();
    writeln!(output, "\tAt the start of every round the game will say whose turn it is: Player 1's turn").unwrap();
    writeln!(output, "\tThen, it will show the last card that was played:               Current Last Card W").unwrap();
    writeln!(output, "\tFollowed by the current player's hand:                          Player 1 current cards").unwrap();
    writeln!(output, "\t                                                                1B SG RR DY W W4").unwrap();
    writeln!(output, "\tTo play type one of the cards in your hand and hit enter").unwrap();
    writeln!(output, "\tThe game follows standard UNO rules; if you make an invalid move you will be prompted to try again\n").unwrap();

    writeln!(output, "Have fun and good luck!\n\n").unwrap();
}

#[test]
fn convert_card_to_string_test() {
    assert_eq!("4R", convert_card_to_string(&UnoCard::new(ColorType::Red, CardType::Number(4))));
    assert_eq!("SG", convert_card_to_string(&UnoCard::new(ColorType::Green, CardType::Skipcard)));
    assert_eq!("RY", convert_card_to_string(&UnoCard::new(ColorType::Yellow, CardType::Reversecard)));
    assert_eq!("DB", convert_card_to_string(&UnoCard::new(ColorType::Blue, CardType::Draw2card)));
    assert_eq!("W", convert_card_to_string(&UnoCard::new(ColorType::None, CardType::Wildcard)));
    assert_eq!("W4", convert_card_to_string(&UnoCard::new(ColorType::None, CardType::Wildcard4)));
}
