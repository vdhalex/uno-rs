use std::collections::HashSet;
use std::io::{stderr, stdin, stdout, BufRead, BufReader, Write, Read};
use std::path::PathBuf;
use std::{env, fs, process};
use uno::game_rules::unostate::UnoState;
use uno::player::unoplayer::{UnoPlayer, UnoCard, CardType, ColorType};
use std::net::TcpStream;
use std::io::prelude::*;
use quick_xml::{ Reader, Writer };

mod errors;
use errors::*;
use std::fs::File;
use uno::game_rules::GameState;

fn main() {
    finish(interact(stdin().lock(), stdout(), stderr()));
    // test_xml();
}

// fn test_xml() -> std::io::Result<()> {
//     let mut stream = TcpStream::connect("127.0.0.1:5222")?;
//     let mut buffer: Vec<u8> = vec![];

//     let request = "
//         <message id=kkkk from='shanks'>
//             <player-turn>3<player-turn/>
//             <last-card>hashlabel</last-card>
//             <deck-encrypted>b'000000'etcetc</deck-encrypted>
//             <current-action-state>string/draw2, draw4 or None</current-action-state>
//         </message>";

//     stream.write(request.as_bytes())?;
//     stream.read(&mut buffer)?;

//     println!("{:?}", &buffer);

//     Ok(())
// }

fn interact(
    input: impl BufRead,
    output: impl Write,
    error: impl Write,
) -> Result<(), Error>{
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