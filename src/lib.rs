use std::collections::HashSet;

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

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
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

    pub fn at_loc(&mut self, location: Location) -> Option<Piece> {
        self.board[location.row][location.col]
    }

    pub fn at(&mut self, row: usize, col: usize) -> Option<Piece> {
        self.board[row][col]
    }
    
    pub fn move_to(&mut self, mut piece: Piece, new_location: Location) -> Result<(), InvalidMove> {
        // check if move is valid
        if !valid_location(new_location) {
            return Err(InvalidMove::OutOfBounds)
        } else if self.at_loc(new_location).is_some() && self.at_loc(new_location).unwrap().player == piece.player {
            return Err(InvalidMove::AlreadyOccupiedBySamePlayer)
        } else {
            // make sure the new_location is in the range of the piece
            let range = piece.range(self);
            if range.contains(&new_location) {
                let old_row = piece.location.row;
                let old_col = piece.location.col;
                piece.location = new_location;
                self.board[new_location.row][new_location.col] = Some(piece);
                self.board[old_row][old_col] = None;
            }
            
        }
        Ok(())
    }

    pub fn to_string(&mut self) -> String {
        let mut str: String = String::new();
        str.push_str("-------------------------");
        
        let mut i = 7;
        loop {
            str.push_str("\n|");
            let mut j = 0;
            while j < 8 {
                let entry = self.board[i][j];
                if entry.is_none() {
                    str.push_str("  ");
                } else {
                    if entry.unwrap().player == Player::Player1 {
                        str.push_str("1");
                    } else {
                        str.push_str("2");
                    }
                    if entry.unwrap().name == PieceType::Pawn {
                        str.push_str("P");
                    } else if entry.unwrap().name == PieceType::Rook {
                        str.push_str("R");
                    } else if entry.unwrap().name == PieceType::Knight {
                        str.push_str("N");
                    } else if entry.unwrap().name == PieceType::Bishop {
                        str.push_str("B");
                    } else if entry.unwrap().name == PieceType::Queen {
                        str.push_str("Q");
                    } else {
                        str.push_str("K");
                    }
                }
                str.push_str("|");
                j = j+1;
            }
            str.push_str(" ");
            str.push_str(&(i+1).to_string());
            if i == 0 {
                break;
            }
            str.push_str("\n-------------------------");
            i = i-1;
        }
        str.push_str("\n-------------------------");
        str.push_str("\n A  B  C  D  E  F  G  H");
        return str;
    }
}

impl Piece {
    pub fn new(name: PieceType, player: Player, location: Location) -> Piece {
        Self{name, player, location}
    }

