extern crate rand;

use rand::Rng;
use super::gameState;
use std::io::{stderr, stdin, stdout, BufRead, BufReader, Write};
use std::string::ToString;
use crate::player::unoplayer::{unoPlayer, unoCard, CardType, ColorType};
use crate::errors::{Error, InputError};
use crate::player::gamePlayer; // write to the CLI interface
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

pub struct unoState {
    deck: Vec<u8>,
    players: [unoPlayer; 4],
    player_lens: [usize; 4],
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
            for ii in self.deck[0..6].to_vec() {
                temp_cards.push(convert_num_to_card(ii));
            }
            self.players[i].add_cards(&temp_cards);
            self.deck.drain(0..6);
        }
        self.lastCard = Some(convert_num_to_card(self.deck.pop().unwrap()));

        let mut tryAgain = false;
        let mut pos: usize = 0;
        let mut delta = 1;
        print_instructions(pos, &self.lastCard.unwrap(), self.players[pos].show_cards(), output);
        for line in input.lines() {
           match check_input(line?.as_str(), self.lastCard.unwrap()) {
               Ok((card, color, action)) => {
                   self.lastCard = Some(card);
                   self.players[pos].remove_card(self.lastCard.unwrap());
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
                           cards.iter().map(|c| convert_num_to_card(*c)).collect());
                       },
                       _ => pos = update_position(pos, delta, 0),
                   };
                   tryAgain = false;
               }
               Err(err) => {
                   writeln!(error, "Error: {}", err)?;
                   tryAgain = true;
               }
           };
            if tryAgain {
                writeln!(output, "Player {} goes again!", pos+1);
            }
            if self.check_winner() {
                break;
            }
            print_instructions(pos, &self.lastCard.unwrap(), self.players[pos].show_cards(), output);
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
        for ii in 0..1000 {
            let from = rng.gen_range(0, len);
            let to = rng.gen_range(0, len);
            let temp = self.deck[from];
            self.deck[from] = self.deck[to];
            self.deck[to] = temp;
        }
    }

    fn check_winner(&self) -> bool {
        for ii in &self.player_lens {
            if ii == 0 {
                return true
            }
        }
        false
    }
}

fn update_position(mut pos: usize, delta: i8, mut skip: usize) -> usize {
    let res = (pos as i8 +(skip as i8 +1)*delta)%4;
    res.abs() as usize
}

fn check_input(input: &str, last_card: unoCard) -> Result<(unoCard, ColorType, Option<CardType>), InputError> {
    if input.len() > 2 {
        InputError::IncorrectInput(input.to_string())
    }
    // for result we should return the last card, a new color, and the action
    if input.contains(" ") {
        // should return an action struct --> update new last card with it
        let temp_vec = input.split_ascii_whitespace().collect();
        if temp_vec.len() > 2 {
            Err(InputError::IncorrectInput(input.to_string()))
        }

        if temp_vec[0] != "W" || temp_vec[0] != "W4" {
            Err(InputError::IncorrectInput(input.to_string()))
        }

        if temp_vec[1] != "R" || temp_vec[1] != "G" || temp_vec[1] != "Y" || temp_vec[1] != "B" {
            Err(InputError::IncorrectInput(input.to_string()))
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
        };
        return Ok((unoCard{
            inst: cardt,
            color: Some(colort)
        }, colort, Some(cardt)));

    }


    let one: usize = 1;
    let zero: usize = 0;

    let cardt = match input[zero].parse::<u32>() {
        Ok(num) => CardType::Number(num as isize),
        Err(err) => match input[zero] {
            "S" => CardType::Skipcard,
            "R" => CardType::Reversecard,
            "D" => CardType::Draw2card,
             _  => CardType::None,
        },
    };

    let colort = match input[one] {
        "R" => ColorType::Red,
        "G" => ColorType::Green,
        "B" => ColorType::Blue,
        "Y" => ColorType::Yellow,
         _  => ColorType::None,
    };

    if cardt == CardType::None || colort == ColorType::None {
        Err(InputError::IncorrectInput(input.to_string()))
    }

    Ok((unoCard{
        inst: cardt,
        color: Some(colort),
    }, colort, Some(cardt)))
}

fn print_instructions(player_num: usize, lastCard: &unoCard, player_cards: &[unoCard], mut output: impl Write) {
    writeln!(output, "Player {}'s turn", player_num+1).unwrap();
    writeln!(output, "Current Last Card {}", convert_card_to_string(lastCard)).unwrap();
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

fn convert_num_to_card(num: u8) -> unoCard {
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

    unoCard{
        inst: cardt,
        color: Some(colort),
    }
}

fn convert_card_to_string(ucard: &unoCard) -> String {
    let color = match ucard.get_color {
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
    let mut card = match ucard.get_card {
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
