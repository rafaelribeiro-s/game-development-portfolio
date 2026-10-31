use macroquad::prelude::*;

// --- Window Configuration ---

fn window_conf() -> Conf {
    Conf {
        window_title: "Pong".to_string(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        high_dpi: false,
        ..Default::default()
    }
}

// --- Main ---

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(BLACK);

        next_frame().await;
    }
}
