use std::net::{ TcpStream, TcpListener};
use crate::game_rules::unostate::{ UnoState };

pub struct Server {
  listener: TcpListener,
  group_key: usize,
}

impl Server {
  pub fn new() -> Self {
    Server {
      listener: TcpListener::bind("127.0.0.1:5222"),
      group_key: 23,
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

  fn update_state(&self, xml: String) -> UnoState {
    todo!();
  }

  fn send_message(&self, state: UnoState, stream: TcpStream) -> Result<(), Err> {
    stream.write(state.to_xml().as_bytes());
  }
}