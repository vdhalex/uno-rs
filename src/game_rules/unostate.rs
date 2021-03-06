extern crate rand;

use super::GameState;
use crate::errors::{Error, InputError};
use crate::player::unoplayer::{CardType, ColorType, UnoCard, UnoPlayer};
use crate::player::GamePlayer;
use rand::Rng;
use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::string::ToString;

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
    last_card: UnoCard,
    _is_active: bool,
}

impl GameState for UnoState {
    fn new() -> Self {
        let mut deck = Vec::new();
        for ii in 1..109 {
            deck.push(ii as u8);
        }
        return UnoState {
            deck: deck,
            players: [
                UnoPlayer::new(),
                UnoPlayer::new(),
                UnoPlayer::new(),
                UnoPlayer::new(),
            ],
            player_lens: [0, 0, 0, 0],
            last_card: UnoCard::new(ColorType::None, CardType::None),
            _is_active: true,
        };
    }

    fn begin_play(
        &mut self,
        input: impl BufRead,
        mut output: impl Write,
        mut error: impl Write,
    ) -> Result<(), Error> {
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
            self.player_lens[i] = 6;
            self.deck.drain(0..6);
        }

        while let Some(cur_card) = self.deck.pop() {
            if cur_card % 15 >= 10 {
                self.deck.push(cur_card);
            } else {
                self.last_card = convert_num_to_card(cur_card);
                break;
            }
        }

        let mut try_again = false;
        let mut pos: usize = 0;
        let mut delta = 1;
        print_instructions(
            pos,
            &self.last_card,
            self.players[pos].show_cards(),
            &mut output,
        );
        for line in input.lines() {
            match check_input(line?.as_str(), &self.last_card) {
                Ok((color, action)) => {
                    self.deck.push(convert_card_to_num(&self.last_card));
                    if action.unwrap() != CardType::None && !self.update_state(&color, action, pos)
                    {
                        writeln!(error, "{}", InputError::YouDontHaveThisCard)?;
                        try_again = true;
                    };
                    if !try_again {
                        match action {
                            Some(CardType::Number(_num)) => {
                                pos = update_position(pos, delta, 0);
                            }
                            Some(CardType::Skipcard) => {
                                pos = update_position(pos, delta, 1);
                            }
                            Some(CardType::Reversecard) => {
                                delta = -1;
                                pos = update_position(pos, delta, 0);
                            }
                            Some(CardType::Draw2card) => {
                                let cards = self.deck[0..2].to_vec();
                                pos = update_position(pos, delta, 0);
                                self.players[pos].add_cards(
                                    &mut cards.iter().map(|c| convert_num_to_card(*c)).collect(),
                                );
                            }
                            Some(CardType::Wildcard) => {
                                pos = update_position(pos, delta, 0);
                            }
                            Some(CardType::Wildcard4) => {
                                let cards = self.deck[0..4].to_vec();
                                pos = update_position(pos, delta, 0);
                                self.players[pos].add_cards(
                                    &mut cards.iter().map(|c| convert_num_to_card(*c)).collect(),
                                );
                            }
                            Some(CardType::None) => {
                                let cards = self.deck[0..2].to_vec();
                                self.players[pos].add_cards(
                                    &mut cards.iter().map(|c| convert_num_to_card(*c)).collect(),
                                );
                                pos = update_position(pos, delta, 0);
                            }
                            _ => pos = update_position(pos, delta, 0),
                        };
                        try_again = false;
                    }
                }
                Err(err) => {
                    writeln!(error, "Error: {}", err)?;
                    try_again = true;
                }
            };
            if try_again {
                writeln!(output, "Player {} goes again!", pos + 1)?;
                try_again = false;
            }
            if self.check_winner() {
                writeln!(output, "Congrats Player {:?}", pos + 1);
                break;
            }
            print_instructions(
                pos,
                &self.last_card,
                self.players[pos].show_cards(),
                &mut output,
            );
        }
        // begin input for the game
        Ok(())
    }

    fn update_state(&mut self, color: &ColorType, card: Option<CardType>, pos: usize) -> bool {
        if card != Some(CardType::Wildcard) && card != Some(CardType::Wildcard4) {
            let temp_card = UnoCard::new(*color, card.unwrap());
            if self.players[pos].remove_card(&temp_card) {
                self.last_card.update_color(*color);
                self.last_card.update_card(card.unwrap());
                self.player_lens[pos] -= 1;
                return true;
            }
        } else {
            let temp_card = UnoCard::new(ColorType::None, card.unwrap());
            if self.players[pos].remove_card(&temp_card) {
                self.last_card.update_color(*color);
                self.last_card.update_card(card.unwrap());
                self.player_lens[pos] -= 1;
                return true;
            }
        }
        false
    }

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
                return true;
            }
        }
        false
    }
}

fn update_position(pos: usize, delta: i8, skip: usize) -> usize {
    let mut res = pos as i8 + (skip as i8 + 1) * delta;
    if res < 0 {
        res += 4;
    }
    res %= 4;
    res.abs() as usize
}

#[cfg(test)]
mod test_update_position {
    use crate::game_rules::unostate::update_position;

