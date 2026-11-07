use macroquad::prelude::*;
use std::collections::VecDeque;

// --- Constants ---

const TRAIL_LIFETIME: f32 = 0.30;
const TRAIL_SAMPLE_INTERVAL: f32 = 0.012;
const TRAIL_MAX_POINTS: usize = 32;

const TRAIL_MAX_RADIUS: f32 = 5.0;
const TRAIL_MIN_RADIUS: f32 = 2.0;

// --- Types ---

struct TrailPoint {
    position: Vec2,
    lifetime: f32,
}

pub struct BallTrail {
    points: VecDeque<TrailPoint>,
    sample_timer: f32,
}

impl BallTrail {
    pub fn new() -> Self {
        Self {
            points: VecDeque::new(),
            sample_timer: 0.0,
        }
    }

    pub fn update(&mut self, position: Vec2, enabled: bool) {
        let delta_time = get_frame_time();

        // Age existing trail points.
        for point in &mut self.points {
            point.lifetime -= delta_time;
        }

        self.points.retain(|point| point.lifetime > 0.0);

        if !enabled {
            return;
        }

        self.sample_timer -= delta_time;

        if self.sample_timer <= 0.0 {
            self.points.push_back(TrailPoint {
                position: position + vec2(4.0, 4.0),
                lifetime: TRAIL_LIFETIME,
            });

            self.sample_timer = TRAIL_SAMPLE_INTERVAL;
        }

        while self.points.len() > TRAIL_MAX_POINTS {
            self.points.pop_front();
        }
    }

    pub fn draw(&self) {
        for point in &self.points {
            let progress = (point.lifetime / TRAIL_LIFETIME).clamp(0.0, 1.0);

            let radius = TRAIL_MIN_RADIUS + (TRAIL_MAX_RADIUS - TRAIL_MIN_RADIUS) * progress;

            let color = Color {
                a: progress * 0.55,
                ..WHITE
            };

            draw_circle(point.position.x, point.position.y, radius, color);
        }
    }

    pub fn clear(&mut self) {
        self.points.clear();
        self.sample_timer = 0.0;
    }
}
