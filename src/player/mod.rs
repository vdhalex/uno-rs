use crate::player::unoplayer::UnoCard;

pub mod unoplayer;

pub trait GamePlayer {
    fn new() -> Self;
    fn add_cards(&mut self, cards: &mut Vec<UnoCard>);
    fn show_cards(&mut self) -> &mut [UnoCard];
    fn remove_card(&mut self, _: &UnoCard) -> bool;
}
