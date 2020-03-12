use crate::player::unoplayer::UnoCard;

pub mod unoplayer;

pub trait GamePlayer {
    fn new() -> Self;
//    fn draw(&self, &mut Vec<u8>);
    fn add_cards(&mut self, cards: &mut Vec<UnoCard>);
    fn show_cards(&self) -> &[UnoCard];
    fn remove_card(&mut self, _: &UnoCard);
}