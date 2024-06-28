use crate::chess::pieces::*;
use crate::chess::gamestate::Color;
use crate::chess::gamestate::Move;

pub struct Board {
    pieces: [[Option<Piece>; 8]; 8], // 8x8 grid of pieces
}

impl Board {
    // create a new board with the pieces in their starting positions
    pub fn new() -> Board {
        Board {
            pieces: [
                [
                    Some(Piece::new(Color::White, PieceType::Rook)),
                    Some(Piece::new(Color::White, PieceType::Knight)),
                    Some(Piece::new(Color::White, PieceType::Bishop)),
                    Some(Piece::new(Color::White, PieceType::King)),
                    Some(Piece::new(Color::White, PieceType::Queen)),
                    Some(Piece::new(Color::White, PieceType::Bishop)),
                    Some(Piece::new(Color::White, PieceType::Knight)),
                    Some(Piece::new(Color::White, PieceType::Rook)),
                ],
                [
                    Some(Piece::new(Color::White, PieceType::Pawn)),
                    Some(Piece::new(Color::White, PieceType::Pawn)),
                    Some(Piece::new(Color::White, PieceType::Pawn)),
                    Some(Piece::new(Color::White, PieceType::Pawn)),
                    Some(Piece::new(Color::White, PieceType::Pawn)),
                    Some(Piece::new(Color::White, PieceType::Pawn)),
                    Some(Piece::new(Color::White, PieceType::Pawn)),
                    Some(Piece::new(Color::White, PieceType::Pawn)),                    
                ],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [
                    Some(Piece::new(Color::Black, PieceType::Pawn)),
                    Some(Piece::new(Color::Black, PieceType::Pawn)),
                    Some(Piece::new(Color::Black, PieceType::Pawn)),
                    Some(Piece::new(Color::Black, PieceType::Pawn)),
                    Some(Piece::new(Color::Black, PieceType::Pawn)),
                    Some(Piece::new(Color::Black, PieceType::Pawn)),
                    Some(Piece::new(Color::Black, PieceType::Pawn)),
                    Some(Piece::new(Color::Black, PieceType::Pawn)),                    
                ],
                [
                    Some(Piece::new(Color::Black, PieceType::Rook)),
                    Some(Piece::new(Color::Black, PieceType::Knight)),
                    Some(Piece::new(Color::Black, PieceType::Bishop)),
                    Some(Piece::new(Color::Black, PieceType::King)),
                    Some(Piece::new(Color::Black, PieceType::Queen)),
                    Some(Piece::new(Color::Black, PieceType::Bishop)),
                    Some(Piece::new(Color::Black, PieceType::Knight)),
                    Some(Piece::new(Color::Black, PieceType::Rook)),
                ]
            ]
        }
    }

    pub fn print(&self, player: Color) {
        // reverse the board if the player is white
        if player == Color::Black {
            for row in self.pieces.iter() {
                for piece in row.iter() {
                    match piece {
                        Some(piece) => piece.print(),
                        None => print!("-"),
                    }
                }
                println!();
            }
        } else {
            for row in self.pieces.iter().rev() {
                for piece in row.iter().rev() {
                    match piece {
                        Some(piece) => piece.print(),
                        None => print!("-"),
                    }
                }
                println!();
            }
        }
    }

    pub fn move_piece(&mut self, curr_move: Move) {
        // get the start and end positions
        let (start_x, start_y) = curr_move.start;
        let (end_x, end_y) = curr_move.end;
        
        // get the piece at the start position
        let piece = self.pieces[start_x as usize][start_y as usize];
        
        // move the piece to the end position
        self.pieces[end_x as usize][end_y as usize] = piece;
        self.pieces[start_x as usize][start_y as usize] = None;
    }
}
