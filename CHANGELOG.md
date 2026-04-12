### 0.0.0 / 2025-05-02

- Binary size/performance optimization of a very basic Ratatui app

// 3.4M

### 0.0.1 / 2026-04-04

- Add lints rules
- Add basic `main.rs`/`app.rs` scaffolding and main loop
- Add logger
- Shave off unnecessary dependencies and/or their features
- Replace `color-eyre` with`anyhow`

// 602K (615.592)

### 0.0.2 / 2026-04-04

- Introduce the Main Buffer `stderr` messages and improve error handling
- Put log file in a more appropriate location: ~/.local/state/env!("CARGO_PKG_NAME")
- Check a minimum number of terminal columns and rows
- Refactor `main.rs` and `logger.rs`

// 611K (625.328)

### 0.0.3 / 2026-04-05

- First draft of layout
- Add multicolored in-console logging, lines are scrollable using keyboard and mouse, toggleable via `[features]`
- Put in-file and in-console logging in a separate module
- Improve frame rate and responsiveness

// 622K (636.384)

### 0.0.4 / 2026-04-06

- opt-level = "z"
- Rearrange and clean up `app.rs` as a preparation for MVC
- Improve usage of "dev-console" feature
- Improve `dev_console.rs` by introducing `std::sync::mpsc` for log messages
- Make crate-wide available macros for colored logging to dev console
- Greatly improve `main.rs`: robustness, flexibility regarding non-essential conditions for app starting, error handling, panic protection, terminal restoration, logging

// 613K (627.624)

### 0.0.5 / 2026-04-06

- Basic MVC

// 614K (627.768)

### 0.0.6 / 2026-04-08

- Create game board border "manually" (writting directly to terminal buffer)
- Add `fastrand` dependency
- Creating blocks in random colors
- Falling, manipulable column (handling user input), spawning at random x position

// 623K (637.496)

### 0.0.7 / 2026-04-11

- Respawn columns in a loop
- Pile up columns
- Detect piled blocks for a falling column
- Detect game over
- Log panic error to file

// 625K (639.392)

### 0.0.8 / 2026-04-12

- Delegated state pattern: stages with handlers and separate renders
- Keys legend at the bottom
- "Ready" stage
- "Gameplay" stage
- "Game Over" stage

// 629K (643.128)
