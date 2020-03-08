pub mod unostate;

pub trait gameState {
    fn new() -> Self;
    fn get_move();
    fn update_state();
    fn shuffle();
    fn send_cards();
    fn check_winner();
}