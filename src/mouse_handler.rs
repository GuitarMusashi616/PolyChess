use bevy::prelude::*;

use crate::position::Position;

const TILE_SIZE: f32 = 256.0;
const ROW_COLS: f32 = 8.0;

#[derive(Resource)]
pub struct MouseHandler {
    click_initial: Option<Position>,
}

impl MouseHandler {

    pub fn new() -> Self {
        Self {
            click_initial: None,
        }
    }

    pub fn on_left_click(&mut self, camera: &Camera, camera_transform: &GlobalTransform, windows: &Window) -> Option<(Position, Position)> {
        if let Some(pos) = self.get_world_position(camera, camera_transform, windows) {
            let tile_position = self.get_tile_position(pos);
            if let Some(initial_position) = self.click_initial {
                self.click_initial = None;
                return Some((initial_position, tile_position));
            }
            self.click_initial = Some(tile_position);
        }
        None
    }

    pub fn get_tile_position(&self, pos: Vec2) -> Position {
        let offset = TILE_SIZE * (ROW_COLS/2.0);
        let row = ((pos.y + offset) / TILE_SIZE).floor();
        let col = ((pos.x + offset) / TILE_SIZE).floor();
        Position::new(row as usize, col as usize)
    }

    pub fn get_world_position(&self, camera: &Camera, camera_transform: &GlobalTransform, window: &Window) -> Option<Vec2> {
        if let Some(world_position) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            return Some(world_position);
        }
        None
    }
}