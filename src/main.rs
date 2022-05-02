use cs128_final::{Board, Player, Location, PieceType, Piece};
pub fn main() {
    let mut board: Board = Board::new();
    let mut player = Player::Player1;
    board.fill();
    loop {
        println!("{}", board.to_string());
        if player == Player::Player1 {
            println!("Player 1's Turn. Enter coordinates of piece as \"X0\": ");
        } else {
            println!("Player 2's Turn. Enter coordinates of piece as \"X0\": ");
        }

        let start_pair = read_input_location();
        let start_coord = start_pair.0;
        let start = start_pair.1;
        let piece = board.at_loc(start);
        if piece.is_none() || piece.unwrap().player != player {
            println!("Invalid location - No valid piece at location. Try again.");
            continue;
        }

        println!("Enter coordinates of new location: ");
        let end_pair = read_input_location();
        let end_coord = end_pair.0;
        let end = end_pair.1;
        if piece.unwrap().range(&mut board).contains(&end) {
            board.move_to(piece.unwrap(), end);
            println!("Moved {:?} {:?} from {} to {}", 
            piece.unwrap().player, piece.unwrap().name, start_coord.to_ascii_uppercase(), end_coord.to_ascii_uppercase());
            if piece.unwrap().name == PieceType::Pawn && (end.row == 0 || end.row == 7) {
                board.board[end.row][end.col] = Some(Piece::new(PieceType::Queen, piece.unwrap().player, end));
                println!("Pawn became Queen at {}", end_coord.to_ascii_uppercase());
            }
        } else {
            println!("Invalid location - Not in range. Try again.");
            continue;
        }

        //if player == Player::Player1 {
        //    player = Player::Player2;
        //} else {
        //    player = Player::Player1;
        //}
    }
}

pub fn read_input_location() -> (String, Location) {
    loop {
        let mut input: String = String::from("");
        std::io::stdin().read_line(&mut input).expect("Unable to read entry.");
        if input.chars().count() != 3 { // two chars plus \n, see https://stackoverflow.com/questions/46290655/get-the-string-length-in-characters-in-rust
            println!("Invalid input. (1) Try again.");
            continue;
        }
        input.pop();
        let copy = input.clone();
        let b = input.pop();
        let a = input.pop();
        if a.is_none() || !a.unwrap().is_alphabetic() 
        || b.is_none() || !b.unwrap().is_alphanumeric() {
            println!("Invalid input. (2) Try again.");
            continue;
        }
        let col_plus_10 = a.unwrap().to_ascii_lowercase().to_digit(26);
        if col_plus_10.is_none() || col_plus_10.unwrap() >= 18 || col_plus_10.unwrap() < 10 {
            println!("Invalid input. (3) Try again.");
            continue;
        }
        let row_plus_1 = b.unwrap().to_digit(10);
        if row_plus_1.is_none() || row_plus_1.unwrap() >= 9 || row_plus_1.unwrap() < 1 {                    
            println!("Invalid input. (4) Try again.");
            continue;
        }
        let row: usize = (row_plus_1.unwrap() - 1).try_into().unwrap();
        let col: usize = (col_plus_10.unwrap() - 10).try_into().unwrap();
        
        return (copy, Location::new(row, col));
    }
}