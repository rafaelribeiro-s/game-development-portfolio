// --- Imports ---

use macroquad::prelude::*;

// --- Render Target ---

pub fn create_render_target() -> RenderTarget {
    let target = render_target(crate::config::SCREEN_WIDTH, crate::config::SCREEN_HEIGHT);

    target.texture.set_filter(FilterMode::Nearest);

    target
}

// --- Viewport Calculation ---

pub fn calculate_viewport(window_width: f32, window_height: f32) -> Rect {
    let virtual_aspect = crate::config::SCREEN_WIDTH as f32 / crate::config::SCREEN_HEIGHT as f32;

    let window_aspect = window_width / window_height;

    if window_aspect > virtual_aspect {
        let viewport_width = window_height * virtual_aspect;

        Rect::new(
            (window_width - viewport_width) / 2.0,
            0.0,
            viewport_width,
            window_height,
        )
    } else {
        let viewport_height = window_width / virtual_aspect;

        Rect::new(
            0.0,
            (window_height - viewport_height) / 2.0,
            window_width,
            viewport_height,
        )
    }
}

// --- Frame Presentation ---

pub fn present_frame(render_target: &RenderTarget) {
    let viewport = calculate_viewport(screen_width(), screen_height());

    set_default_camera();
    clear_background(BLACK);

    draw_texture_ex(
        &render_target.texture,
        viewport.x,
        viewport.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(viewport.w, viewport.h)),
            ..Default::default()
        },
    );
}
