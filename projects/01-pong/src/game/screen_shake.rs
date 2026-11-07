use macroquad::prelude::*;

const SHAKE_DURATION: f32 = 0.10;
const SHAKE_SMOOTHING: f32 = 01.0;

pub struct ScreenShake {
    timer: f32,
    duration: f32,
    intensity: f32,
    offset: Vec2,
    target_offset: Vec2,
}

impl ScreenShake {
    pub fn new() -> Self {
        Self {
            timer: 0.0,
            duration: SHAKE_DURATION,
            intensity: 0.0,
            offset: Vec2::ZERO,
            target_offset: Vec2::ZERO,
        }
    }

    pub fn trigger(&mut self, intensity: f32) {
        self.timer = self.duration;
        self.intensity = intensity;
    }

    pub fn update(&mut self) {
        let dt = get_frame_time();

        if self.timer > 0.0 {
            self.timer = (self.timer - dt).max(0.0);

            let fade = self.timer / self.duration;

            self.target_offset = vec2(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0))
                * self.intensity
                * fade;
        } else {
            self.target_offset = Vec2::ZERO;
        }

        let smoothing = 1.0 - (-SHAKE_SMOOTHING * dt).exp();

        self.offset = self.offset.lerp(self.target_offset, smoothing);

        if self.timer <= 0.0 && self.offset.length() < 0.01 {
            self.offset = Vec2::ZERO;
        }
    }

    pub fn offset(&self) -> Vec2 {
        self.offset
    }
}
