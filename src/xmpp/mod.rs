pub mod xmppserver;

pub trait ServeXMPP {
  fn new() -> Self;
  fn send_message();
}