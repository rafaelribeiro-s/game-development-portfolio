// --- Imports ---

use macroquad::prelude::*;

// --- Settings Constants ---

pub const VOLUME_STEP: u8 = 5;
pub const DEFAULT_VOLUME: u8 = 100;

const TITLE: &str = "SETTINGS";
const TITLE_SIZE: u16 = 24;
const TITLE_Y: f32 = 150.0;

const MENU_START_Y: f32 = 245.0;
const MENU_ITEM_SPACING: f32 = 110.0;
const LABEL_FONT_SIZE: u16 = 14;

const BLOCK_SIZE: f32 = 12.0;
const BLOCK_GAP: f32 = 4.0;
const TOTAL_BLOCKS: usize = 20;
const VOLUME_FONT_SIZE: u16 = 12;

const BACK_TEXT: &str = "ESC - BACK";
const BACK_TEXT_X: f32 = 300.0;
const BACK_TEXT_Y: f32 = 490.0;

// --- Settings ---

pub struct Settings {
    pub sound_effects_volume: u8,
    pub background_music_volume: u8,
    selected_option: usize,
    font: Font,
}

impl Settings {
    // --- Construction ---

    pub fn new(font: Font) -> Self {
        Self {
            sound_effects_volume: DEFAULT_VOLUME,
            background_music_volume: DEFAULT_VOLUME,
            selected_option: 0,
            font,
        }
    }

    // --- Public API ---

    pub fn update(&mut self) {
        if is_key_pressed(KeyCode::Up) {
            self.selected_option = self.selected_option.saturating_sub(1);
        }

        if is_key_pressed(KeyCode::Down) {
            self.selected_option = (self.selected_option + 1).min(1);
        }

        if is_key_pressed(KeyCode::Left) {
            self.change_volume(-1);
        }

        if is_key_pressed(KeyCode::Right) {
            self.change_volume(1);
        }
    }

    pub fn draw(&self) {
        self.draw_title();
        self.draw_volume_options();
        self.draw_back_instruction();
    }

    // --- Volume Controls ---

    fn change_volume(&mut self, direction: i8) {
        let volume = match self.selected_option {
            0 => &mut self.sound_effects_volume,
            1 => &mut self.background_music_volume,
            _ => return,
        };

        let new_volume = *volume as i16 + direction as i16 * VOLUME_STEP as i16;

        *volume = new_volume.clamp(0, 100) as u8;
    }

    // --- Rendering ---

    fn draw_title(&self) {
        let title_dimensions = measure_text(TITLE, Some(&self.font), TITLE_SIZE, 1.0);

        draw_text_ex(
            TITLE,
            (800.0 - title_dimensions.width) / 2.0,
            TITLE_Y,
            TextParams {
                font: Some(&self.font),
                font_size: TITLE_SIZE,
                color: WHITE,
                ..Default::default()
            },
        );
    }

    fn draw_volume_options(&self) {
        let options = [
            ("SOUND EFFECTS", self.sound_effects_volume),
            ("BACKGROUND MUSIC", self.background_music_volume),
        ];

        let bar_width = TOTAL_BLOCKS as f32 * (BLOCK_SIZE + BLOCK_GAP) - BLOCK_GAP;
        let bar_x = (800.0 - bar_width) / 2.0;

        for (index, (label, volume)) in options.iter().enumerate() {
            let label_y = MENU_START_Y + index as f32 * MENU_ITEM_SPACING;
            let bar_y = label_y + 25.0;

            self.draw_volume_label(index, label, label_y);
            self.draw_volume_bar(*volume, bar_x, bar_y);
            self.draw_volume_percentage(*volume, bar_x, bar_width, bar_y);
        }
    }

    fn draw_volume_label(&self, index: usize, label: &str, y: f32) {
        let is_selected = self.selected_option == index;

        let color = if is_selected { WHITE } else { GRAY };
        let prefix = if is_selected { ">" } else { " " };

        draw_text_ex(
            format!("{prefix} {label}"),
            170.0,
            y,
            TextParams {
                font: Some(&self.font),
                font_size: LABEL_FONT_SIZE,
                color,
                ..Default::default()
            },
        );
    }

    fn draw_volume_bar(&self, volume: u8, x: f32, y: f32) {
        let filled_blocks = (volume / VOLUME_STEP) as usize;

        for block in 0..TOTAL_BLOCKS {
            let block_x = x + block as f32 * (BLOCK_SIZE + BLOCK_GAP);

            let color = if block < filled_blocks {
                WHITE
            } else {
                DARKGRAY
            };

            draw_rectangle(block_x, y, BLOCK_SIZE, BLOCK_SIZE, color);
        }
    }

    fn draw_volume_percentage(&self, volume: u8, bar_x: f32, bar_width: f32, bar_y: f32) {
        draw_text_ex(
            format!("{volume}%"),
            bar_x + bar_width + 15.0,
            bar_y + BLOCK_SIZE,
            TextParams {
                font: Some(&self.font),
                font_size: VOLUME_FONT_SIZE,
                color: WHITE,
                ..Default::default()
            },
        );
    }

    fn draw_back_instruction(&self) {
        draw_text_ex(
            BACK_TEXT,
            BACK_TEXT_X,
            BACK_TEXT_Y,
            TextParams {
                font: Some(&self.font),
                font_size: VOLUME_FONT_SIZE,
                color: GRAY,
                ..Default::default()
            },
        );
    }
}
