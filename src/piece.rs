use crate::{position::Position, piece_type::PieceType::{self, *}, piece_color::PieceColor::{self, *}};

pub struct Piece {
    pub id: usize,
    pub pos: Position,
    pub piece_type: PieceType,
    pub piece_color: PieceColor,
}


impl Piece {
    pub fn new(id: usize, pos: Position, piece_type: PieceType, piece_color: PieceColor) -> Self {
        Piece {
            id,
            pos,
            piece_type,
            piece_color,
        }
    }

    pub fn from(starting_pos: Position) -> Option<Self> {
        let id = Self::starting_pos_to_piece_id(starting_pos);
        let (piece_type, piece_color) = Self::piece_id_to_piece_type_color(id)?;
        
        Some(Piece::new(id, starting_pos, piece_type, piece_color))
    }

    fn starting_pos_to_piece_id(pos: Position) -> usize {
        pos.row*8 + pos.col
    }

    fn piece_id_to_piece_type_color(piece_id: usize) -> Option<(PieceType, PieceColor)>  {
        match piece_id {
            0 => Some((Rook, Black)),
            1 => Some((Knight, Black)),
            2 => Some((Bishop, Black)),
            3 => Some((King, Black)),
            4 => Some((Queen, Black)),
            5 => Some((Bishop, Black)),
            6 => Some((Knight, Black)),
            7 => Some((Rook, Black)),
            i if i >= 8 && i < 16 => Some((Pawn, Black)),
            i if i >= 48 && i < 56 => Some((Pawn, White)),
            56 => Some((Rook, White)),
            57 => Some((Knight, White)),
            58 => Some((Bishop, White)),
            59 => Some((King, White)),
            60 => Some((Queen, White)),
            61 => Some((Bishop, White)),
            62 => Some((Knight, White)),
            63 => Some((Rook, White)),
            _ => None,
        }
    }
}