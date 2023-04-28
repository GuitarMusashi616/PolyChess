use bevy::prelude::Vec2;

use crate::{position::Position};

#[derive(Clone)]
pub struct BoardPositions {
    tile_length: f32,
    background_margin: f32,
    row_cols: usize,
}

impl BoardPositions {
    pub fn new(tile_length: f32, background_margin: f32, row_cols: usize) -> Self {
        BoardPositions {
            tile_length,
            background_margin,
            row_cols,
        }
    }

    fn from_index_to_vec2(&self, index: usize) -> f32 {
        let offset = (self.row_cols as f32/2.0)*self.tile_length;
        let half_tile = self.tile_length/2.0;
        let base_tile = self.tile_length * index as f32;
        return base_tile - offset + half_tile;
    }

    pub fn get_xy(&self, pos: Position) -> Vec2 {
        let y = self.from_index_to_vec2(pos.row);
        let x = self.from_index_to_vec2(pos.col);
        Vec2::new(x, y)
    }

    pub fn get_background_scale(&self, img_length: f32) -> f32 {
        let size_needed  = self.row_cols as f32 * self.tile_length + self.background_margin;
        size_needed / img_length
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_positions() {
        let bp = BoardPositions::new(256.0, 128.0, 8);
        for r in 0..8 {
            for c in 0..8 {
                let v2 = bp.get_xy(Position::new(r, c));
                println!("({}, {}) = {}", r, c, v2);
            }
        }
    }
}