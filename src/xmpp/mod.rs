pub mod servexmpp;
use std::error::Error;
use std::net::TcpStream;

pub trait ServeXMPP {
    fn start(&self);
    fn handle_requests(&mut self);
    fn handle_connection(&mut self, stream: TcpStream) -> Result<(), Box<dyn Error>>;
    fn update_state(&mut self, xml: Vec<String>);
    fn send_message(&self, stream: TcpStream) -> Result<(), Box<dyn Error>>;
}
