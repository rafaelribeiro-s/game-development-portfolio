// --- Imports ---

use macroquad::prelude::*;

use crate::config::{
    COLLISION_HEIGHT, COLLISION_MARGIN_TOP, GAMEPLAY_WIDTH, GAMEPLAY_X, GAMEPLAY_Y,
};

use crate::game::paddle::Paddle;

// --- Constants ---

const BALL_SIZE: f32 = 8.0;
const BALL_INITIAL_SPEED: f32 = 200.0;
const BALL_MAX_SPEED: f32 = 440.0;

const SPEED_TIER_1: f32 = 220.0;
const SPEED_TIER_2: f32 = 280.0;
const SPEED_TIER_3: f32 = 350.0;
const SPEED_TIER_4: f32 = 420.0;

const HITS_TIER_2: u32 = 3;
const HITS_TIER_3: u32 = 6;
const HITS_TIER_4: u32 = 9;

const SCORE_MARGIN: f32 = 32.0;

// --- Collision Constants ---

const CENTER_ZONE: f32 = 0.20;
const INTERMEDIATE_ZONE: f32 = 0.65;

const CENTER_ANGLE: f32 = 0.0;
const INTERMEDIATE_ANGLE: f32 = 30.0;
const EXTREME_ANGLE: f32 = 55.0;

// --- Types ---

pub struct Ball {
    texture: Texture2D,
    position: Vec2,
    velocity: Vec2,
    hit_count: u32,
}

// --- Construction ---

impl Ball {
    pub fn new(texture: Texture2D) -> Self {
        Self {
            texture,
            position: Self::initial_position(),
            velocity: vec2(-BALL_INITIAL_SPEED, 0.0),
            hit_count: 0,
        }
    }

    // --- Public API ---

    pub fn update(&mut self, player_paddle: &Paddle, cpu_paddle: &Paddle) -> (bool, bool) {
        const MAX_STEP_DISTANCE: f32 = 2.0;

        let delta_time = get_frame_time();
        let movement = self.velocity * delta_time;

        let max_distance = movement.x.abs().max(movement.y.abs());

        let steps = (max_distance / MAX_STEP_DISTANCE).ceil().max(1.0) as u32;

        let step_delta = movement / steps as f32;

        let mut wall_collision = false;
        let mut paddle_collision = false;

        for _ in 0..steps {
            self.position += step_delta;

            if self.handle_wall_collision() {
                wall_collision = true;
            }

            let player_collision = self.handle_paddle_collision(player_paddle, true);
            let cpu_collision = self.handle_paddle_collision(cpu_paddle, false);

            if player_collision || cpu_collision {
                paddle_collision = true;
            }
        }

        (wall_collision, paddle_collision)
    }

    pub fn draw(&self) {
        draw_texture_ex(
            &self.texture,
            self.position.x,
            self.position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(BALL_SIZE, BALL_SIZE)),
                ..Default::default()
            },
        );
    }

    pub fn reset(&mut self) {
        self.position = Self::initial_position();
        self.velocity = Vec2::ZERO;
        self.hit_count = 0;
    }

    pub fn launch(&mut self, to_right: bool) {
        let direction = if to_right { 1.0 } else { -1.0 };

        self.velocity = vec2(direction * BALL_INITIAL_SPEED, 0.0);
    }

    // --- Scoring Detection ---

    /// Checks whether the ball has left the gameplay area horizontally.
    ///
    /// Returns `Some(true)` when the player scores,
    /// `Some(false)` when the CPU scores, or `None` otherwise.
    pub fn check_score(&self) -> Option<bool> {
        let ball_center_x = self.position.x + BALL_SIZE / 2.0;

        let left_score_limit = GAMEPLAY_X - SCORE_MARGIN;
        let right_score_limit = GAMEPLAY_X + GAMEPLAY_WIDTH + SCORE_MARGIN;

        if ball_center_x < left_score_limit {
            Some(false)
        } else if ball_center_x > right_score_limit {
            Some(true)
        } else {
            None
        }
    }

    // --- Getters ---

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn velocity(&self) -> Vec2 {
        self.velocity
    }

    // --- Collision Handling ---

    fn handle_wall_collision(&mut self) -> bool {
        let min_y = GAMEPLAY_Y + COLLISION_MARGIN_TOP;
        let max_y = min_y + COLLISION_HEIGHT - BALL_SIZE;

        if self.position.y <= min_y {
            self.position.y = min_y;
            self.velocity.y = self.velocity.y.abs();

            return true;
        }

        if self.position.y >= max_y {
            self.position.y = max_y;
            self.velocity.y = -self.velocity.y.abs();

            return true;
        }

        false
    }

    pub fn handle_paddle_collision(&mut self, paddle: &Paddle, is_left_paddle: bool) -> bool {
        let ball_rect = Rect::new(self.position.x, self.position.y, BALL_SIZE, BALL_SIZE);

        let paddle_rect = paddle.collision_rect();

        // Ignore collisions when the ball is moving away from the paddle.
        if is_left_paddle && self.velocity.x >= 0.0 {
            return false;
        }

        if !is_left_paddle && self.velocity.x <= 0.0 {
            return false;
        }

        if !ball_rect.overlaps(&paddle_rect) {
            return false;
        }

        // Calculate the contact point relative to the paddle center.
        let ball_center_y = self.position.y + BALL_SIZE / 2.0;
        let paddle_center_y = paddle_rect.y + paddle_rect.h / 2.0;

        let contact_offset = (ball_center_y - paddle_center_y) / (paddle_rect.h / 2.0);

        let contact_offset = contact_offset.clamp(-1.0, 1.0);

        // Determine the rebound angle from the contact region.
        let absolute_offset = contact_offset.abs();

        let angle = if absolute_offset <= CENTER_ZONE {
            CENTER_ANGLE
        } else if absolute_offset <= INTERMEDIATE_ZONE {
            INTERMEDIATE_ANGLE
        } else {
            EXTREME_ANGLE
        };

        let angle_radians = angle.to_radians();
        let vertical_direction = contact_offset.signum();
        let horizontal_direction = if is_left_paddle { 1.0 } else { -1.0 };

        let speed = self.velocity.length();

        self.velocity = vec2(
            horizontal_direction * speed * angle_radians.cos(),
            vertical_direction * speed * angle_radians.sin(),
        );

        self.register_hit();

        // Move the ball outside the paddle after the collision.
        if is_left_paddle {
            self.position.x = paddle_rect.x + paddle_rect.w;
        } else {
            self.position.x = paddle_rect.x - BALL_SIZE;
        }

        true
    }

    // --- Private Helpers ---

    fn initial_position() -> Vec2 {
        let x = GAMEPLAY_X + (GAMEPLAY_WIDTH - BALL_SIZE) / 2.0;

        let y = GAMEPLAY_Y + COLLISION_MARGIN_TOP + (COLLISION_HEIGHT - BALL_SIZE) / 2.0;

        vec2(x, y)
    }

    fn register_hit(&mut self) {
        self.hit_count += 1;

        let target_speed = if self.hit_count < HITS_TIER_2 {
            SPEED_TIER_1
        } else if self.hit_count < HITS_TIER_3 {
            SPEED_TIER_2
        } else if self.hit_count < HITS_TIER_4 {
            SPEED_TIER_3
        } else {
            SPEED_TIER_4
        };

        let current_speed = self.velocity.length();
        let new_speed = target_speed.min(BALL_MAX_SPEED);

        if current_speed > 0.0 {
            self.velocity = self.velocity.normalize() * new_speed;
        }
    }
}
