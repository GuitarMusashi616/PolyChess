use bevy::{utils::HashMap, prelude::{Resource, Vec2}};

use crate::{position::Position, piece::Piece, board_positions::BoardPositions, iboard_positions::IBoardPositions};

const ROWS: usize = 8;
const COLS: usize = 8;

#[derive(Resource)]
pub struct PieceRegistry {
    piece_map: HashMap<usize, Piece>,
    piece_positions: [[Option<usize>; COLS]; ROWS],
    board_positions: BoardPositions,
}

impl PieceRegistry {
    pub fn new(board_positions: BoardPositions) -> Self {
        Self {
            piece_map: HashMap::new(),
            piece_positions: [[None; COLS]; ROWS],
            board_positions,
        }
    }

    pub fn get_xy(&self, piece_id: usize) -> Option<Vec2> {
        let piece = self.piece_map.get(&piece_id)?;
        let res = self.board_positions.get_xy(piece.pos);
        Some(res)
    }

    pub fn insert(&mut self, piece: Piece) {
        self.piece_positions[piece.pos.row][piece.pos.col] = Some(piece.id);
        self.piece_map.insert(piece.id, piece);

    }

    pub fn transfer(&mut self, from: Position, to: Position) {
        let from_piece_id = match self.piece_positions[from.row][from.col] {
            Some(id) => id,
            None => return,
        };

        let from_piece = match self.piece_map.get_mut(&from_piece_id) {
            Some(piece) => piece,
            None => return,
        };

        from_piece.pos = to;
        self.piece_positions[from.row][from.col] = None;
        self.piece_positions[to.row][to.col] = Some(from_piece.id);
    }
}
