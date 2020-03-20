use super::ServeXMPP;
use crate::game_rules::unostate::UnoState;
use crate::game_rules::GameState;
use std::error::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use quick_xml::{events::Event, Reader};
use uuid::Uuid;

pub struct Server {
    listener: TcpListener,
    group_key: Uuid,
    game_state: UnoState,
}

impl Server {
    pub fn new(game_state: UnoState) -> Self {
        Server {
            listener: TcpListener::bind("127.0.0.1:5222").unwrap(),
            group_key: Uuid::new_v4(),
            game_state,
        }
    }
}

impl ServeXMPP for Server {
    fn handle_requests(&mut self) {
        loop {
            match self.listener.accept() {
                Ok((stream, _addr)) => self
                    .handle_connection(stream)
                    .expect("Connection handling failed"),
                Err(_) => (),
            }
        }
    }

    fn start(&self) {
        unimplemented!();
    }

    fn handle_connection(&mut self, mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
        let mut buff = vec![];
        stream.read(&mut buff).expect("Failed to read buffer");
        let xml = parse_xml(&buff);
        self.update_state(xml);
        self.send_message(stream)?;

        Ok(())
    }

    fn update_state(&mut self, xml: Vec<String>) {
        // hard to implement with current design
        // here we have one gamestate shared among all 4 players, in actuality each would have their own
        // game state, so it would be easier to update the current game state
        self.game_state.last_card = self
            .game_state
            .convert_num_to_card(xml[1].parse::<u8>().unwrap());

        self.game_state.deck = xml[2]
            .split(" ")
            .map(|c| c.parse::<u8>().unwrap())
            .collect();

        if xml[3] == "none" {
            self.game_state.curr_action_state = None;
        } else {
            self.game_state.curr_action_state = Some(xml[3].clone());
        }

        if self.game_state.curr_player == xml[0].parse::<usize>().unwrap() {
            self.game_state.play_turn(); // not implemented in current design
        }
    }

    fn send_message(&self, mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
        let message = format!(
            "<message key={} from={}>{}</message>",
            self.group_key,
            self.game_state.curr_player,
            self.game_state.to_xml()
        );
        stream
            .write(message.as_bytes())
            .expect("Failed to write to stream.");

        Ok(())
    }
}

pub fn parse_xml(buff: &[u8]) -> Vec<String> {
    let mut reader = Reader::from_str(std::str::from_utf8(buff).unwrap());
    reader.trim_text(true);

    let mut txt = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"start-game" => {
                        println!("starting game!");
                        // self.game_state.start_game(); // not implemented, begin play implemented differently
                    }
                    _ => (),
                }
            }
            Ok(Event::Text(e)) => txt.push(e.unescape_and_decode(&reader).unwrap()),
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
    }

    buf.clear();
    txt
}
