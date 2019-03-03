// WTHOR files are real games file
// Data format is described here: http://www.ffothello.org/wthor/Format_WThor.pdf

use std::fs::File;
use std::io::Read;
use std::mem;

use byteorder::{LittleEndian, ReadBytesExt};

// trait to implement to read specific parts of WTHOR files: games, players, ...
pub trait WThorable<T> {
    fn read_specific_data(wthor_file: &mut File) -> T;
    fn get_number_of_records(n1_or_n2: (u32, u16)) -> u32;
}

// This header is common to all WTHOR files
#[derive(Debug)]
pub struct WThorFileHeader {
    file_date: u32, // 4 bytes for describing WTHOR file date
    n1: u32,        // n1 is the number of games in a WTHOR game file
    n2: u16, // n1 is either the number of players in a players file or the number of tournaments in a tournament file
    game_year: u16,
    p1: u8, // p1 gives the board size: 0 or 8 for 8x8, 10 for 10x10 boards
    p2: u8, // N/A for game files
    p3: u8, // not used yet
    reserved: u8,
}

impl WThorFileHeader {
    pub fn new(wthor_file: &mut File) -> Self {
        WThorFileHeader {
            file_date: wthor_file.read_u32::<LittleEndian>().unwrap(),
            n1: wthor_file.read_u32::<LittleEndian>().unwrap(),
            n2: wthor_file.read_u16::<LittleEndian>().unwrap(),
            game_year: wthor_file.read_u16::<LittleEndian>().unwrap(),
            p1: wthor_file.read_u8().unwrap(),
            p2: wthor_file.read_u8().unwrap(),
            p3: wthor_file.read_u8().unwrap(),
            reserved: wthor_file.read_u8().unwrap(),
        }
    }
}

// For 8x8 or 10x10 games
#[derive(Debug)]
pub struct WThorGame {
    pub tournament_title_id: u16,
    pub black_player_id: u16,
    pub white_player_id: u16,
    pub black_pieces_id: u8,
    pub theoretical_score: u8,
    pub moves: Vec<u8>,
}

impl WThorable<WThorGame> for WThorGame {
    fn read_specific_data(wthor_file: &mut File) -> Self {
        WThorGame {
            tournament_title_id: wthor_file.read_u16::<LittleEndian>().unwrap(),
            black_player_id: wthor_file.read_u16::<LittleEndian>().unwrap(),
            white_player_id: wthor_file.read_u16::<LittleEndian>().unwrap(),
            black_pieces_id: wthor_file.read_u8().unwrap(),
            theoretical_score: wthor_file.read_u8().unwrap(),
            moves: {
                // 60 is the maximum number of moves (64-4)
                let mut buffer = [0; 60];

                // read 60 bytes, convert to a vector and delete 0 because the game is over
                wthor_file.read_exact(&mut buffer).unwrap();

                let mut moves = buffer.to_vec();
                println!("{:?}", moves);
                moves.retain(|&x| x != 0);

                // as bytes represent row,col coordinates, the spread from 11 to 88. We can test this here
                if !moves.iter().all(|&x| (x >= 11) & (x <= 88)) {
                    eprintln!("{:?}", moves);
                }

                moves
            },
        }
    }

    fn get_number_of_records(n1_or_n2: (u32, u16)) -> u32 {
        n1_or_n2.0
    }
}

// A WTHOR file for the players
#[derive(Debug)]
pub struct WThorPlayer {
    pub player: String,
}

// remove NULL chars
const PLAYER_LENGTH: usize = 20;

impl WThorable<WThorPlayer> for WThorPlayer {
    fn read_specific_data(wthor_file: &mut File) -> Self {
        WThorPlayer {
            player: {
                // WTHOR spec states a player name is a least 19 chars + \0
                let mut buffer = [0; PLAYER_LENGTH];
                wthor_file.read_exact(&mut buffer).unwrap();
                String::from_utf8_lossy(&buffer.to_vec()).replace("\0", "")
            },
        }
    }

    fn get_number_of_records(n1_or_n2: (u32, u16)) -> u32 {
        n1_or_n2.1 as u32
    }
}

// A WTHOR file for the tournaments
const TOURNEMENT_LENGTH: usize = 26;

#[derive(Debug)]
pub struct WThorTournament {
    pub tournament: String,
}

// remove NULL chars
impl WThorable<WThorTournament> for WThorTournament {
    fn read_specific_data(wthor_file: &mut File) -> Self {
        WThorTournament {
            tournament: {
                // WTHOR spec states a player name is a least 19 chars + \0
                let mut buffer = [0; TOURNEMENT_LENGTH];
                wthor_file.read_exact(&mut buffer).unwrap();
                String::from_utf8_lossy(&buffer.to_vec()).replace("\0", "")
            },
        }
    }

    fn get_number_of_records(n1_or_n2: (u32, u16)) -> u32 {
        n1_or_n2.1 as u32
    }
}

// A WTHOR file is either describing games, players, tournaments...
#[derive(Debug)]
pub struct WThorFile<T> {
    pub header: WThorFileHeader,
    pub data: Vec<T>,
}

impl<T> WThorFile<T>
where
    T: WThorable<T>,
{
    pub fn new(wthor_file_name: &str) -> Self {
        // open file
        let mut wthor_file = File::open(wthor_file_name).expect("Unable to open WTHOR file");

        // get header to get the number of records
        let header = WThorFileHeader::new(&mut wthor_file);

        // read specific data and add them to the vector
        let mut buffer = Vec::<T>::with_capacity(mem::size_of::<T>());

        // as n1 or n2 gives the number of records, ask for it
        let n = T::get_number_of_records((header.n1, header.n2));

        // just add our specific data
        for _i in 1..=n {
            buffer.push(T::read_specific_data(&mut wthor_file));
        }

        // close file
        drop(wthor_file);

        WThorFile {
            header: header,
            data: buffer,
        }
    }
}
