// Simple utility to convert an Othello board to an SVG file representation.

use std::env;

// use std::fs::File;
// use std::io::Read;

extern crate othlib;

use othlib::game::wthor::{WThorFile, WThorGame, WThorPlayer};

// single argument: output file location
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("read_wthor <wthor_file>");
        ::std::process::exit(1);
    }

    let wthor_file = WThorFile::<WThorPlayer>::new(&args[1]);
    println!("{:?}", wthor_file);

    let games = WThorFile::<WThorGame>::new(&args[1]);
    println!("{:?}", games);

    // let mut f = File::open(&args[1]).unwrap();
    // let mut buffer = [0; 50];
    // f.read_exact(&mut buffer).unwrap();
    // for e in buffer.iter() {
    //     println!("{}", *e as char);
    // }
}
