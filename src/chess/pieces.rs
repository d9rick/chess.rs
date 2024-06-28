use crate::chess::gamestate::Color;

/* Pieces */
#[derive(Copy, Clone)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Copy, Clone)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
}
impl Piece {
    pub fn new(color: Color, piece_type: PieceType) -> Piece {
        Piece { color, piece_type, }
    }

    // prints the associated standard chess piece character
    pub fn print(&self) {
        // get the piece string
        let piece_str = match self.piece_type {
            PieceType::Pawn => "P",
            PieceType::Rook => "R",
            PieceType::Knight => "N",
            PieceType::Bishop => "B",
            PieceType::Queen => "Q",
            PieceType::King => "K",
        };

        // if the piece is black make it lowercase
        let piece_str = if self.color == Color::Black {
            piece_str.to_lowercase()
        } else {
            piece_str.to_string()
        };

        print!("{}", piece_str);
    }
}