    #[test]
    fn basic_test() {
        assert_eq!(update_position(0, 1, 0), 1)
    }

    #[test]
    fn test_skip() {
        assert_eq!(update_position(0, 1, 1), 2)
    }

    #[test]
    fn test_reverse() {
        assert_eq!(update_position(1, -1, 0), 0)
    }

    #[test]
    fn test_skip_reverse() {
        assert_eq!(update_position(3, -1, 1), 1)
    }
}

fn check_input(
    input: &str,
    last_card: &UnoCard,
) -> Result<(ColorType, Option<CardType>), InputError> {
    // for result we should return the last card, a new color, and the action
    if input == "No Move" {
        return Ok((last_card.get_color().unwrap(), Some(CardType::None)));
    }
    if input.contains(" ") {
        // should return an action struct --> update new last card with it
        let temp_vec: Vec<_> = input.split_ascii_whitespace().collect();
        if temp_vec.len() > 2 {
            return Err(InputError::IncorrectInput(input.to_string()));
        }

        if temp_vec[0] == "w" || temp_vec[0] == "w4" {
            if temp_vec[1] != "R" && temp_vec[1] != "G" && temp_vec[1] != "Y" && temp_vec[1] != "B"
            {
                return Err(InputError::IncorrectInput(input.to_string()));
            }

            let colort = match temp_vec[1] {
                "R" => ColorType::Red,
                "G" => ColorType::Green,
                "B" => ColorType::Blue,
                "Y" => ColorType::Yellow,
                _ => ColorType::None,
            };

            let cardt = match temp_vec[0] {
                "w" => CardType::Wildcard,
                "w4" => CardType::Wildcard4,
                _ => CardType::None,
            };
            return Ok((colort, Some(cardt)));
        } else {
            return Err(InputError::IncorrectInput(input.to_string()));
        }
    }

    if input.len() > 2 {
        return Err(InputError::IncorrectInput(input.to_string()));
    }

    let mut input_chars = input.chars();
    let cardt = match input_chars.next() {
        Some(num) if num.is_numeric() => CardType::Number(num.to_digit(10).unwrap() as usize),
        Some(letter) => match letter {
            's' => CardType::Skipcard,
            'r' => CardType::Reversecard,
            'd' => CardType::Draw2card,
            _ => CardType::None,
        },
        _ => CardType::None,
    };

    let colort = match input_chars.next() {
        Some('R') => ColorType::Red,
        Some('G') => ColorType::Green,
        Some('B') => ColorType::Blue,
        Some('Y') => ColorType::Yellow,
        _ => ColorType::None,
    };

    if cardt == CardType::None || colort == ColorType::None {
        return Err(InputError::IncorrectInput(input.to_string()));
    }

    if colort != last_card.get_color().unwrap() && cardt != last_card.get_card() {
        return Err(InputError::WrongColorCard);
    }
    Ok((colort, Some(cardt)))
}

#[cfg(test)]
mod test_check_input {
    use crate::errors::InputError;
    use crate::game_rules::unostate::check_input;
    use crate::player::unoplayer::{CardType, ColorType, UnoCard};

    #[test]
    fn basic_test() {
        match check_input(
            "8R",
            &UnoCard {
                inst: CardType::Number(4),
                color: Some(ColorType::Red),
            },
        ) {
            Ok((color, num)) => {
                assert_eq!(color, ColorType::Red);
                assert_eq!(num.unwrap(), CardType::Number(8))
            }
            Err(_err) => println!("Found an error"),
        }
    }

    #[test]
    fn basic_test_number() {
        match check_input(
            "4B",
            &UnoCard {
                inst: CardType::Number(4),
                color: Some(ColorType::Red),
            },
        ) {
            Ok((color, num)) => {
                assert_eq!(color, ColorType::Blue);
                assert_eq!(num.unwrap(), CardType::Number(4))
            }
            Err(_err) => println!("Found an error"),
        }
    }

    #[test]
    fn test_wildcard4() {
        match check_input(
            "w4 B",
            &UnoCard {
                inst: CardType::Number(6),
                color: Some(ColorType::Red),
            },
        ) {
            Ok((color, num)) => {
                assert_eq!(color, ColorType::Blue);
                assert_eq!(num.unwrap(), CardType::Wildcard4)
            }
            Err(_err) => println!("Found an error in test_wildcard()"),
        }
    }

    #[test]
    fn test_input_error() {
        match check_input(
            "4 R",
            &UnoCard {
                inst: CardType::Number(4),
                color: Some(ColorType::Red),
            },
        ) {
            Ok((_c, _a)) => println!("Should get error but test_input_error worked"),
            Err(err) => assert_eq!(err, InputError::IncorrectInput("4 R".to_string())),
        }
    }

    #[test]
    fn test_wrong_color_input() {
        match check_input(
            "5B",
            &UnoCard {
                inst: CardType::Number(4),
                color: Some(ColorType::Red),
            },
        ) {
            Ok((_c, _a)) => println!("Should get error but test_input_error worked"),
            Err(err) => assert_eq!(err, InputError::WrongColorCard),
        }
    }

