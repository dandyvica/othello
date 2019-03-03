// Simple utility to convert Othello WTHOR files to plain CSV

use std::env;

extern crate othlib;

use othlib::game::wthor::{WThorFile, WThorGame, WThorPlayer, WThorTournament};

// used to convert to algebric

// single argument: output file location
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("wthor2csv <players> <tournaments> <games>");
        ::std::process::exit(1);
    }

    // load WTHOR files
    let players = WThorFile::<WThorPlayer>::new(&args[1]);
    let tournaments = WThorFile::<WThorTournament>::new(&args[2]);
    let games = WThorFile::<WThorGame>::new(&args[3]);

    // now read game file and print out data as CSV
    for game in &games.data {
        // convert moves
        let move_alg: Vec<_> = game.moves.iter().map(|x| convert_to_algebric(x)).collect();

        // get players & tournaments labels
        let tournament = &tournaments.data[game.tournament_title_id as usize].tournament;
        let black_player = &players.data[usize::from(game.black_player_id)].player;
        let white_player = &players.data[usize::from(game.white_player_id)].player;

        println!(
            "{};{};{};{};{};{}",
            tournament,
            black_player,
            white_player,
            game.black_pieces_id,
            game.theoretical_score,
            move_alg.join("-")
        );
    }
}

// simple function to convert from WTHOR move to algebric move
fn convert_to_algebric(r#move: &u8) -> String {
    const ASCII_UPPER: [char; 8] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];

    // tenth is index on letters
    let letter = ASCII_UPPER[(r#move % 10) as usize - 1];
    let digit = format!("{}", r#move / 10);

    format!("{}{}", letter, digit)
}
