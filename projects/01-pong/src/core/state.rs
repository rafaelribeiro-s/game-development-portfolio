// --- Game State ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    SplashScreen,
    MainMenu,
    DifficultySelect,
    Options,
    Credits,
    Gameplay,
    PauseMenu,
    GameOver,
    Quit,
}

// --- State Manager ---

pub struct StateManager {
    current_state: GameState,
}

impl StateManager {
    // --- Construction ---

    pub fn new() -> Self {
        Self {
            current_state: GameState::SplashScreen,
        }
    }

    // --- Public API ---

    pub fn current(&self) -> GameState {
        self.current_state
    }

    pub fn change(&mut self, new_state: GameState) -> bool {
        let is_valid_transition = matches!(
            (self.current_state, new_state),
            (GameState::MainMenu, GameState::DifficultySelect)
                | (GameState::MainMenu, GameState::Options)
                | (GameState::MainMenu, GameState::Credits)
                | (GameState::MainMenu, GameState::Quit)
                | (GameState::DifficultySelect, GameState::Gameplay)
                | (GameState::DifficultySelect, GameState::MainMenu)
                | (GameState::Options, GameState::MainMenu)
                | (GameState::Credits, GameState::MainMenu)
                | (GameState::Gameplay, GameState::PauseMenu)
                | (GameState::Gameplay, GameState::MainMenu)
                | (GameState::Gameplay, GameState::GameOver)
                | (GameState::PauseMenu, GameState::Gameplay)
                | (GameState::PauseMenu, GameState::MainMenu)
                | (GameState::PauseMenu, GameState::Options)
                | (GameState::SplashScreen, GameState::MainMenu)
                | (GameState::GameOver, GameState::Gameplay)
                | (GameState::GameOver, GameState::MainMenu)
                | (GameState::Options, GameState::PauseMenu)
        );

        if is_valid_transition {
            self.current_state = new_state;
        }

        is_valid_transition
    }
}

// --- Tests ---

#[cfg(test)]
mod tests {
    use super::*;

    // --- Initial State ---

    #[test]
    fn starts_at_splash_screen() {
        let state_manager = StateManager::new();

        assert_eq!(state_manager.current(), GameState::SplashScreen);
    }

    // --- State Transitions ---

    #[test]
    fn allows_valid_transition() {
        let mut state_manager = StateManager::new();

        assert!(state_manager.change(GameState::MainMenu));
        assert_eq!(state_manager.current(), GameState::MainMenu);
    }

    #[test]
    fn rejects_invalid_transition() {
        let mut state_manager = StateManager::new();

        assert!(!state_manager.change(GameState::Gameplay));
        assert_eq!(state_manager.current(), GameState::SplashScreen);
    }

    #[test]
    fn allows_gameplay_pause_and_resume() {
        let mut state_manager = StateManager::new();

        assert!(state_manager.change(GameState::MainMenu));
        assert!(state_manager.change(GameState::DifficultySelect));
        assert!(state_manager.change(GameState::Gameplay));
        assert!(state_manager.change(GameState::PauseMenu));
        assert!(state_manager.change(GameState::Gameplay));

        assert_eq!(state_manager.current(), GameState::Gameplay);
    }

    #[test]
    fn allows_game_over_transitions() {
        let mut state_manager = StateManager::new();

        assert!(state_manager.change(GameState::MainMenu));
        assert!(state_manager.change(GameState::DifficultySelect));
        assert!(state_manager.change(GameState::Gameplay));
        assert!(state_manager.change(GameState::GameOver));
        assert!(state_manager.change(GameState::Gameplay));
        assert!(state_manager.change(GameState::GameOver));
        assert!(state_manager.change(GameState::MainMenu));

        assert_eq!(state_manager.current(), GameState::MainMenu);
    }
}
