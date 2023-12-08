// Title:        fencat
// Author:       Abe van der Wielen <info@avdw.dev>
// Github:       github.com/the-abe/fencat
// Description:  A simple FEN viewer.
// Usage:        fencat (--flip) [FILE]
// Example:      echo rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR | fencat
// Example:      fencat fen.txt
// TODO:         Add support for FEN strings with move counters.
// TODO:         Add support for FEN strings with castling availability.
// TODO:         Add support for FEN strings with color to move.

use regex::Regex;
use std::{env, io};

// Currently only cares about the board position.
// See: https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation#Definition
const FEN_REGEX: &str = r"([rnbqkpRNBQKP1-8]+\/){7}([rnbqkpRNBQKP1-8]+)";

// ANSI escape codes for colors.
// TODO: Make these configurable. Readability is important. Maybe use preset color schemes?
// Default colors chosen to make sure both white and black pieces are visible on the background.
const BACKGROUND_DARK: &str = "\x1b[48;5;246m";
const BACKGROUND_LIGHT: &str = "\x1b[48;5;249m";
const RESET_COLOR: &str = "\x1b[0m";

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    // Check if the user has provided a FEN string in a file or stdin.
    let mut fen = match args.len() {
        2 => match std::fs::read_to_string(args[1].as_str()) {
            Ok(fen) => fen,
            Err(_) => String::new(),
        },
        3 => match std::fs::read_to_string(args[2].as_str()) {
            Ok(fen) => fen,
            Err(_) => String::new(),
        },
        _ => String::new(),
    };
    // If the user has not provided a FEN string in a file, check if the user has provided a FEN in
    // stdin.
    if fen.is_empty() {
        io::stdin().read_line(&mut fen)?;
    }

    // Check if the FEN string is present and valid and print an error if it is not.
    if fen.is_empty() || !Regex::is_match(&Regex::new(&FEN_REGEX).unwrap(), fen.as_str()) {
        println!("No FEN string provided or not readable.");
        usage();
        std::process::exit(1);
    }

    // Set flip if --flip is passed as an argument.
    let flip = if args.len() > 1 {
        args[1] == "--flip" || args[1] == "-f"
    } else {
        false
    };

    // Split the FEN string into lines.
    let board_lines = split_fen(fen);

    // Print the board in the correct orientation.
    // Orientation is determined by the flip argument and changes:
    //  - The order of the lines.
    //  - The order of the characters in each line.
    //  - The numbering of the ranks and files.
    if flip {
        println!("   h  g  f  e  d  c  b  a");
        for (i, line) in board_lines.iter().rev().enumerate() {
            println!("{} {} {}", i + 1, chessify(line, i % 2 == 0, flip), i + 1);
        }
        println!("   h  g  f  e  d  c  b  a");
    } else {
        println!("   a  b  c  d  e  f  g  h");
        for (i, line) in board_lines.iter().enumerate() {
            println!("{} {} {}", 8 - i, chessify(line, i % 2 == 0, flip), 8 - i);
        }
        println!("   a  b  c  d  e  f  g  h");
    }
    Ok(())
}

// Print the usage information.
// TODO: Make this more comprehensive.
fn usage() -> () {
    println!("Usage: fencat (--flip) [FILE]");
    println!("Example: echo rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR | fencat");
    println!("Example: fencat fen.txt");
}

// Split the FEN string into lines.
// Takes the whole string and extracts the board through a regex.
// Garbage before and after the board is ignored.
fn split_fen(fen: String) -> Vec<String> {
    let stripped_fen = Regex::captures(&Regex::new(&FEN_REGEX).unwrap(), fen.as_str())
        .unwrap() // Safe to unwrap because we know the regex matches.
        .get(0) // Get the first match.
        .unwrap() // Safe to unwrap because we know the regex matches.
        .as_str() // Get the string from the match.
        .to_string(); // Convert the string slice to a String.

    // Split the string into chessboard ranks and turn them into Strings.
    // We can't return a str because the size if not known at compile time.
    stripped_fen.split("/").map(|s| s.to_string()).collect()
}

// Convert a line of FEN to a line of chessboard.
// Arguments:
// - line: The line of FEN to convert.
// - even: Whether the line is an even or odd rank for the purposes of coloring.
// - reversed: Whether the line should be reversed for the purposes of orientation
fn chessify(line: &String, even: bool, reversed: bool) -> String {
    // The output string.
    let mut chessified_line = String::new();

    // The square counter keeps track of the current square.
    // It is used to determine the color of the square with modulo 2.
    // Start at 0 for even ranks and 1 for odd ranks.
    let mut square_counter = match even {
        true => 0,
        false => 1,
    };

    // Reverse the line if necessary.
    let ordered_line = if reversed {
        line.chars().rev().collect::<String>()
    } else {
        line.to_string()
    };

    // Iterate over the characters in the line.
    // If the character is a number, add that many empty squares to the output.
    // If the character is a piece, add that piece to the output.
    for character in ordered_line.chars() {
        if character.is_numeric() {
            // Safe to unwrap because we know the character is numeric.
            let empty_square_count = character.to_digit(10).unwrap();
            // Add empty squares to the output.
            // Alternate the color of the squares.
            for _ in 0..empty_square_count {
                square_counter += 1;
                chessified_line.push_str(match square_counter % 2 {
                    0 => BACKGROUND_DARK,
                    _ => BACKGROUND_LIGHT,
                });
                chessified_line.push_str("   ");
            }
        } else {
            square_counter += 1;
            // Alternate the color of the squares.
            chessified_line.push_str(match square_counter % 2 {
                0 => BACKGROUND_DARK,
                _ => BACKGROUND_LIGHT,
            });
            // Add the piece to the output with the correct color.
            // Add a space after the piece to make sure the squares are the correct width.
            chessified_line.push_str(match character {
                'r' => "\u{1b}[38;5;0m ♜ ",
                'n' => "\u{1b}[38;5;0m ♞ ",
                'b' => "\u{1b}[38;5;0m ♝ ",
                'q' => "\u{1b}[38;5;0m ♛ ",
                'k' => "\u{1b}[38;5;0m ♚ ",
                'p' => "\u{1b}[38;5;0m ♟︎ ",
                'R' => "\u{1b}[38;5;231m ♜ ",
                'N' => "\u{1b}[38;5;231m ♞ ",
                'B' => "\u{1b}[38;5;231m ♝ ",
                'Q' => "\u{1b}[38;5;231m ♛ ",
                'K' => "\u{1b}[38;5;231m ♚ ",
                'P' => "\u{1b}[38;5;231m ♟︎ ",
                _ => " ", // Should never happen.
            });
        }
        // Reset the color to the default so newlines are not colored.
        chessified_line.push_str(RESET_COLOR);
    }

    chessified_line
}
