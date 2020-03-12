use std::net::{ TcpStream, TcpListener};
use crate::game_rules::unostate::{ UnoState };
use uuid::Uuid;

pub struct Server {
  listener: TcpListener,
  group_key: usize,
  group_key: String,
}

impl Server {
  pub fn new() -> Self {
    Server {
      listener: TcpListener::bind("127.0.0.1:5222"),
      group_key: Uuid::new_v4(),
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

  fn parse_xml(&self, buff: &[u8]) {
    todo!();
  }

  fn update_state(&self, xml: Vec<String>) -> UnoState {
    todo!();
  }

  fn send_message(&self, state: UnoState, stream: TcpStream) -> Result<(), Err> {
    let message = format!("<message key={} from={}>{}</message>", self.group_key, state.curr_player, state.to_xml());
    stream.write(message.as_bytes());
  }
}