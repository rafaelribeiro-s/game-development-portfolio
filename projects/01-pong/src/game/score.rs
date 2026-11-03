// --- Constants ---

/// Maximum score required to win the match.
pub const WINNING_SCORE: u8 = 5;

// --- Score ---

/// Stores the points earned by each player.
#[derive(Debug, Default)]
pub struct Score {
    player: u8,
    cpu: u8,
}

// --- Construction and Public API ---

impl Score {
    pub fn new() -> Self {
        Self::default()
    }

    // --- Score Updates ---

    pub fn add_player_point(&mut self) {
        self.player = self.player.saturating_add(1);
    }

    pub fn add_cpu_point(&mut self) {
        self.cpu = self.cpu.saturating_add(1);
    }

    // --- Score Accessors ---

    pub fn player_score(&self) -> u8 {
        self.player
    }

    pub fn cpu_score(&self) -> u8 {
        self.cpu
    }

    // --- Match Status ---

    pub fn is_match_over(&self) -> bool {
        self.player >= WINNING_SCORE || self.cpu >= WINNING_SCORE
    }
}
