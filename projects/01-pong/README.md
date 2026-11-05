# Pong — 2D Game Development Project

A single-player Pong implementation built with **Rust** and **Macroquad**, featuring custom gameplay mechanics, a rule-based CPU opponent, state management, audio integration, and configurable settings.

This project is part of my Game Development Portfolio and demonstrates practical applications of game architecture, physics simulation, collision handling, input management, and audio systems.

---

## 🎮 Project Overview

The project recreates the classic Pong experience with a minimalist monochrome visual style and a 4:3 gameplay presentation.

The player competes against a CPU-controlled opponent across three difficulty levels.

**Project Status:** Completed  
**Project Type:** 2D Arcade Game  
**Resolution:** 800 × 600  
**Aspect Ratio:** 4:3

## 🧠 Core Features

- **Gameplay Mechanics:** Paddle movement, ball physics, scoring, and round progression.
- **Collision Detection:** Paddle and wall collision handling, including substep-based updates to reduce tunneling.
- **CPU Opponent:** Rule-based AI with Easy, Normal, and Hard difficulty levels.
- **Game State Management:** Splash Screen, Main Menu, Difficulty Selection, Gameplay, Pause, Game Over, Credits, and Settings.
- **Audio Integration:** Sound effects and looping background music using Rodio.
- **Settings System:** Independent sound effects and background music volume controls, adjustable in 5% increments.
- **Visual Design:** Monochrome 1-bit-inspired presentation and pixel-art assets.
- **Frame-Rate Independence:** Delta-time-based gameplay updates.

## 🛠️ Technology Stack

| Technology       | Purpose                           |
| ---------------- | --------------------------------- |
| Rust             | Programming language              |
| Macroquad 0.4.16 | 2D game framework and rendering   |
| Rodio 0.21.1     | Audio playback                    |
| Cargo            | Dependency and project management |
| VSCodium         | Development environment           |
| GIMP             | Asset creation and editing        |
| Press Start 2P   | Pixel-style typography            |

## 🕹️ Controls

| Key   | Action                             |
| ----- | ---------------------------------- |
| ↑ / ↓ | Navigate menus and move the paddle |
| ← / → | Adjust selected volume in Settings |
| Space | Serve the ball                     |
| Enter | Confirm menu selection             |
| Esc   | Pause, go back, or cancel          |

## 🚀 Getting Started

### Prerequisites

- Rust toolchain
- Cargo
- Linux, Windows, or another supported Macroquad environment

### Run the Game

Clone the repository:

```bash
git clone https://github.com/rafaelribeiro-s/game-development-portfolio.git
cd game-development-portfolio
```

Run the Pong project from the workspace root:

```bash
cargo run -p pong
```

### Build and Validate

Check the project:

```bash
cargo check -p pong
```

Run the tests:

```bash
cargo test -p pong
```

Format the source code:

```bash
cargo fmt
```

## 📂 Project Structure

```text
01-pong/
├── assets/
│   ├── audio/
│   ├── fonts/
│   └── images/
├── src/
│   ├── audio/
│   ├── core/
│   ├── game/
│   ├── graphics/
│   ├── states/
│   └── main.rs
├── Cargo.toml
└── Cargo.lock
```

## 📚 Technical Documentation

The project's technical case study will document the engineering process behind the implementation, including:

- Game architecture and module responsibilities
- Physics and collision handling
- CPU opponent behavior and difficulty scaling
- Finite State Machine (FSM) and state transitions
- Audio integration and volume management
- Testing, validation, and problem-solving decisions

**Technical Case Study:** Documentation in progress.

## 🎯 Learning Objectives

This project explores practical game development concepts:

- Applying Rust to interactive 2D applications
- Organizing game systems using modular architecture
- Implementing collision detection and response
- Designing rule-based opponent behavior
- Managing game states and user input
- Integrating external audio playback
- Validating functionality through testing

## 👤 Author

**Rafael Ribeiro**

Game Development Portfolio

- GitHub: [rafaelribeiro-s](https://github.com/rafaelribeiro-s)
- Repository: [Game Development Portfolio](https://github.com/rafaelribeiro-s/game-development-portfolio)

---

*Built with Rust, Macroquad, and a focus on practical game engineering.*
