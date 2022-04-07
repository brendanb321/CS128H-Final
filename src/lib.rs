
const BOARD_SIZE: usize = 8;

#[derive(Debug, Clone, Copy)]
pub struct Board {
    pub board: [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE]
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub name: PieceType,
    pub player: Player,
    pub location: Location,
}

#[derive(Debug, Clone, Copy)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Player {
    Player1,
    Player2
}

#[derive(Debug, Clone, Copy)]
pub struct Location {
    pub row: usize,
    pub col: usize
}

#[derive(Debug)]
pub enum InvalidMove {
    AlreadyOccupiedBySamePlayer,
    OutOfBounds,
    NotInRange
}

impl Board {
    pub fn new() -> Board {
        let board: [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE] = [[None; BOARD_SIZE]; BOARD_SIZE];
        Self{board}
    }

    // fill in board with pieces in default position
    pub fn fill(&mut self) {
        self.board[0][0] = Some(Piece::new(PieceType::Rook,   Player::Player1, Location::new(0, 0)));
        self.board[0][1] = Some(Piece::new(PieceType::Knight, Player::Player1, Location::new(0, 1)));
        self.board[0][2] = Some(Piece::new(PieceType::Bishop, Player::Player1, Location::new(0, 2)));
        self.board[0][3] = Some(Piece::new(PieceType::Queen,  Player::Player1, Location::new(0, 3)));
        self.board[0][4] = Some(Piece::new(PieceType::King,   Player::Player1, Location::new(0, 4)));
        self.board[0][5] = Some(Piece::new(PieceType::Bishop, Player::Player1, Location::new(0, 5)));
        self.board[0][6] = Some(Piece::new(PieceType::Knight, Player::Player1, Location::new(0, 6)));
        self.board[0][7] = Some(Piece::new(PieceType::Rook,   Player::Player1, Location::new(0, 7)));
        self.board[7][0] = Some(Piece::new(PieceType::Rook,   Player::Player2, Location::new(7, 0)));
        self.board[7][1] = Some(Piece::new(PieceType::Knight, Player::Player2, Location::new(7, 1)));
        self.board[7][2] = Some(Piece::new(PieceType::Bishop, Player::Player2, Location::new(7, 2)));
        self.board[7][3] = Some(Piece::new(PieceType::Queen,  Player::Player2, Location::new(7, 3)));
        self.board[7][4] = Some(Piece::new(PieceType::King,   Player::Player2, Location::new(7, 4)));
        self.board[7][5] = Some(Piece::new(PieceType::Bishop, Player::Player2, Location::new(7, 5)));
        self.board[7][6] = Some(Piece::new(PieceType::Knight, Player::Player2, Location::new(7, 6)));
        self.board[7][7] = Some(Piece::new(PieceType::Rook,   Player::Player2, Location::new(7, 7)));
        for col in 0 .. BOARD_SIZE {
            self.board[1][col] = Some(Piece::new(PieceType::Pawn,   Player::Player1, Location::new(1, col)));
            self.board[6][col] = Some(Piece::new(PieceType::Pawn,   Player::Player2, Location::new(6, col)));
        }
    }

    pub fn at(&mut self, location: Location) -> Option<Piece> {
        self.board[location.row][location.col]
    }
}

impl Piece {
    pub fn new(name: PieceType, player: Player, location: Location) -> Piece {
        Self{name, player, location}
    }

    pub fn move_to(&mut self, new_location: Location, board: &mut Board) -> Result<(), InvalidMove> {
        // check if move is valid
        if !valid_location(new_location) {
            return Err(InvalidMove::OutOfBounds)
        } else if board.at(new_location).is_some() && board.at(new_location).unwrap().player == self.player {
            return Err(InvalidMove::AlreadyOccupiedBySamePlayer)
        } else {
            // make sure the new_location is in the range of the piece
        }
        Ok(())
    }
}

impl Location {
    pub fn new(row: usize, col: usize) -> Location {
        Self{row, col}
    }
}

pub fn valid_location(location: Location) -> bool {
    location.row < BOARD_SIZE && location.col < BOARD_SIZE
}