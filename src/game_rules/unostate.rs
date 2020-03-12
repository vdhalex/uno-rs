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
    curr_player: usize,
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
            curr_player: 0
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
        // let mut self.curr_player: usize = 0;
        let mut delta = 1;
        print_instructions(self.curr_player, self.last_card.as_ref().unwrap(), self.players[self.curr_player].show_cards(), &mut output);
        for line in input.lines() {
           match check_input(line?.as_str(), self.last_card.as_ref().unwrap()) {
               Ok((card, color, action)) => {
                   self.last_card = Some(card);
                   self.players[self.curr_player].remove_card(self.last_card.as_ref().unwrap());
                   self.player_lens[self.curr_player] -= 1;
                   match action {
                       Some(CardType::Skipcard) => self.curr_player = update_position(self.curr_player, delta, 2),
                       Some(CardType::Reversecard) => {
                           self.curr_player = update_position(self.curr_player, -1, 0);
                           delta = -1;
                       },
                       Some(CardType::Draw2card) => {
                           let cards = self.deck[0..2].to_vec();
                           self.curr_player = update_position(self.curr_player, delta, 0);
                           self.players[self.curr_player].add_cards(
                           &mut cards.iter().map(|c| convert_num_to_card(*c)).collect());
                       },
                       _ => self.curr_player = update_position(self.curr_player, delta, 0),
                   };
                   try_again = false;
               }
               Err(err) => {
                   writeln!(error, "Error: {}", err)?;
                   try_again = true;
               }
           };
            if try_again {
                writeln!(output, "Player {} goes again!", self.curr_player+1)?;
            }
            if self.check_winner() {
                break;
            }

            // ERROR IS HERE
            // OUTPUT HAS USE OF MOVED VALUE ERROR
            print_instructions(self.curr_player, self.last_card.as_ref().unwrap(), self.players[self.curr_player].show_cards(), &mut output);
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

    fn to_xml(&self) -> String {
        let next_player = format!("<player>{}</player>", self.curr_player+1);
        let last_card = format!("<last-card>{}</last-card>", self.last_card);
        next_player
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
    write!(output, "Player {} current cards: ", player_num+1).unwrap();
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
