// --- Imports ---

use macroquad::prelude::*;

use crate::config::SCREEN_WIDTH;

// --- Constants ---

const TITLE_FONT_SIZE: u16 = 16;
const SECTION_FONT_SIZE: u16 = 8;
const TEXT_FONT_SIZE: u16 = 8;

// --- Layout Coordinates ---

const CONTENT_LEFT: f32 = 80.0;
const CONTENT_RIGHT: f32 = 720.0;

// --- Credits ---

pub struct Credits {
    font: Font,
}

// --- Construction and Public API ---

impl Credits {
    pub fn new(font: Font) -> Self {
        Self { font }
    }

    // --- Input Handling ---

    pub fn update(&self) -> bool {
        is_key_pressed(KeyCode::Escape)
    }

    // --- Rendering ---

    pub fn draw(&self) {
        clear_background(BLACK);

        self.draw_header();
        self.draw_author_section();
        self.draw_tech_stack_section();
        self.draw_assets_section();
        self.draw_links_section();
    }

    // --- Header ---

    fn draw_header(&self) {
        self.draw_centered("PONG", 45.0, TITLE_FONT_SIZE);

        self.draw_separator(60.0, 2.0);
    }

    // --- Author Section ---

    fn draw_author_section(&self) {
        self.draw_centered("A SOLO GAME BY", 88.0, SECTION_FONT_SIZE);
        self.draw_centered("Rafael Ribeiro", 110.0, TEXT_FONT_SIZE);

        self.draw_centered("Design, Code & Architecture:", 145.0, TEXT_FONT_SIZE);
        self.draw_centered("Rafael Ribeiro", 165.0, TEXT_FONT_SIZE);

        self.draw_separator(185.0, 1.0);
    }

    // --- Tech Stack Section ---

    fn draw_tech_stack_section(&self) {
        self.draw_centered("TECH STACK & TOOLS", 207.0, SECTION_FONT_SIZE);

        self.draw_centered("Language: Rust", 245.0, TEXT_FONT_SIZE);
        self.draw_centered("Framework: Macroquad", 263.0, TEXT_FONT_SIZE);
        self.draw_centered("Environment: VSCodium", 281.0, TEXT_FONT_SIZE);
        self.draw_centered("Art & Assets: GIMP", 299.0, TEXT_FONT_SIZE);

        self.draw_separator(318.0, 1.0);
    }

    // --- Assets Section ---

    fn draw_assets_section(&self) {
        self.draw_centered("ASSETS & CREDITS", 340.0, SECTION_FONT_SIZE);

        self.draw_centered("Typography: Press Start 2P", 375.0, TEXT_FONT_SIZE);
        self.draw_centered("by codeman38", 395.0, TEXT_FONT_SIZE);
        self.draw_centered("SIL Open Font License", 415.0, TEXT_FONT_SIZE);

        self.draw_separator(435.0, 1.0);
    }

    // --- Links Section ---

    fn draw_links_section(&self) {
        self.draw_centered("LINKS & PORTFOLIO", 457.0, SECTION_FONT_SIZE);

        self.draw_centered("GitHub Profile:", 487.0, TEXT_FONT_SIZE);
        self.draw_centered("github.com/rafaelribeiro-s", 507.0, TEXT_FONT_SIZE);

        self.draw_centered("THANKS FOR PLAYING!", 550.0, SECTION_FONT_SIZE);

        self.draw_separator(570.0, 2.0);
    }

    // --- Drawing Helpers ---

    fn draw_centered(&self, text: &str, y: f32, font_size: u16) {
        let dimensions = measure_text(text, Some(&self.font), font_size, 1.0);
        let x = (SCREEN_WIDTH as f32 - dimensions.width) / 2.0;

        draw_text_ex(
            text,
            x,
            y,
            TextParams {
                font: Some(&self.font),
                font_size,
                color: WHITE,
                ..Default::default()
            },
        );
    }

    fn draw_separator(&self, y: f32, thickness: f32) {
        draw_line(CONTENT_LEFT, y, CONTENT_RIGHT, y, thickness, WHITE);
    }
}
