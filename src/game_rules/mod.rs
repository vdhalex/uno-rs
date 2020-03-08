pub mod unostate;

pub trait gameState {
    fn new(cards: Vec<String>) -> Self;
    fn begin_play(
        &mut self,
        input: impl BufRead,
        output: impl Write,
        error: impl Write) -> Result<(), Error>;
    fn get_move();
    fn update_state();
    fn shuffle();
    fn send_cards();
    fn check_winner();
}