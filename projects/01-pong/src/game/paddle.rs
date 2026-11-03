// --- Imports ---

use macroquad::prelude::*;

use crate::config::{
    COLLISION_HEIGHT, COLLISION_MARGIN_TOP, GAMEPLAY_WIDTH, GAMEPLAY_X, GAMEPLAY_Y,
};

// --- Constants ---

const PADDLE_WIDTH: f32 = 8.0;
const PADDLE_HEIGHT: f32 = 48.0;
const PADDLE_SPEED: f32 = 360.0;
const PADDLE_HORIZONTAL_MARGIN: f32 = 8.0;

// --- Paddle ---

pub struct Paddle {
    texture: Texture2D,
    position: Vec2,
}

// --- Construction and Public API ---

impl Paddle {
    pub fn new(texture: Texture2D, x: f32, y: f32) -> Self {
        Self {
            texture,
            position: vec2(x, y),
        }
    }

    // --- Player Controls ---

    pub fn update_player(&mut self) {
        let mut direction = 0.0;

        if is_key_down(KeyCode::Up) {
            direction -= 1.0;
        }

        if is_key_down(KeyCode::Down) {
            direction += 1.0;
        }

        self.position.y += direction * PADDLE_SPEED * get_frame_time();

        self.clamp_position();
    }

    // --- CPU Controls ---

    pub fn update_cpu(&mut self, target_y: f32, ball_is_approaching: bool, speed: f32) {
        if !ball_is_approaching {
            return;
        }

        let delta_time = get_frame_time();
        let paddle_center = self.position.y + PADDLE_HEIGHT / 2.0;
        let difference = target_y - paddle_center;
        let max_movement = speed * delta_time;

        self.position.y += difference.clamp(-max_movement, max_movement);

        self.clamp_position();
    }

    // --- Rendering ---

    pub fn draw(&self) {
        draw_texture_ex(
            &self.texture,
            self.position.x,
            self.position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(PADDLE_WIDTH, PADDLE_HEIGHT)),
                ..Default::default()
            },
        );
    }

    // --- Collision and Position ---

    pub fn collision_rect(&self) -> Rect {
        Rect::new(
            self.position.x,
            self.position.y,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
        )
    }

    pub fn reset_position(&mut self, x: f32, y: f32) {
        self.position = vec2(x, y);
    }

    // --- Starting Positions ---

    pub fn player_start_position() -> Vec2 {
        let x = GAMEPLAY_X + PADDLE_HORIZONTAL_MARGIN;
        let y = GAMEPLAY_Y + COLLISION_MARGIN_TOP + (COLLISION_HEIGHT - PADDLE_HEIGHT) / 2.0;

        vec2(x, y)
    }

    pub fn cpu_start_position() -> Vec2 {
        let x = GAMEPLAY_X + GAMEPLAY_WIDTH - PADDLE_HORIZONTAL_MARGIN - PADDLE_WIDTH;
        let y = GAMEPLAY_Y + COLLISION_MARGIN_TOP + (COLLISION_HEIGHT - PADDLE_HEIGHT) / 2.0;

        vec2(x, y)
    }

    // --- Private Helpers ---

    fn clamp_position(&mut self) {
        let min_y = GAMEPLAY_Y + COLLISION_MARGIN_TOP;
        let max_y = min_y + COLLISION_HEIGHT - PADDLE_HEIGHT;

        self.position.y = self.position.y.clamp(min_y, max_y);
    }
}
