use std::collections::HashSet;
use std::io::{stderr, stdin, stdout, BufRead, BufReader, Write};
use std::path::PathBuf;
use std::{env, fs, process};
use uno::game_rules::unostate::unoState;
use uno::player::unoplayer::{unoPlayer, unoCard, CardType, ColorType};

mod errors;
use errors::*;
use std::fs::File;
use uno::game_rules::gameState;

fn main() {
    finish(interact(stdin().lock(), stdout(), stderr()));
}

fn interact(
    input: impl BufRead,
    mut output: impl Write,
    mut error: impl Write,
) -> Result<(), Error>{
    let mut new_game_state = unoState::new();
    new_game_state.begin_play();
    Ok(())
}

fn finish(result: Result<(), Error>) -> ! {
    let code = match result {
        Ok(()) => 0,
        Err(e) => {
            eprintln!(
                "{}: {}",
                env::args().next().unwrap_or_else(|| "graph".into()),
                e
            );
            1
        }
    };
    process::exit(code);
}