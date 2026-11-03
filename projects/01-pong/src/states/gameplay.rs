// --- Imports ---

use macroquad::prelude::*;

use crate::audio::manager::AudioManager;
use crate::game::ball::Ball;
use crate::game::difficulty::CpuDifficultyConfig;
use crate::game::paddle::Paddle;
use crate::game::round::RoundState;
use crate::game::score::Score;

// --- Gameplay Constants ---

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

const COUNTDOWN_DURATION: f32 = 3.0;

const CPU_SIDE_THRESHOLD: f32 = 300.0;
const CPU_TARGET_OFFSET: f32 = 4.0;

const COUNTDOWN_FONT_SIZE: f32 = 32.0;
const COUNTDOWN_AREA_X: f32 = 400.0 - 16.0;
const COUNTDOWN_AREA_Y: f32 = 80.0;
const COUNTDOWN_AREA_WIDTH: f32 = 32.0;
const COUNTDOWN_AREA_HEIGHT: f32 = 48.0;

const SCORE_AREA_WIDTH: f32 = 32.0;
const SCORE_AREA_HEIGHT: f32 = 48.0;
const SCORE_Y: f32 = 80.0;
const SCORE_FONT_SIZE: u16 = 32;

const PLAYER_SCORE_X: f32 = 230.0;
const CPU_SCORE_X: f32 = 538.0;

// --- Gameplay ---

pub struct Gameplay {
    background: Texture2D,
    player_paddle: Paddle,
    cpu_paddle: Paddle,
    ball: Ball,
    score: Score,
    round_state: RoundState,
    countdown_timer: f32,
    serve_to_right: bool,
    cpu_config: CpuDifficultyConfig,
    cpu_reaction_timer: f32,
    cpu_target_y: f32,
    font: Font,
}

impl Gameplay {
    // --- Construction ---

    pub fn new(
        background: Texture2D,
        player_texture: Texture2D,
        cpu_texture: Texture2D,
        ball_texture: Texture2D,
        cpu_config: CpuDifficultyConfig,
        font: Font,
    ) -> Self {
        let player_position = Paddle::player_start_position();
        let cpu_position = Paddle::cpu_start_position();

        Self {
            background,
            player_paddle: Paddle::new(player_texture, player_position.x, player_position.y),
            cpu_paddle: Paddle::new(cpu_texture, cpu_position.x, cpu_position.y),
            ball: Ball::new(ball_texture),
            score: Score::new(),
            round_state: RoundState::Playing,
            countdown_timer: 0.0,
            serve_to_right: true,
            cpu_config,
            cpu_reaction_timer: 0.0,
            cpu_target_y: Paddle::cpu_start_position().y + 24.0,
            font,
        }
    }

    // --- Public API ---

    pub fn update(&mut self, audio_manager: &AudioManager) {
        match self.round_state {
            RoundState::Playing => self.update_playing(audio_manager),
            RoundState::Countdown => self.update_countdown(),
            RoundState::MatchOver => {}
        }
    }

    pub fn draw(&self) {
        self.draw_background();
        self.draw_game_objects();
        self.draw_score();

        if self.round_state == RoundState::Countdown {
            self.draw_countdown();
        }
    }

    pub fn is_match_over(&self) -> bool {
        self.round_state == RoundState::MatchOver
    }

    pub fn player_score(&self) -> u8 {
        self.score.player_score()
    }

    pub fn cpu_score(&self) -> u8 {
        self.score.cpu_score()
    }

    // --- Gameplay Updates ---

    fn update_playing(&mut self, audio_manager: &AudioManager) {
        self.player_paddle.update_player();

        self.update_cpu();
        self.update_ball(audio_manager);
    }

    fn update_cpu(&mut self) {
        let ball_position = self.ball.position();
        let ball_velocity = self.ball.velocity();

        let ball_is_on_cpu_side = ball_position.x > CPU_SIDE_THRESHOLD;
        let ball_is_approaching = ball_velocity.x > 0.0;

        if ball_is_on_cpu_side && ball_is_approaching {
            self.cpu_reaction_timer -= get_frame_time();

            if self.cpu_reaction_timer <= 0.0 {
                let error = macroquad::rand::gen_range(
                    -self.cpu_config.error_margin,
                    self.cpu_config.error_margin,
                );

                self.cpu_target_y = ball_position.y + CPU_TARGET_OFFSET + error;
                self.cpu_reaction_timer = self.cpu_config.reaction_time;
            }

            self.cpu_paddle
                .update_cpu(self.cpu_target_y, true, self.cpu_config.speed);
        } else {
            self.cpu_reaction_timer = 0.0;
        }
    }

