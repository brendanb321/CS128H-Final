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
        }
    }
    
}