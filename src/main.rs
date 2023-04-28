use bevy::prelude::*;
mod board_positions;
mod position;
mod piece_type;
mod piece_id;
mod piece_color;
mod piece_registry;
mod piece;
mod sprite_bundles;
mod mouse_handler;
use board_positions::BoardPositions;
use mouse_handler::MouseHandler;
use piece::Piece;
use piece_id::PieceId;
use piece_registry::PieceRegistry;
use position::Position;
use sprite_bundles::SpriteBundles;

fn main() {
    let board_positions = BoardPositions::new(256.0, 128.0, 8);
    let sprite_bundles = SpriteBundles::new(board_positions.clone());
    let piece_registry = PieceRegistry::new(board_positions);
    let mouse_handler = MouseHandler::new();

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(sprite_bundles)
        .insert_resource(piece_registry)
        .insert_resource(mouse_handler)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_board_background)
        .add_startup_system(setup_board_tiles)
        .add_startup_system(setup_board_pieces)
        .add_system(update_camera)
        .add_system(update_pieces)
        .add_system(handle_input)
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

fn setup_board_background(mut commands: Commands, asset_server: Res<AssetServer>, sprite_bundles: Res<SpriteBundles>) {
    let sprite = sprite_bundles.get_background(&asset_server);
    commands.spawn(sprite);
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        ..default()
    });
}
// fn update_camera(time: Res<Time>, mut camera_query: Query<&mut Transform, With<Camera2d>>, keyboard_input: Res<Input<KeyCode>>, mouse_scroll: EventReader<MouseWheel>) {
//     let mut camera = camera_query.single();
//     for ev in mouse_scroll.iter() {
//         match ev.unit {
//             camera.scale -=  time.delta_seconds() * 10.0;
//             MouseScrollUnit::Pixel => {
//                 println!("Scroll (pixel units): vertical: {}, horizontal: {}", ev.y, ev.x);
//             }
//         }
//     }
// }

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

fn handle_input(
    // time: Res<Time>,
    mouse_input: Res<Input<MouseButton>>, 
    mut piece_registry: ResMut<PieceRegistry>, 
    mut mouse_handler: ResMut<MouseHandler>,
    windows_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
)
{
    if mouse_input.just_pressed(MouseButton::Left) {
        let window = windows_query.single();
        let (camera, camera_transform) = camera_query.single();
        let movement = mouse_handler.on_left_click(camera, camera_transform, window);
        match movement {
            Some((from, to)) => piece_registry.transfer(from, to),
            None => return,
        }
    }
}

fn update_pieces(mut piece_query: Query<(&mut Transform, &mut PieceId)>, piece_registry: Res<PieceRegistry>) {
    for (mut transform, id) in &mut piece_query {
        if let Some(pos) = piece_registry.get_xy(id.0) {
            transform.translation.x = pos.x;
            transform.translation.y = pos.y;
        }
    }
}