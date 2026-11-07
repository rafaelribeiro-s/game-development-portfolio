// --- Imports ---

use macroquad::prelude::*;

// --- Layout Constants ---

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

const MODAL_WIDTH: f32 = 300.0;
const MODAL_HEIGHT: f32 = 150.0;

const TITLE_FONT_SIZE: u16 = 12;
const OPTION_FONT_SIZE: u16 = 8;

const TITLE_Y_OFFSET: f32 = 35.0;
const OPTIONS_Y_OFFSET: f32 = 65.0;
const OPTION_SPACING: f32 = 22.0;

const TOTAL_OPTIONS: usize = 4;

const TITLE: &str = "PAUSED";
const OPTIONS: [&str; TOTAL_OPTIONS] = ["RESUME GAME", "RETRY", "SETTINGS", "MAIN MENU"];

const SELECTION_INDICATOR: &str = "> ";
const EMPTY_INDICATOR: &str = "  ";

// --- Pause Menu Actions ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PauseMenuAction {
    ResumeGame,
    Retry,
    Settings,
    MainMenu,
}

// --- Pause Menu ---

pub struct PauseMenu {
    modalbox_texture: Texture2D,
    font: Font,
    selected_option: usize,
}

impl PauseMenu {
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
            self.selected_option = (self.selected_option + TOTAL_OPTIONS - 1) % TOTAL_OPTIONS;
        }
    }

    pub fn draw(&self) {
        let modal_x = (SCREEN_WIDTH - MODAL_WIDTH) / 2.0;
        let modal_y = (SCREEN_HEIGHT - MODAL_HEIGHT) / 2.0;

        self.draw_modal(modal_x, modal_y);
        self.draw_title(modal_x, modal_y);
        self.draw_options(modal_x, modal_y);
    }

    pub fn confirm(&self) -> Option<PauseMenuAction> {
        if !is_key_pressed(KeyCode::Enter) {
            return None;
        }

        match self.selected_option {
            0 => Some(PauseMenuAction::ResumeGame),
            1 => Some(PauseMenuAction::Retry),
            2 => Some(PauseMenuAction::Settings),
            3 => Some(PauseMenuAction::MainMenu),
            _ => None,
        }
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

    fn draw_title(&self, modal_x: f32, modal_y: f32) {
        let dimensions = measure_text(TITLE, Some(&self.font), TITLE_FONT_SIZE, 1.0);

        let x = modal_x + (MODAL_WIDTH - dimensions.width) / 2.0;
        let y = modal_y + TITLE_Y_OFFSET;

        draw_text_ex(
            TITLE,
            x,
            y,
            TextParams {
                font: Some(&self.font),
                font_size: TITLE_FONT_SIZE,
                color: WHITE,
                ..Default::default()
            },
        );
    }

    fn draw_options(&self, modal_x: f32, modal_y: f32) {
        let max_width = OPTIONS
            .iter()
            .map(|option| measure_text(option, Some(&self.font), OPTION_FONT_SIZE, 1.0).width)
            .fold(0.0_f32, f32::max);

        let indicator_width =
            measure_text(SELECTION_INDICATOR, Some(&self.font), OPTION_FONT_SIZE, 1.0).width;

        let block_width = max_width + indicator_width;
        let options_x = modal_x + (MODAL_WIDTH - block_width) / 2.0;

        for (index, option) in OPTIONS.iter().enumerate() {
            let y = modal_y + OPTIONS_Y_OFFSET + index as f32 * OPTION_SPACING;

            let indicator = if index == self.selected_option {
                SELECTION_INDICATOR
            } else {
                EMPTY_INDICATOR
            };

            draw_text_ex(
                format!("{indicator}{option}"),
                options_x,
                y,
                TextParams {
                    font: Some(&self.font),
                    font_size: OPTION_FONT_SIZE,
                    color: WHITE,
                    ..Default::default()
                },
            );
        }
    }
}
