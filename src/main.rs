use std::{io::Read, ascii::AsciiExt};

#[warn(unused_imports)]
use cs128_final::{Board,Player};
pub fn main() {
    let mut board = Board::new();
    let mut player = Player::Player1;
    board.fill();
    loop {
        println!("{}", board.to_string());
        if player == Player::Player1 {
            println!("Player 1's Turn. Enter coordinates of piece as \"X0\": ");
        } else {
            println!("Player 2's Turn. Enter coordinates of piece as \"X0\": ");
        }

        let mut input = String::new();
        loop {
            std::io::stdin().read_line(&mut input).expect("Unable to read entry.");
            println!("input {}", input);
            if input.chars().count() != 3 { // two chars plus \n, see https://stackoverflow.com/questions/46290655/get-the-string-length-in-characters-in-rust
                println!("Invalid input 1. Try again.");
                continue;
            } else {
                input.pop();
                let b = input.pop();
                let a = input.pop();
                println!("a{} b{}", a.unwrap(), b.unwrap());
                if a.is_none() || !a.unwrap().is_alphabetic() 
                || b.is_none() || !b.unwrap().is_alphanumeric() {
                    println!("Invalid input 2. Try again.");
                    continue;
                }
                let col_plus_10 = a.unwrap().to_ascii_lowercase().to_digit(26);
                if col_plus_10.is_none() || col_plus_10.unwrap() >= 18 || col_plus_10.unwrap() < 10 {
                    println!("Invalid input 3. Try again.");
                    continue;
                }
                let row_plus_1 = b.unwrap().to_digit(10);
                if row_plus_1.is_none() || row_plus_1.unwrap() >= 9 || row_plus_1.unwrap() < 1 {
                    println!("Invalid input 4. Try again.");
                    continue;
                }
                let mut piece = board.at((row_plus_1.unwrap() - 1).try_into().unwrap(), (col_plus_10.unwrap() - 10).try_into().unwrap());
                if piece.is_none() || piece.unwrap().player != player {
                    println!("Invalid location 5. Try again.");
                    continue;
                }
                println!("Should be valid till here");
                
            } 
        }
    }
    
}