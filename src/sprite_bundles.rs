use bevy::prelude::*;

use crate::{piece::Piece, piece_id::PieceId, piece_type::PieceType, piece_color::PieceColor, board_positions::BoardPositions, position::Position};

#[derive(Resource)]
pub struct SpriteBundles {
    sprite_sheet_path: String,
    board_background_path: String,
    board_background_px_len: f32,
    board_positions: BoardPositions,

}

impl SpriteBundles {
    pub fn new(board_positions: BoardPositions) -> Self {
        // let board_background_path = "../assets/board_background.png".to_owned();
        // let sprite_sheet_path = "../assets/default_board.png".to_owned();
        // let project_path = "C:/Users/awilliams/Code/rust/polychess";
        let project_path = "..";

        let board_background_path = format!("{}/{}", project_path, "assets/board_background.png");
        let sprite_sheet_path = format!("{}/{}", project_path, "assets/default_board.png");

        let board_background_px_len = 1518.0;

        SpriteBundles {
            sprite_sheet_path,
            board_background_path,
            board_background_px_len,
            board_positions,
        }
    }

    pub fn get_background(&self, asset_server: &Res<AssetServer>, texture_atlases: &mut ResMut<Assets<TextureAtlas>>) -> SpriteBundle {
        let scale = self.board_positions.get_background_scale(self.board_background_px_len);
        SpriteBundle {
            texture: asset_server.load(&self.board_background_path),
            transform: Transform::from_scale(Vec3::new(scale, scale, 0.0)),
            ..default()
        }
    }

    pub fn get_piece(&self, piece: &Piece, asset_server: &Res<AssetServer>, texture_atlases: &mut ResMut<Assets<TextureAtlas>>) -> (PieceId, SpriteSheetBundle)  {
        let selection = (&piece.piece_type, &piece.piece_color);
        let sprite_index = Self::piece_type_color_to_sprite_index(selection);
        let mut sprite = self.create_board_sprite(sprite_index, asset_server, texture_atlases);
        let xy = self.board_positions.get_xy(piece.pos);
        sprite.transform = Transform::from_xyz(xy.x, xy.y, 4.0);
        (PieceId(piece.id), sprite)
    }

    pub fn get_tile(&self, pos: Position, asset_server: &Res<AssetServer>, texture_atlases: &mut ResMut<Assets<TextureAtlas>>) -> SpriteSheetBundle {
        let flip = (pos.row + pos.col) % 2;
        let mut ssb = self.create_board_sprite(flip, asset_server, texture_atlases);
        let xy = self.board_positions.get_xy(pos);
        ssb.transform = Transform::from_xyz(xy.x, xy.y, 3.0);
        ssb
    }

    pub fn create_board_sprite(&self, index: usize, asset_server: &Res<AssetServer>, texture_atlases: &mut ResMut<Assets<TextureAtlas>>) -> SpriteSheetBundle {
        let texture_handle = asset_server.load(&self.sprite_sheet_path);
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(256.0, 256.0), 14, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(index),
            ..default()
        }
    }

    fn piece_type_color_to_sprite_index(selection: (&PieceType, &PieceColor)) -> usize {
        match selection {
            (PieceType::Pawn, PieceColor::Black) => 2,
            (PieceType::Knight, PieceColor::Black) => 3,
            (PieceType::Bishop, PieceColor::Black) => 4,
            (PieceType::Rook, PieceColor::Black) => 5,
            (PieceType::King, PieceColor::Black) => 6,
            (PieceType::Queen, PieceColor::Black) => 7,
            (PieceType::Pawn, PieceColor::White) => 8,
            (PieceType::Knight, PieceColor::White) => 9,
            (PieceType::Bishop, PieceColor::White) => 10,
            (PieceType::Rook, PieceColor::White) => 11,
            (PieceType::King, PieceColor::White) => 12,
            (PieceType::Queen, PieceColor::White) => 13,
        }
    }
}