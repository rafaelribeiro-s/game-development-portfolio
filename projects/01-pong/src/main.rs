// --- Imports ---

mod audio;
mod config;
mod core;
mod game;
mod graphics;
mod states;

use macroquad::prelude::*;

use audio::manager::AudioManager;

use config::{SCREEN_HEIGHT, SCREEN_WIDTH, WINDOW_TITLE};
use core::state::{GameState, StateManager};
use game::difficulty::CpuDifficultyConfig;
use graphics::{create_render_target, present_frame};
use states::credits::Credits;
use states::difficulty_select::{DifficultySelect, DifficultySelectAction};
use states::game_over_menu::{GameOverMenu, GameOverMenuAction};
use states::gameplay::Gameplay;
use states::main_menu::{MainMenu, MainMenuAction};
use states::pause_menu::{PauseMenu, PauseMenuAction};
use states::settings::Settings;
use states::splash_screen::SplashScreen;

// --- Window Configuration ---

fn window_conf() -> Conf {
    Conf {
        window_title: WINDOW_TITLE.to_string(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        window_resizable: false,
        high_dpi: false,
        ..Default::default()
    }
}

// --- Application Entry Point ---

#[macroquad::main(window_conf)]
async fn main() {
    // --- Rendering Setup ---

    let render_target = create_render_target();

    let camera = Camera2D {
        render_target: Some(render_target.clone()),
        zoom: vec2(2.0 / SCREEN_WIDTH as f32, 2.0 / SCREEN_HEIGHT as f32),
        target: vec2(SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0),
        ..Default::default()
    };

    // --- Asset Loading ---

    let splash_texture = load_texture("projects/01-pong/assets/images/splash_screen.png")
        .await
        .expect("Failed to load splash screen image");

    splash_texture.set_filter(FilterMode::Nearest);

    let main_menu_texture = load_texture("projects/01-pong/assets/images/main-menu-screen.png")
        .await
        .expect("Failed to load main menu image");

    main_menu_texture.set_filter(FilterMode::Nearest);

    let gameplay_texture = load_texture("projects/01-pong/assets/images/gameplay-screen.png")
        .await
        .expect("Failed to load gameplay background");

    let splash_font =
        load_ttf_font_from_bytes(include_bytes!("../assets/fonts/PressStart2P-Regular.ttf"))
            .expect("Failed to load splash screen font");

    let menu_font =
        load_ttf_font_from_bytes(include_bytes!("../assets/fonts/PressStart2P-Regular.ttf"))
            .expect("Failed to load main menu font");

    let player_paddle_texture = load_texture("projects/01-pong/assets/images/player-paddle.png")
        .await
        .expect("Failed to load player paddle");

    let cpu_paddle_texture = load_texture("projects/01-pong/assets/images/cpu-paddle.png")
        .await
        .expect("Failed to load CPU paddle");

    let ball_texture = load_texture("projects/01-pong/assets/images/ball.png")
        .await
        .expect("Failed to load ball");

    let modalbox_texture = load_texture("projects/01-pong/assets/images/modalbox.png")
        .await
        .expect("Failed to load modalbox.png");

    modalbox_texture.set_filter(FilterMode::Nearest);

    // --- Audio Initialization ---

    let mut audio_manager = AudioManager::new();
    audio_manager.play_music();

    // --- State Initialization ---

    let splash_screen = SplashScreen::new(splash_texture, splash_font);

    let mut main_menu = MainMenu::new(main_menu_texture.clone(), menu_font.clone());

    let credits = Credits::new(menu_font.clone());

    let mut difficulty_select = DifficultySelect::new(main_menu_texture, menu_font.clone());

    let mut state_manager = StateManager::new();

    let mut gameplay: Option<Gameplay> = None;
    let mut selected_cpu_config: Option<CpuDifficultyConfig> = None;

    let mut pause_menu = PauseMenu::new(modalbox_texture.clone(), menu_font.clone());

    let mut settings = Settings::new(menu_font.clone());
    let mut settings_return_state = GameState::MainMenu;

    let mut game_over_menu = GameOverMenu::new(modalbox_texture.clone(), menu_font.clone());

    // --- Main Game Loop ---

    loop {
        // --- State Update ---

        match state_manager.current() {
            GameState::SplashScreen => {
                if splash_screen.update() {
                    state_manager.change(GameState::MainMenu);
                }
            }

            GameState::MainMenu => {
                audio_manager.play_music();

                if let Some(action) = main_menu.update() {
                    match action {
                        MainMenuAction::PlayGame => {
                            state_manager.change(GameState::DifficultySelect);
                        }

                        MainMenuAction::Settings => {
                            settings_return_state = GameState::MainMenu;
                            state_manager.change(GameState::Options);
                        }

                        MainMenuAction::Credits => {
                            state_manager.change(GameState::Credits);
                        }

                        MainMenuAction::Exit => {
                            state_manager.change(GameState::Quit);
                        }
                    }
                }
            }

            GameState::DifficultySelect => {
                if let Some(action) = difficulty_select.update() {
                    match action {
                        DifficultySelectAction::Selected(difficulty) => {
                            let cpu_config = difficulty.cpu_config();
                            selected_cpu_config = Some(cpu_config);

                            gameplay = Some(Gameplay::new(
                                gameplay_texture.clone(),
                                player_paddle_texture.clone(),
                                cpu_paddle_texture.clone(),
                                ball_texture.clone(),
                                cpu_config,
                                menu_font.clone(),
                            ));

                            state_manager.change(GameState::Gameplay);
                        }

                        DifficultySelectAction::Back => {
                            state_manager.change(GameState::MainMenu);
                        }
                    }
                }
            }

            GameState::Options => {
                if is_key_pressed(KeyCode::Escape) {
                    state_manager.change(settings_return_state);
                } else {
                    settings.update();

                    audio_manager
                        .set_sound_effects_volume(settings.sound_effects_volume as f32 / 100.0);

                    audio_manager.set_music_volume(settings.background_music_volume as f32 / 100.0);
                }
            }

            GameState::Credits => {
                if credits.update() {
                    state_manager.change(GameState::MainMenu);
                }
            }

            GameState::Gameplay => {
                audio_manager.stop_music();

                if is_key_pressed(KeyCode::Escape) {
                    state_manager.change(GameState::PauseMenu);
                } else if let Some(gameplay) = &mut gameplay {
                    gameplay.update(&audio_manager);

                    if gameplay.is_match_over() {
                        state_manager.change(GameState::GameOver);
                    }
                }
            }

            GameState::PauseMenu => {
                if is_key_pressed(KeyCode::Escape) {
                    state_manager.change(GameState::Gameplay);
                } else {
                    pause_menu.update();

                    if let Some(action) = pause_menu.confirm() {
                        match action {
                            PauseMenuAction::ResumeGame => {
                                state_manager.change(GameState::Gameplay);
                            }

                            PauseMenuAction::Retry => {
                                if let Some(cpu_config) = selected_cpu_config {
                                    gameplay = Some(Gameplay::new(
                                        gameplay_texture.clone(),
                                        player_paddle_texture.clone(),
                                        cpu_paddle_texture.clone(),
                                        ball_texture.clone(),
                                        cpu_config,
                                        menu_font.clone(),
                                    ));

                                    state_manager.change(GameState::Gameplay);
                                }
                            }

                            PauseMenuAction::Settings => {
                                settings_return_state = GameState::PauseMenu;
                                state_manager.change(GameState::Options);
                            }

                            PauseMenuAction::MainMenu => {
                                state_manager.change(GameState::MainMenu);
                            }
                        }
                    }
                }
            }

            GameState::GameOver => {
                game_over_menu.update();

                if let Some(action) = game_over_menu.confirm() {
                    match action {
                        GameOverMenuAction::PlayAgain => {
                            if let Some(cpu_config) = selected_cpu_config {
                                gameplay = Some(Gameplay::new(
                                    gameplay_texture.clone(),
                                    player_paddle_texture.clone(),
                                    cpu_paddle_texture.clone(),
                                    ball_texture.clone(),
                                    cpu_config,
                                    menu_font.clone(),
                                ));

                                game_over_menu =
                                    GameOverMenu::new(modalbox_texture.clone(), menu_font.clone());

                                state_manager.change(GameState::Gameplay);
                            }
                        }

                        GameOverMenuAction::MainMenu => {
                            state_manager.change(GameState::MainMenu);
                        }
                    }
                }
            }

            GameState::Quit => break,
        }

        // --- Virtual Rendering ---

        set_camera(&camera);
        clear_background(BLACK);

        match state_manager.current() {
            GameState::SplashScreen => {
                splash_screen.draw();
            }

            GameState::MainMenu => {
                main_menu.draw();
            }

            GameState::DifficultySelect => {
                difficulty_select.draw();
            }

            GameState::Options => {
                settings.draw();
            }

            GameState::Credits => {
                credits.draw();
            }

            GameState::Gameplay => {
                if let Some(gameplay) = &gameplay {
                    gameplay.draw();
                }
            }

            GameState::PauseMenu => {
                if let Some(gameplay) = &gameplay {
                    gameplay.draw();
                }

                pause_menu.draw();
            }

            GameState::GameOver => {
                if let Some(gameplay) = &gameplay {
                    gameplay.draw();

                    game_over_menu.draw(gameplay.player_score(), gameplay.cpu_score());
                }
            }

            GameState::Quit => break,
        }

        // --- Frame Presentation ---

        present_frame(&render_target);

        next_frame().await;
    }
}
