// --- Round State ---

/// Controls the current phase of a round.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoundState {
    WaitingForServe,
    Playing,
    Countdown,
    MatchOver,
}
