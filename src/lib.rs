
const BOARD_SIZE: u32 = 8;

#[derive(Debug)]
pub struct Board {
    pub board: Vec<Vec<Option<Piece>>>
}

#[derive(Debug)]
pub struct Piece {
    pub name: PieceType,
    pub player: Player,
    pub location: Location,
}

#[derive(Debug)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}

#[derive(Debug)]
pub enum Player {
    Player1,
    Player2
}

#[derive(Debug)]
pub struct Location {
    pub row: u32,
    pub col: u32
}

impl Board {
    pub fn new() -> Board {
        let mut board: Vec<Vec<Option<Piece>>> = Vec::new();
        for _col in 0 .. BOARD_SIZE {
            let mut board_row: Vec<Option<Piece>> = Vec::new();
            for _row in 0 .. BOARD_SIZE {
                board_row.push(None);
            }
            board.push(board_row);
        }
        Self{board}
    }

    // fill in board with pieces in default position
    pub fn fill(&mut self) {
        self.board[0][0] = Some(Piece::new(PieceType::Rook, Player::Player1, Location::new(0, 0)));
    }
}

impl Piece {
    pub fn new(name: PieceType, player: Player, location: Location) -> Piece {
        Self{name, player, location}
    }
}

impl Location {
    pub fn new(row: u32, col: u32) -> Location {
        Self{row, col}
    }
}