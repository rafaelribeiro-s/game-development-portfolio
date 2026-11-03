// --- Imports ---

use macroquad::prelude::*;

// --- Layout Constants ---

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

const MODAL_WIDTH: f32 = 300.0;
const MODAL_HEIGHT: f32 = 150.0;

const SCREEN_CENTER_X: f32 = SCREEN_WIDTH / 2.0;

const TITLE_FONT_SIZE: u16 = 16;
const SCORE_FONT_SIZE: u16 = 12;
const OPTION_FONT_SIZE: u16 = 10;

const TITLE_Y_OFFSET: f32 = 35.0;
const SCORE_Y_OFFSET: f32 = 65.0;
const OPTIONS_Y_OFFSET: f32 = 100.0;
const OPTION_SPACING: f32 = 22.0;

const TOTAL_OPTIONS: usize = 2;

const OPTIONS: [&str; TOTAL_OPTIONS] = ["PLAY AGAIN", "MAIN MENU"];

// --- Game Over Menu Actions ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameOverMenuAction {
    PlayAgain,
    MainMenu,
}

// --- Game Over Menu ---

pub struct GameOverMenu {
    modalbox_texture: Texture2D,
    font: Font,
    selected_option: usize,
}

impl GameOverMenu {
    // --- Construction ---

    pub fn new(modalbox_texture: Texture2D, font: Font) -> Self {
        Self {
            modalbox_texture,
            font,
            selected_option: 0,
        }
    }

    // --- Public API ---

    pub fn update(&mut self) {
        if is_key_pressed(KeyCode::Down) {
            self.selected_option = (self.selected_option + 1) % TOTAL_OPTIONS;
        }

        if is_key_pressed(KeyCode::Up) {
            self.selected_option = (self.selected_option + 1) % TOTAL_OPTIONS;
        }
    }

    pub fn confirm(&self) -> Option<GameOverMenuAction> {
        if !is_key_pressed(KeyCode::Enter) {
            return None;
        }

        match self.selected_option {
            0 => Some(GameOverMenuAction::PlayAgain),
            1 => Some(GameOverMenuAction::MainMenu),
            _ => None,
        }
    }

    pub fn draw(&self, player_score: u8, cpu_score: u8) {
        let modal_x = (SCREEN_WIDTH - MODAL_WIDTH) / 2.0;
        let modal_y = (SCREEN_HEIGHT - MODAL_HEIGHT) / 2.0;

        self.draw_modal(modal_x, modal_y);
        self.draw_result_title(player_score, cpu_score, modal_y);
        self.draw_score(player_score, cpu_score, modal_y);
        self.draw_options(modal_y);
    }

    // --- Rendering ---

    fn draw_modal(&self, x: f32, y: f32) {
        draw_texture_ex(
            &self.modalbox_texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(MODAL_WIDTH, MODAL_HEIGHT)),
                ..Default::default()
            },
        );
    }

    fn draw_result_title(&self, player_score: u8, cpu_score: u8, modal_y: f32) {
        let title = if player_score > cpu_score {
            "YOU WIN!"
        } else {
            "YOU LOSE!"
        };

        let dimensions = measure_text(title, Some(&self.font), TITLE_FONT_SIZE, 1.0);

        draw_text_ex(
            title,
            SCREEN_CENTER_X - dimensions.width / 2.0,
            modal_y + TITLE_Y_OFFSET,
            TextParams {
                font: Some(&self.font),
                font_size: TITLE_FONT_SIZE,
                color: WHITE,
                ..Default::default()
            },
        );
    }

    fn draw_score(&self, player_score: u8, cpu_score: u8, modal_y: f32) {
        let score_text = format!("{player_score} X {cpu_score}");

        let dimensions = measure_text(&score_text, Some(&self.font), SCORE_FONT_SIZE, 1.0);

        draw_text_ex(
            &score_text,
            SCREEN_CENTER_X - dimensions.width / 2.0,
            modal_y + SCORE_Y_OFFSET,
            TextParams {
                font: Some(&self.font),
                font_size: SCORE_FONT_SIZE,
                color: WHITE,
                ..Default::default()
            },
        );
    }

    fn draw_options(&self, modal_y: f32) {
        for (index, option) in OPTIONS.iter().enumerate() {
            let color = if index == self.selected_option {
                YELLOW
            } else {
                WHITE
            };

            let dimensions = measure_text(option, Some(&self.font), OPTION_FONT_SIZE, 1.0);

            draw_text_ex(
                option,
                SCREEN_CENTER_X - dimensions.width / 2.0,
                modal_y + OPTIONS_Y_OFFSET + index as f32 * OPTION_SPACING,
                TextParams {
                    font: Some(&self.font),
                    font_size: OPTION_FONT_SIZE,
                    color,
                    ..Default::default()
                },
            );
        }
    }
}
