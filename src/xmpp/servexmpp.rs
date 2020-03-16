use std::net::{ TcpStream, TcpListener};
use crate::game_rules::unostate::{ UnoState };
use crate::player::unoplayer::UnoCard;
use uuid::Uuid;
use quick_xml::{
  Reader,
  events::Event
};

pub struct Server {
  listener: TcpListener,
  group_key: String,
  game_state: UnoState
}

impl Server {
  pub fn new(game_state: UnoState) -> Self {
    Server {
      listener: TcpListener::bind("127.0.0.1:5222"),
      group_key: Uuid::new_v4(),
      game_state,
    }
  }
}

impl ServeXMPP for Server {
  fn handle_requests(&self) {
    for stream in self.listener.incoming() {
      match stream {
        Ok(stream) => {
          handle_connection(stream);
        },
        Err(_) => {
          panic!("Connection failed");
        }
      }
    }
  }

  fn handle_stream(&self, stream: TcpStream) -> Result<(), Err> {
    let buff = vec![];
    stream.read(&buff);
    let xml = self.parse_xml(buff);
    let state = self.update_state(xml);
    self.send_message(state, stream)?;
  }

  fn parse_xml(&self, buff: &[u8]) -> Vec<String> {
    let mut reader = Reader::from_str(std::str::from_utf8(buff));
    reader.trim_text = true;

    let mut txt = Vec::new();
    let mut buf = Vec::new();

    loop {
      match reader.read_event(&mut buf) {
        Ok(Event::Start(ref e)) => {
          match e.name() {
            b"start-game" => {
              println!("starting game!");
              self.game_state.start_game(); // not implemented
            },
            _ => ()
          }
        },
        Ok(Event::Text(e)) => txt.push(e.unescape_and_decode(&reader).unwrap()),
        Ok(Event::Eof) => break,
        Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
        _ => (),
      }
    }

    buf.clear();
    txt
  }

  fn update_state(&self, xml: Vec<String>) {
    // hard to implement with current design
    // here we have one gamestate shared among all 4 players, in actuality each would have their own
    // game state, so it would be easier to update the current game state
    self.game_state.last_card = xml[1];
    // self.game_state.deck = xml[2];

    self.game_state.deck = xml[2].split(" ").map(|c| self.game_state.convert_num_to_card(c)).collect::<Vec<UnoCard>>();
    
    self.game_state.curr_action_state = xml[3];

    if self.game_state.curr_player == xml.pop() {
      self.game_state.play_turn(); // not implemented in current design
    }
  }

  fn send_message(&self, state: UnoState, stream: TcpStream) -> Result<(), Err> {
    let message = format!("<message key={} from={}>{}</message>", self.group_key, state.curr_player, state.to_xml());
    stream.write(message.as_bytes());
  }
}