    pub fn range(&mut self, board: &mut Board) -> HashSet<Location> {
        
        let mut set: HashSet<Location> = std::collections::HashSet::new();
        let row = self.location.row;
        let col = self.location.col;
        match self.name {
            PieceType::Pawn => {
                if self.player == Player::Player1 {
                    if board.at(row+1,col).is_none() {
                        set.insert(Location::new(row+1, col));
                    }
                    if col < 7 && board.at(row+1, col+1).is_some() && board.at(row+1, col+1).unwrap().player != self.player {
                        set.insert(Location::new(row+1, col+1));
                    }
                    if col > 0 && board.at(row+1, col-1).is_some() && board.at(row+1, col-1).unwrap().player != self.player {
                        set.insert(Location::new(row+1, col-1));
                    }
                } else {
                    if board.at(row-1,col).is_none() {
                        set.insert(Location::new(row-1, col));
                    }
                    if col < 7 && board.at(row-1, col+1).is_some() && board.at(row-1, col+1).unwrap().player != self.player {
                        set.insert(Location::new(row-1, col+1));
                    }
                    if col > 0 && board.at(row-1, col-1).is_some() && board.at(row-1, col-1).unwrap().player != self.player {
                        set.insert(Location::new(row-1, col-1));
                    }
                }
                set
            },
            PieceType::Rook => {
                let mut i = row;
                let mut j = col;
                let mut k = 0;
                loop {
                    if k == 0 {
                        i = i+1;
                    } else if k == 1 {
                        i = i-1;
                    } else if k == 2 {
                        j = j+1;
                    } else if k == 3 {
                        j = j-1;
                    } else {
                        return set;
                    }
                    if valid_location(Location::new(i,j)) && board.at(i,j).is_none() {
                        set.insert(Location::new(i,j));
                    } else if valid_location(Location::new(i,j)) && board.at(i,j).unwrap().player != self.player {
                        set.insert(Location::new(i,j));
                        k = k+1;
                    } else {
                        k = k+1;
                    }
                }
            },
            PieceType::Knight => {
                let mut k = 0;
                loop {
                    let r: usize;
                    let c: usize;
                    if k == 0 {
                        r = row + 2;
                        c = col + 1;
                    } else if k == 1 {
                        r = row + 2;
                        c = col - 1;
                    } else if k == 2 {
                        r = row - 2;
                        c = col + 1;
                    } else if k == 3 {
                        r = row - 2;
                        c = col - 1;
                    } else if k == 4 {
                        r = row + 1;
                        c = col + 2;
                    } else if k == 5 {
                        r = row + 1;
                        c = col - 2;
                    } else if k == 6 {
                        r = row - 1;
                        c = col + 2;
                    } else if k == 7 {
                        r = row - 1;
                        c = col - 2;
                    } else {
                        return set;
                    }

                    let loc = Location::new(r,c);
                    if valid_location(loc) && (board.at_loc(loc).is_none() || board.at_loc(loc).unwrap().player != self.player) {
                        set.insert(loc);
                    }
                    k = k+1;
                }
            },
            PieceType::Bishop => {
                let mut i = row;
                let mut j = col;
                let mut k = 0;
                loop {
                    if k == 0 {
                        i = i+1;
                        j = j+1;
                    } else if k == 1 {
                        i = i+1;
                        j = j-1;
                    } else if k == 2 {
                        i = i-1;
                        j = j+1;
                    } else if k == 3 {
                        i = i-1;
                        j = j-1;
                    } else {
                        return set;
                    }
                    if valid_location(Location::new(i,j)) && board.at(i,j).is_none() {
                        set.insert(Location::new(i,j));
                    }  else if valid_location(Location::new(i,j)) && board.at(i,j).unwrap().player != self.player {
                        set.insert(Location::new(i,j));
                        k = k+1;
                    } else {
                        k = k+1;
                    }
                }
            },
            PieceType::Queen => {
                let mut i = row;
                let mut j = col;
                let mut k = 0;
                while k < 4 {
                    if k == 0 {
                        i = i+1;
                    } else if k == 1 {
                        i = i-1;
                    } else if k == 2 {
                        j = j+1;
                    } else if k == 3 {
                        j = j-1;
                    }
                    if valid_location(Location::new(i,j)) && board.at(i,j).is_none() {
                        set.insert(Location::new(i,j));
                    } else if valid_location(Location::new(i,j)) && board.at(i,j).unwrap().player != self.player {
                        set.insert(Location::new(i,j));
                        k = k+1;
                    } else {
                        k = k+1;
                    }
                }

                i = row;
                j = col;
                k = 0;
                while k < 4 {
                    if k == 0 {
                        i = i+1;
                        j = j+1;
                    } else if k == 1 {
                        i = i+1;
                        j = j-1;
                    } else if k == 2 {
                        i = i-1;
                        j = j+1;
                    } else if k == 3 {
                        i = i-1;
                        j = j-1;
                    }
                    if valid_location(Location::new(i,j)) && board.at(i,j).is_none() {
                        set.insert(Location::new(i,j));
                    }  else if valid_location(Location::new(i,j)) && board.at(i,j).unwrap().player != self.player {
                        set.insert(Location::new(i,j));
                        k = k+1;
                    } else {
                        k = k+1;
                    }
                }

                set
            },
            PieceType::King => {
                let mut k = 0;
                while k < 8 {
                    let mut i: usize = 8;
                    let mut j: usize = 8;
                    if k == 0 {
                        if row > 0 && col > 0 {
                            i = row - 1;
                            j = col - 1;
                        }
                    } else if k == 1 {
                        i = row;
                        if col > 0 {
                            j = col - 1;
                        }
                    } else if k == 2 {
                        i = row + 1;
                        if col > 0 {
                            j = col - 1;
                        }
                    } else if k == 3 {
                        if row > 0 {
                            i = row - 1;
                        }
                        j = col;
                    } else if k == 4 {
                        i = row + 1;
                        j = col;
                    } else if k == 5 {
                        if row > 0 {
                            i = row - 1;
                        }
                        j = col + 1;
                    } else if k == 6 {
                        i = row;
                        j = col + 1;
                    } else if k == 7 {
                        i = row + 1;
                        j = col + 1;
                    }

                    let loc = Location::new(i,j);
                    if valid_location(loc) && (board.at_loc(loc).is_none() || board.at_loc(loc).unwrap().player != self.player) {
                        set.insert(loc);
                    }

                    k = k+1;
                }
                set
            }
        }
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