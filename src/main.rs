use std::borrow::BorrowMut;

use bevy::prelude::*;
mod board_positions;
mod iboard_positions;
mod position;
mod create_sprite;
mod piece_type;
mod piece_id;
mod piece_color;
mod piece_registry;
mod piece;
mod sprite_bundles;
use board_positions::BoardPositions;
use piece::Piece;
use piece_id::PieceId;
use piece_registry::PieceRegistry;
use position::Position;
use sprite_bundles::SpriteBundles;

fn main() {
    let board_positions = BoardPositions::new(256.0, 128.0, 8);
    let sprite_bundles = SpriteBundles::new(board_positions.clone());
    let piece_registry = PieceRegistry::new(board_positions);

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(sprite_bundles)
        .insert_resource(piece_registry)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_board_background)
        .add_startup_system(setup_board_tiles)
        .add_startup_system(setup_board_pieces)
        .add_system(update_camera)
        .add_system(handle_input)
        .add_system(update_pieces)
        .run();
}

fn setup_board_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    sprite_bundles: Res<SpriteBundles>,
    mut piece_registry: ResMut<PieceRegistry>
) {
    for r in 0..8 {
        for c in 0..8 {
            let starting_pos = Position::new(r, c);
            if let Some(piece) = Piece::from(starting_pos) {
                let sprite = sprite_bundles.get_piece(&piece, &asset_server, &mut texture_atlases);
                commands.spawn(sprite);
                piece_registry.insert(piece);
            }
        }
    }
}

fn setup_board_tiles(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>, sprite_bundles: Res<SpriteBundles>) {
    for r in 0..8 {
        for c in 0..8 {
            let pos = Position::new(r, c);
            let bundle = sprite_bundles.get_tile(pos, &asset_server, &mut texture_atlases);
            commands.spawn(bundle);
        }
    }
}

fn setup_board_background(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>, sprite_bundles: Res<SpriteBundles>) {
    let sprite = sprite_bundles.get_background(&asset_server, &mut texture_atlases);
    commands.spawn(sprite);
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        ..default()
    });
}

fn update_camera(time: Res<Time>, mut camera_query: Query<&mut Transform, With<Camera2d>>, keyboard_input: Res<Input<KeyCode>>) {
    for mut camera in &mut camera_query {
        if keyboard_input.pressed(KeyCode::W) {
            camera.scale -=  time.delta_seconds() * 10.0;
        }
        if keyboard_input.pressed(KeyCode::S) {
            camera.scale +=  time.delta_seconds() * 10.0;
        }
    }
}

fn handle_input(time: Res<Time>, keyboard_input: Res<Input<KeyCode>>, mouse_input: Res<Input<MouseButton>>, mut piece_registry: ResMut<PieceRegistry>) {
    if keyboard_input.pressed(KeyCode::Up) {
        piece_registry.transfer(Position::new(0,0), Position::new(4,4));
    }

    if mouse_input.just_pressed(MouseButton::Left) {
        // get world pos of cursor
        // get closest row col of cursor?
        // check if source and dest have (row, col)
        // swap those two spots on the board
    }
}

fn update_pieces(mut piece_query: Query<(&mut Transform, &mut PieceId)>, keyboard_input: Res<Input<KeyCode>>, piece_registry: Res<PieceRegistry>) {
    for (mut transform, id) in &mut piece_query {
        if let Some(pos) = piece_registry.get_xy(id.0) {
            transform.translation.x = pos.x;
            transform.translation.y = pos.y;
        }
    }
}