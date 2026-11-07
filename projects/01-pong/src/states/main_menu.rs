// --- Imports ---

use macroquad::prelude::*;

use crate::audio::manager::AudioManager;
use crate::config::{SCREEN_HEIGHT, SCREEN_WIDTH};

// --- Constants ---

const MENU_FONT_SIZE: u16 = 16;
const MENU_START_Y: f32 = 270.0;
const MENU_ITEM_SPACING: f32 = 40.0;
const SELECTION_INDICATOR: &str = "> ";
const EMPTY_INDICATOR: &str = "  ";

// --- Menu Actions ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainMenuAction {
    PlayGame,
    Settings,
    Credits,
    Exit,
}

impl MainMenuAction {
    const ALL: [Self; 4] = [Self::PlayGame, Self::Settings, Self::Credits, Self::Exit];

    fn label(self) -> &'static str {
        match self {
            Self::PlayGame => "PLAY GAME",
            Self::Settings => "SETTINGS",
            Self::Credits => "CREDITS",
            Self::Exit => "EXIT",
        }
    }
}

// --- Main Menu ---

pub struct MainMenu {
    background: Texture2D,
    font: Font,
    selected_option: usize,
}

// --- Construction and Public API ---

impl MainMenu {
    pub fn new(background: Texture2D, font: Font) -> Self {
        Self {
            background,
            font,
            selected_option: 0,
        }
    }

    // --- Input Handling ---

    pub fn update(&mut self, audio_manager: &AudioManager) -> Option<MainMenuAction> {
        let option_count = MainMenuAction::ALL.len();
        let previous_option = self.selected_option;

        if is_key_pressed(KeyCode::Up) {
            self.selected_option = (self.selected_option + option_count - 1) % option_count;
        }

        if is_key_pressed(KeyCode::Down) {
            self.selected_option = (self.selected_option + 1) % option_count;
        }

        // Play navigation sound when the selected option changes.
        if self.selected_option != previous_option {
            audio_manager.play_brick();
        }

        if is_key_pressed(KeyCode::Enter) {
            return Some(MainMenuAction::ALL[self.selected_option]);
        }

        None
    }

    // --- Rendering ---

    pub fn draw(&self) {
        self.draw_background();
        self.draw_menu_options();
    }

    // --- Background Rendering ---

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

    // --- Menu Rendering ---

    fn draw_menu_options(&self) {
        let max_width = MainMenuAction::ALL
            .iter()
            .map(|option| measure_text(option.label(), Some(&self.font), MENU_FONT_SIZE, 1.0).width)
            .fold(0.0_f32, f32::max);

        let indicator_width =
            measure_text(SELECTION_INDICATOR, Some(&self.font), MENU_FONT_SIZE, 1.0).width;

        let block_width = max_width + indicator_width;
        let block_x = (SCREEN_WIDTH as f32 - block_width) / 2.0;

        for (index, option) in MainMenuAction::ALL.iter().enumerate() {
            let y = MENU_START_Y + index as f32 * MENU_ITEM_SPACING;
            let indicator = self.selection_indicator(index);

            draw_text_ex(
                format!("{indicator}{}", option.label()),
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

    // --- Selection Indicator ---

    fn selection_indicator(&self, index: usize) -> &'static str {
        if index == self.selected_option {
            SELECTION_INDICATOR
        } else {
            EMPTY_INDICATOR
        }
    }
}
