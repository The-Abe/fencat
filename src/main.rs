use regex::Regex;
use std::io;

const FEN_REGEX: &str = r"([rnbqkpRNBQKP1-8]+\/){7}([rnbqkpRNBQKP1-8]+)";
const BLACK_PAWN: &str = " ♟︎ ";
const BLACK_KNIGHT: &str = " ♞ ";
const BLACK_BISHOP: &str = " ♝ ";
const BLACK_ROOK: &str = " ♜ ";
const BLACK_QUEEN: &str = " ♛ ";
const BLACK_KING: &str = " ♚ ";
const WHITE_PAWN: &str = " ♙ ";
const WHITE_KNIGHT: &str = " ♘ ";
const WHITE_BISHOP: &str = " ♗ ";
const WHITE_ROOK: &str = " ♖ ";
const WHITE_QUEEN: &str = " ♕ ";
const WHITE_KING: &str = " ♔ ";
const BACKGROUND_COLOR: &str = "\x1b[48;5;240m";

// Author: Abe van der Wielen <the-abe@github>
fn main() -> io::Result<()> {
    // Check if the user has provided a FEN string in stdin.
    // If not, print a message and exit.
    let mut fen = String::new();
    io::stdin().read_line(&mut fen)?;

    if Regex::is_match(&Regex::new(&FEN_REGEX).unwrap(), fen.as_str()) {
        let board_lines = split_fen(fen);
        board_lines.iter().for_each(|line| println!("{}", chessify(line)));
    } else {
        println!("FEN string is invalid.");
        println!("Please provide a valid FEN string.");
        println!("Example: rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    }

    Ok(())
}

fn split_fen(fen: String) -> Vec<String> {
    let stripped_fen = Regex::captures(&Regex::new(&FEN_REGEX).unwrap(), fen.as_str())
        .unwrap()     // Safe to unwrap because we know the regex matches.
        .get(0)       // Get the first match.
        .unwrap()     // Safe to unwrap because we know the regex matches.
        .as_str()     // Get the string from the match.
        .to_string(); // Convert the string slice to a String.
    let mut board_lines: Vec<String> = Vec::new();

    for line in stripped_fen.split("/") {
        board_lines.push(line.to_string());
    }

    board_lines
}

fn chessify(line: &String) -> String {
    let mut chessified_line = String::new();

    let mut square_counter = 0;
    for character in line.chars() {
        square_counter += 1;
        if character.is_numeric() {
            let mut empty_squares = String::new();
            let empty_square_count = character.to_digit(10).unwrap();

            for _ in 0..empty_square_count {
                empty_squares.push_str("   ");
            }

            chessified_line.push_str(empty_squares.as_str());
        } else {
            chessified_line.push_str(match character {
                'r' => BLACK_ROOK,
                'n' => BLACK_KNIGHT,
                'b' => BLACK_BISHOP,
                'q' => BLACK_QUEEN,
                'k' => BLACK_KING,
                'p' => BLACK_PAWN,
                'R' => WHITE_ROOK,
                'N' => WHITE_KNIGHT,
                'B' => WHITE_BISHOP,
                'Q' => WHITE_QUEEN,
                'K' => WHITE_KING,
                'P' => WHITE_PAWN,
                _ => " ",
            });
        }
    }

    chessified_line
}