    fn update_ball(&mut self, audio_manager: &AudioManager) {
        let (wall_collision, paddle_collision) =
            self.ball.update(&self.player_paddle, &self.cpu_paddle);

        if wall_collision {
            audio_manager.play_wall();
        }

        if paddle_collision {
            audio_manager.play_paddle();
        }

        if let Some(player_scored) = self.ball.check_score() {
            self.handle_score(player_scored, audio_manager);
        }
    }

    fn handle_score(&mut self, player_scored: bool, audio_manager: &AudioManager) {
        audio_manager.play_brick();

        if player_scored {
            self.score.add_player_point();
        } else {
            self.score.add_cpu_point();
        }

        if self.score.is_match_over() {
            if player_scored {
                audio_manager.play_victory();
            } else {
                audio_manager.play_game_over();
            }

            self.round_state = RoundState::MatchOver;
            return;
        }

        self.prepare_next_round();
    }

    fn update_countdown(&mut self) {
        self.countdown_timer -= get_frame_time();

        if self.countdown_timer <= 0.0 {
            self.countdown_timer = 0.0;
            self.ball.launch(self.serve_to_right);
            self.round_state = RoundState::Playing;
        }
    }

    // --- Round Management ---

    fn prepare_next_round(&mut self) {
        self.ball.reset();

        let player_position = Paddle::player_start_position();
        let cpu_position = Paddle::cpu_start_position();

        self.player_paddle
            .reset_position(player_position.x, player_position.y);

        self.cpu_paddle
            .reset_position(cpu_position.x, cpu_position.y);

        self.serve_to_right = !self.serve_to_right;
        self.countdown_timer = COUNTDOWN_DURATION;
        self.round_state = RoundState::Countdown;
    }

    // --- Rendering ---

    fn draw_background(&self) {
        draw_texture_ex(
            &self.background,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(SCREEN_WIDTH, SCREEN_HEIGHT)),
                ..Default::default()
            },
        );
    }

    fn draw_game_objects(&self) {
        self.player_paddle.draw();
        self.cpu_paddle.draw();
        self.ball.draw();
    }

    fn draw_score(&self) {
        let player_text = self.score.player_score().to_string();
        let cpu_text = self.score.cpu_score().to_string();

        self.draw_score_digit(
            &player_text,
            PLAYER_SCORE_X,
            SCORE_Y,
            SCORE_AREA_WIDTH,
            SCORE_AREA_HEIGHT,
            SCORE_FONT_SIZE,
        );

        self.draw_score_digit(
            &cpu_text,
            CPU_SCORE_X,
            SCORE_Y,
            SCORE_AREA_WIDTH,
            SCORE_AREA_HEIGHT,
            SCORE_FONT_SIZE,
        );
    }

    fn draw_score_digit(
        &self,
        text: &str,
        area_x: f32,
        area_y: f32,
        area_width: f32,
        area_height: f32,
        font_size: u16,
    ) {
        let dimensions = measure_text(text, Some(&self.font), font_size, 1.0);

        let text_x = area_x + (area_width - dimensions.width) / 2.0;
        let text_y = area_y + (area_height + dimensions.height) / 2.0;

        draw_text_ex(
            text,
            text_x,
            text_y,
            TextParams {
                font: Some(&self.font),
                font_size,
                color: WHITE,
                ..Default::default()
            },
        );
    }

    fn draw_countdown(&self) {
        let number = self.countdown_timer.ceil() as u8;
        let text = number.to_string();

        let dimensions = measure_text(&text, None, COUNTDOWN_FONT_SIZE as u16, 1.0);

        let text_x = COUNTDOWN_AREA_X + (COUNTDOWN_AREA_WIDTH - dimensions.width) / 2.0;

        let text_y = COUNTDOWN_AREA_Y + (COUNTDOWN_AREA_HEIGHT + dimensions.height) / 2.0;

        draw_text(&text, text_x, text_y, COUNTDOWN_FONT_SIZE, WHITE);
    }
}
