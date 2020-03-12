use std::io::{stderr, stdin, stdout, BufRead, Write};
use std::{env, process};
use uno::game_rules::unostate::UnoState;

mod errors;
use errors::*;
use uno::game_rules::GameState;

fn main() {
    finish(interact(stdin().lock(), stdout(), stderr()));
}

fn interact(input: impl BufRead, output: impl Write, error: impl Write) -> Result<(), Error> {
    let mut new_game_state = UnoState::new();
    new_game_state.begin_play(input, output, error);
    Ok(())
}

fn finish(result: Result<(), Error>) -> ! {
    let code = match result {
        Ok(()) => 0,
        Err(e) => {
            eprintln!(
                "{}: {}",
                env::args().next().unwrap_or_else(|| "uno".into()),
                e
            );
            1
        }
    };
    process::exit(code);
}
