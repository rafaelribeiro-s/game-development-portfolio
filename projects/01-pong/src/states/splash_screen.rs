// --- Imports ---

use macroquad::prelude::*;

use crate::config::{SCREEN_HEIGHT, SCREEN_WIDTH};

// --- Constants ---

const INSTRUCTION_TEXT: &str = "PRESS ENTER";
const INSTRUCTION_FONT_SIZE: u16 = 16;
const INSTRUCTION_Y: f32 = 463.0;
const BLINK_INTERVAL: f64 = 1.0;
const BLINK_DURATION: f64 = 0.5;

// --- Splash Screen ---

pub struct SplashScreen {
    background: Texture2D,
    font: Font,
}

// --- Construction and Public API ---

impl SplashScreen {
    pub fn new(background: Texture2D, font: Font) -> Self {
        Self { background, font }
    }

    // --- Input Handling ---

    pub fn update(&self) -> bool {
        is_key_pressed(KeyCode::Enter)
    }

    // --- Rendering ---

    pub fn draw(&self) {
        self.draw_background();
        self.draw_blinking_instruction();
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

    // --- Instruction Rendering ---

    fn draw_blinking_instruction(&self) {
        if (get_time() % BLINK_INTERVAL) >= BLINK_DURATION {
            return;
        }

        let dimensions = measure_text(
            INSTRUCTION_TEXT,
            Some(&self.font),
            INSTRUCTION_FONT_SIZE,
            1.0,
        );

        let x = (SCREEN_WIDTH as f32 - dimensions.width) / 2.0;

        draw_text_ex(
            INSTRUCTION_TEXT,
            x,
            INSTRUCTION_Y,
            TextParams {
                font: Some(&self.font),
                font_size: INSTRUCTION_FONT_SIZE,
                color: WHITE,
                ..Default::default()
            },
        );
    }
}
