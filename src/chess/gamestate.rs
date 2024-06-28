use crate::chess::board::Board;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Color {
    White,
    Black,
}
pub struct GameState {
    current_player: Color,
    board: Board,
    moves: Vec<Move>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            current_player: Color::White,
            board: Board::new(),
            moves: Vec::new(),
        }
    }

    pub fn print(&self) {
        println!("Current player: {:?}", self.current_player);
        self.board.print(self.current_player);
    }

    pub fn make_move(&mut self, start: (i32, i32), end: (i32, i32)) {
        // create the move object
        let curr_move = Move::new(start, end, None);

        // check if the move is valid
        if self.is_valid_move(curr_move) {
            // add the move to the list of moves
            self.moves.push(curr_move);

            // move the piece
            self.board.move_piece(curr_move);

            // switch the current player
            self.current_player = match self.current_player {
                Color::White => Color::Black,
                Color::Black => Color::White,
            };
        }
    }

    // returns true if a given move is valid
    fn is_valid_move(&self, curr_move: Move) -> bool {
        return true;
    }
}

/* Moves */
#[derive(Copy, Clone)]
pub enum SpecialMove {
    Castling,
    EnPassant,
    Promotion,
}

#[derive(Copy, Clone)]
pub struct Move {
    pub start: (i32, i32),
    pub end: (i32, i32),
    pub special: Option<SpecialMove>,
}

impl Move {
    pub fn new(start: (i32, i32), end: (i32, i32), special: Option<SpecialMove>) -> Move {
        Move { start, end, special }
    }
}