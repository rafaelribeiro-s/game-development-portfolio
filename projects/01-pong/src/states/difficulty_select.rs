// --- Imports ---

use macroquad::prelude::*;

use crate::config::{SCREEN_HEIGHT, SCREEN_WIDTH};

// --- Difficulty Options ---

const MENU_FONT_SIZE: u16 = 16;
const MENU_START_Y: f32 = 310.0;
const MENU_ITEM_SPACING: f32 = 50.0;
const SELECTION_INDICATOR: &str = "> ";
const EMPTY_INDICATOR: &str = "  ";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

impl Difficulty {
    const ALL: [Self; 3] = [Self::Easy, Self::Normal, Self::Hard];

    pub fn cpu_config(self) -> crate::game::difficulty::CpuDifficultyConfig {
        use crate::game::difficulty::CpuDifficultyConfig;

        match self {
            Self::Easy => CpuDifficultyConfig::EASY,
            Self::Normal => CpuDifficultyConfig::NORMAL,
            Self::Hard => CpuDifficultyConfig::HARD,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Easy => "EASY",
            Self::Normal => "NORMAL",
            Self::Hard => "HARD",
        }
    }
}

// --- Difficulty Select Actions ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DifficultySelectAction {
    Selected(Difficulty),
    Back,
}

// --- Difficulty Select ---

pub struct DifficultySelect {
    background: Texture2D,
    font: Font,
    selected_option: usize,
}

impl DifficultySelect {
    // --- Construction ---

    pub fn new(background: Texture2D, font: Font) -> Self {
        Self {
            background,
            font,
            selected_option: 0,
        }
    }

    // --- Public API ---

    pub fn update(&mut self) -> Option<DifficultySelectAction> {
        if is_key_pressed(KeyCode::Escape) {
            return Some(DifficultySelectAction::Back);
        }

        if is_key_pressed(KeyCode::Up) {
            self.selected_option =
                (self.selected_option + Difficulty::ALL.len() - 1) % Difficulty::ALL.len();
        }

        if is_key_pressed(KeyCode::Down) {
            self.selected_option = (self.selected_option + 1) % Difficulty::ALL.len();
        }

        if is_key_pressed(KeyCode::Enter) {
            return Some(DifficultySelectAction::Selected(
                Difficulty::ALL[self.selected_option],
            ));
        }

        None
    }

    pub fn draw(&self) {
        self.draw_background();
        self.draw_difficulty_options();
    }

    // --- Rendering ---

    fn draw_background(&self) {
        draw_texture_ex(
            &self.background,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32)),
                ..Default::default()
            },
        );
    }

    fn draw_difficulty_options(&self) {
        let max_width = Difficulty::ALL
            .iter()
            .map(|difficulty| {
                measure_text(difficulty.label(), Some(&self.font), MENU_FONT_SIZE, 1.0).width
            })
            .fold(0.0_f32, f32::max);

        let indicator_width =
            measure_text(SELECTION_INDICATOR, Some(&self.font), MENU_FONT_SIZE, 1.0).width;

        let block_width = max_width + indicator_width;
        let block_x = (SCREEN_WIDTH as f32 - block_width) / 2.0;

        for (index, difficulty) in Difficulty::ALL.iter().enumerate() {
            let y = MENU_START_Y + index as f32 * MENU_ITEM_SPACING;

            let indicator = if index == self.selected_option {
                SELECTION_INDICATOR
            } else {
                EMPTY_INDICATOR
            };

            draw_text_ex(
                &format!("{indicator}{}", difficulty.label()),
                block_x,
                y,
                TextParams {
                    font: Some(&self.font),
                    font_size: MENU_FONT_SIZE,
                    color: WHITE,
                    ..Default::default()
                },
            );
        }
    }
}
