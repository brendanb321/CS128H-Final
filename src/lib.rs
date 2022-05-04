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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    Player1,
    Player2
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Location {
    pub row: usize,
    pub col: usize
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
    
    pub fn move_to(&mut self, mut piece: Piece, new_location: Location) {
        let old_row = piece.location.row;
        let old_col = piece.location.col;
        piece.location = new_location;
        self.board[new_location.row][new_location.col] = Some(piece);
        self.board[old_row][old_col] = None;
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
                    if row == 1 && board.at(row+2,col).is_none() {
                        set.insert(Location::new(row+2, col));
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
                    if row == 6 && board.at(row-2, col).is_none() {
                        set.insert(Location::new(row-2, col));
                    }
                }
                set
            },
            PieceType::Rook => {
                let mut i: i32 = row as i32;
                let mut j: i32 = col as i32;
                let mut k = 0;
                while k < 4 {
                    if k < 2 {
                        i = i + (-1 as i32).pow(k);
                    } else {
                        j = j + (-1 as i32).pow(k);
                    }

                    if i < 0 || j < 0 {
                        k = k+1;
                        i = row as i32;
                        j = col as i32;
                        continue;
                    }
                    let loc = Location::new(i.try_into().unwrap(), j.try_into().unwrap());
                    if valid_location(loc) && board.at_loc(loc).is_none() {
                        set.insert(loc);
                    } else if valid_location(loc) && board.at_loc(loc).unwrap().player != self.player {
                        set.insert(loc);
                        k = k+1;
                        i = row as i32;
                        j = col as i32;
                    } else {
                        k = k+1;
                        i = row as i32;
                        j = col as i32;
                    }
                }
                set
            },
            PieceType::Knight => {
                for k in 0 .. 8 {
                    let mut r: i32 = row as i32;
                    let mut c: i32 = col as i32;

                    // loop through all 8 positions
                    if k < 4 {
                        r = r + 2 * (-1 as i32).pow(k);
                        c = c + 1 * (-1 as i32).pow(k/2);
                    } else if k < 8 {
                        r = r + 1 * (-1 as i32).pow(k);
                        c = c + 2 * (-1 as i32).pow(k/2);
                    }

                    if r < 0 || c < 0 {
                        continue;
                    }
                    let loc = Location::new(r.try_into().unwrap(),c.try_into().unwrap());
                    if valid_location(loc) && (board.at_loc(loc).is_none() 
                    || board.at_loc(loc).unwrap().player != self.player) {
                        set.insert(loc);
                    }
                }
                set
            },
            PieceType::Bishop => {
                let mut i: i32 = row.try_into().unwrap();
                let mut j: i32 = col.try_into().unwrap();
                let mut k = 0;
                while k < 4 {
                    i = i + (-1 as i32).pow(k);
                    j = j + (-1 as i32).pow(k/2);
                    if i < 0 || j < 0 {
                        k = k + 1;
                        i = row as i32;
                        j = col as i32;
                        continue;
                    } 
                    let loc = Location::new(i.try_into().unwrap(), j.try_into().unwrap());
                    if valid_location(loc) && board.at_loc(loc).is_none() {
                        set.insert(loc);
                    } else if valid_location(loc) && board.at_loc(loc).unwrap().player != self.player {
                        set.insert(loc);
                        k = k + 1;
                        i = row as i32;
                        j = col as i32;
                    } else {
                        k = k + 1;
                        i = row as i32;
                        j = col as i32;
                    }
                }
                set
            },
            PieceType::Queen => {
                let mut r: i32 = row as i32;
                let mut c: i32 = col as i32;
                let mut k = 0;
                while k < 9 {
                    let i: i32 = (k / 3) - 1;
                    let j: i32 = (k % 3) - 1;
                    r = r + i;
                    c = c + j;
                    if k == 4 || r < 0 || c < 0 {
                        r = row as i32;
                        c = col as i32;
                        k = k + 1;
                        continue;
                    }
                    let loc = Location::new(r.try_into().unwrap(), c.try_into().unwrap());
                    if valid_location(loc) && board.at_loc(loc).is_none() {
                        set.insert(loc);
                    } else if valid_location(loc) && board.at_loc(loc).unwrap().player != self.player {
                        set.insert(loc);
                        r = row as i32;
                        c = col as i32;
                        k = k + 1;
                    } else {
                        r = row as i32;
                        c = col as i32;
                        k = k + 1;
                    }
                }
                set
            },
            PieceType::King => {
                for k in 0 .. 9 {
                    let r: i32 = (row as i32) + (k / 3) - 1;
                    let c: i32 = (col as i32) + (k % 3) - 1;
                    if k == 4 || r < 0 || c < 0 {
                        continue;
                    }
                    let loc = Location::new(r.try_into().unwrap(),c.try_into().unwrap());
                    if valid_location(loc) && (board.at_loc(loc).is_none() || board.at_loc(loc).unwrap().player != self.player) {
                        set.insert(loc);
                    }
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