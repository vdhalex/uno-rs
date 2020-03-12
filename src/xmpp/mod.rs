pub mod xmppserver;
use crate::game_rules::unostate::{ UnoState };
use std::net::{ TcpStream };

use quick_xml::

pub trait ServeXMPP {
  fn start() -> String;
  fn handle_requests(&self);
  fn handle_connection(&self, stream: TcpStream) -> Result<(), Err>;
  fn parse_xml(&self, buff: &[u8]);
  fn update_state(&self, Vec<String>) -> UnoState;
  fn send_message(&self, state: UnoState, state: TcpStream) -> Result<(), Err>;
}