// --- CPU Difficulty Configuration ---

#[derive(Debug, Clone, Copy)]
pub struct CpuDifficultyConfig {
    pub speed: f32,
    pub reaction_time: f32,
    pub error_margin: f32,
}

// --- Difficulty Presets ---

impl CpuDifficultyConfig {
    pub const EASY: Self = Self {
        speed: 240.0,
        reaction_time: 0.30,
        error_margin: 24.0,
    };

    pub const NORMAL: Self = Self {
        speed: 320.0,
        reaction_time: 0.18,
        error_margin: 10.0,
    };

    pub const HARD: Self = Self {
        speed: 400.0,
        reaction_time: 0.12,
        error_margin: 6.0,
    };
}