    #[test]
    fn test_wildcard() {
        match check_input(
            "w B",
            &UnoCard {
                inst: CardType::Number(4),
                color: Some(ColorType::Red),
            },
        ) {
            Ok((color, num)) => {
                assert_eq!(color, ColorType::Blue);
                assert_eq!(num.unwrap(), CardType::Wildcard)
            }
            Err(_err) => println!("Found an error"),
        }
    }
}

fn print_instructions(
    player_num: usize,
    last_card: &UnoCard,
    player_cards: &[UnoCard],
    mut output: impl Write,
) {
    writeln!(output, "\n\nPlayer {}'s turn", player_num + 1).unwrap();
    writeln!(
        output,
        "Current Last Card {}",
        convert_card_to_string(last_card)
    )
    .unwrap();
    write!(output, "Player {} current cards: ", player_num + 1).unwrap();
    let card_len = player_cards.len();
    for ii in 0..card_len {
        if ii < card_len - 1 {
            write!(output, "{} ", convert_card_to_string(&player_cards[ii])).unwrap();
        } else {
            writeln!(output, "{}", convert_card_to_string(&player_cards[ii])).unwrap();
        }
    }
}

fn convert_num_to_card(num: u8) -> UnoCard {
    let cardt;
    let colort;
    let key = num % 15;
    if key >= 10 {
        cardt = NUM_CODE_MAP[&key];
    } else {
        cardt = CardType::Number(key as usize);
    }
    if key <= 12 {
        colort = match num / 28 {
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

#[cfg(test)]
mod test_convert_num_to_card {
    use crate::game_rules::unostate::convert_num_to_card;
    use crate::player::unoplayer::{CardType, ColorType, UnoCard};

    #[test]
    fn basic_test() {
        assert_eq!(
            convert_num_to_card(45),
            UnoCard {
                inst: CardType::Number(0),
                color: Some(ColorType::Green)
            }
        )
    }

    #[test]
    fn test_wildcard() {
        assert_eq!(
            convert_num_to_card(59),
            UnoCard {
                inst: CardType::Wildcard4,
                color: Some(ColorType::None)
            }
        )
    }

    #[test]
    fn test_skip() {
        assert_eq!(
            convert_num_to_card(10),
            UnoCard {
                inst: CardType::Skipcard,
                color: Some(ColorType::Red)
            }
        )
    }
}

fn convert_card_to_string(ucard: &UnoCard) -> String {
    let color = match ucard.get_color() {
        Some(col) => match col {
            ColorType::Red => "R",
            ColorType::Green => "G",
            ColorType::Yellow => "Y",
            ColorType::Blue => "B",
            _ => "",
        },
        None => "",
    };
    let mut card = match ucard.get_card() {
        CardType::Number(n) => n.to_string(),
        CardType::Skipcard => "s".to_string(),
        CardType::Reversecard => "r".to_string(),
        CardType::Draw2card => "d".to_string(),
        CardType::Wildcard => "w".to_string(),
        CardType::Wildcard4 => "w4".to_string(),
        _ => "".to_string(),
    };
    card.push_str(color);
    card
}

#[cfg(test)]
mod test_convert_card_to_string {
    use crate::game_rules::unostate::convert_card_to_string;
    use crate::player::unoplayer::{CardType, ColorType, UnoCard};

    #[test]
    fn test_basic_conversion() {
        assert_eq!(
            convert_card_to_string(&UnoCard {
                inst: CardType::Number(2),
                color: Some(ColorType::Red),
            }),
            "2R".to_string()
        )
    }

    #[test]
    fn test_skip() {
        assert_eq!(
            convert_card_to_string(&UnoCard {
                inst: CardType::Skipcard,
                color: Some(ColorType::Blue),
            }),
            "sB".to_string()
        )
    }

    #[test]
    fn test_reverse() {
        assert_eq!(
            convert_card_to_string(&UnoCard {
                inst: CardType::Reversecard,
                color: Some(ColorType::Green),
            }),
            "rG".to_string()
        )
    }

    #[test]
    fn test_draw2() {
        assert_eq!(
            convert_card_to_string(&UnoCard {
                inst: CardType::Draw2card,
                color: Some(ColorType::Yellow),
            }),
            "dY".to_string()
        )
    }

    #[test]
    fn test_wildcard() {
        assert_eq!(
            convert_card_to_string(&UnoCard {
                inst: CardType::Wildcard,
                color: Some(ColorType::Red),
            }),
            "wR".to_string()
        )
    }
}

fn convert_card_to_num(card: &UnoCard) -> u8 {
    let rem = match card.get_card() {
        CardType::Number(num) => num,
        CardType::Skipcard => 10,
        CardType::Reversecard => 11,
        CardType::Draw2card => 12,
        CardType::Wildcard => 13,
        CardType::Wildcard4 => 14,
        _ => 0,
    };

    let quo = match card.get_color().unwrap() {
        ColorType::Red => 0,
        ColorType::Green => 1,
        ColorType::Blue => 2,
        ColorType::Yellow => 4,
        _ => 0,
    };

    ((quo + 1) * 27) + (rem + 1) as u8
}
