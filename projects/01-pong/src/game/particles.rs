use macroquad::prelude::*;

const PARTICLE_COUNT: usize = 8;

const PARTICLE_LIFETIME: f32 = 0.22;

const PARTICLE_MIN_SPEED: f32 = 50.0;
const PARTICLE_MAX_SPEED: f32 = 120.0;

const PARTICLE_SIZE: f32 = 2.0;

const PARTICLE_SPREAD: f32 = 0.85;

// --------------------------------------------------
// Particle
// --------------------------------------------------

struct Particle {
    position: Vec2,
    velocity: Vec2,
    lifetime: f32,
    size: f32,
}

// --------------------------------------------------
// Particle System
// --------------------------------------------------

pub struct ParticleSystem {
    particles: Vec<Particle>,
}

impl ParticleSystem {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
        }
    }

    // --------------------------------------------------
    // Emit
    // --------------------------------------------------

    pub fn emit_wall_hit(&mut self, position: Vec2, wall_direction: f32) {
        for i in 0..PARTICLE_COUNT {
            let angle = (i as f32 / PARTICLE_COUNT as f32) * std::f32::consts::TAU;

            let speed = rand::gen_range(PARTICLE_MIN_SPEED, PARTICLE_MAX_SPEED);

            let velocity = vec2(angle.cos() * speed, angle.sin() * speed) * PARTICLE_SPREAD;

            let size = rand::gen_range(1.0, PARTICLE_SIZE + 1.0);

            self.particles.push(Particle {
                position,
                velocity: vec2(velocity.x, velocity.y * wall_direction),
                lifetime: PARTICLE_LIFETIME,
                size,
            });
        }
    }

    // --------------------------------------------------
    // Update
    // --------------------------------------------------

    pub fn update(&mut self) {
        let delta_time = get_frame_time();

        for particle in &mut self.particles {
            particle.position += particle.velocity * delta_time;
            particle.lifetime -= delta_time;
        }

        self.particles.retain(|particle| particle.lifetime > 0.0);
    }

    // --------------------------------------------------
    // Draw
    // --------------------------------------------------

    pub fn draw(&self) {
        for particle in &self.particles {
            draw_rectangle(
                particle.position.x,
                particle.position.y,
                particle.size,
                particle.size,
                WHITE,
            );
        }
    }
}
