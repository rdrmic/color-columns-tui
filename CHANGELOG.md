### 0.1.6 / 2026-06-03

- Enable blinking labels
- Adjust messages' fading out
- Derive `Copy` and `Clone` on appropriate structs and enums (which hold "plain" values and are <= 8 bytes)
- Rename `Game` to `GameState`
- Tidy up comments and remove unnecessary clippy allow annotation
- `cargo update`
- Use toolchain `nightly-2026-06-03`

// 459K (469.504)

### 0.1.5 / 2026-06-01

- Clean up TODOs and FIXMEs
- Improve messages' fading out
- Bump Rust version: 1.95.0 -> 1.96.0
- `cargo update`
- Binary size optimizations:
  - Use toolchain `nightly-2026-06-01` (-1.712)

// 459K (469.120)

### 0.1.4 / 2026-05-31

- Reorganize layout to show level and in-game messages
- In-game messages
- Make `dev_console` take string literals AND strings to be formatted

// 460K (470.816)

### 0.1.3 / 2026-05-25

- Adjust leveling; refactor scoring
- Fix missing score updating when there are no hanging gems
- For falling columns, replace `Option` with `FallingColumnPlaceholder`
- Extract `get_game_state_after_matches_search` function
- Refactor

// 458K (468.320)

### 0.1.2 / 2026-05-09

- Binary size optimizations:
  - Use linker garbage collection and ICF (Identical Code Folding): `-Clink-arg=-Wl,--gc-sections` and `-Clink-arg=-Wl,--icf=all` (-6.528)
  - Enable `-Zlocation-detail=none`  (-37.424)
  - Enable `-Zfmt-debug=none` (-9.920)
  - Reduce inline threshold for monomorphization control (`-Cllvm-args=--inline-threshold=45`) (-11.376)
  - Add `stderr_warning` utility function in `main.rs` (-224)

// 458K (468.640)

### 0.1.1 / 2026-05-08

- Skip tick if there aren't hanging blocks
- Log random seed for reproducibility
- Simplify `GameOverHandler`
- Binary size optimizations:
  - Render key legends dynamically
  - Remove all `#[derive(Debug)]` attributes
  - Implement `Debug` for `Error` in `errors.rs`
  - Replace `with_context` debug-formatted message with unified `.context` one in `game.rs`: `create_rng`
  - Eliminate `f64` math for acceleration calculation
  - Use `concat!` macro to create terminal title in compile time
  - Reuse `FAILED_TO_START_GAME_ERROR` `&str` constant across stage handlers

// 522K (534.112)

### 0.1.0 / 2026-05-06    --> Fully playable game

- Persist and read high score from file
- Use common application state directory (for log and high score files)
- Improve/optimize and refactor

// 525K (536.872)

### 0.0.15 / 2026-05-06

- Make clearing matches and "gravity" effect visually sequential

// 520K (532.128)

### 0.0.14 / 2026-05-02

- Find and clear the matches
- Add scoring calculations
- Fix randomizing Gems in `Paused` stage
- Move listening to `F1` key to `ReadyHandler` only
- Adjust instructions text and improve layout

// 521K (533.352)

### 0.0.13 / 2026-04-28

- Binary size optimizations:
    - Change `lto = true` to `lto = "fat"`
    - Add `features = ["release_max_level_off"]` to `log`
    - Replace `env_logger` with a simple custom logger
    - Remove `time`
    - Replace `anyhow` with "manual" error handling
    - Replace unnecessary `{:?}`/`{:#?}` with `{}`
    - Introduce `rust-toolchain.toml` with `channel = "nightly-2026-04-26"`

// 511K (523.080)

## 0.0.12 / 2026-04-23

- `Instructions` stage
- Further refactoring of `rendering/mod.rs` and sub-modules

// 630K (644.272)

## 0.0.11 / 2026-04-23

- `Paused` stage with "flickering" Gems
- Extract rendering of shared areas into the `stage_handlers/mod.rs`

// 628K (642.728)

## 0.0.10 / 2026-04-22

- Show next column
- Update timings because of the next column
- Pile up columns until the `Gameover` stage
- Bump Rust version: 1.94.1 -> 1.95.0

// 627K (641.376)

## 0.0.9 / 2026-04-19

- Add terminal size checking and accompanying in-console user message
- Improve layout areas handling
- Optimize and improve keys legend mechanism
- `cargo update`

// 630K (644.232)

## 0.0.8 / 2026-04-12

- Delegated state pattern: stages with handlers and separate renders
- Keys legend at the bottom
- Add stages: `Ready`, `Gameplay` and `Game Over`

// 629K (643.128)

## 0.0.7 / 2026-04-11

- Respawn columns in a loop
- Pile up columns
- Detect piled blocks for a falling column
- Detect game over
- Log panic error to file

// 625K (639.392)

## 0.0.6 / 2026-04-08

- Create game board border "manually" (writting directly to terminal buffer)
- Add `fastrand` dependency
- Creating blocks in random colors
- Falling, manipulable column (handling user input), spawning at random x position

// 623K (637.496)

## 0.0.5 / 2026-04-06

- Basic MVC

// 614K (627.768)

## 0.0.4 / 2026-04-06

- Rearrange and clean up `app.rs` as a preparation for MVC
- Improve usage of "dev-console" feature
- Improve `dev_console.rs` by introducing `std::sync::mpsc` for log messages
- Make crate-wide available macros for colored logging to dev console
- Greatly improve `main.rs`: robustness, flexibility regarding non-essential conditions for app starting, error handling, panic protection, terminal restoration, logging
- Binary size optimizations:
  - opt-level = "z"

// 613K (627.624)

## 0.0.3 / 2026-04-05

- First draft of layout
- Add multicolored in-console logging, lines are scrollable using keyboard and mouse, toggleable via `[features]`
- Put in-file and in-console logging in a separate module
- Improve frame rate and responsiveness

// 622K (636.384)

## 0.0.2 / 2026-04-04

- Introduce the Main Buffer `stderr` messages and improve error handling
- Put log file in a more appropriate location: ~/.local/state/env!("CARGO_PKG_NAME")
- Check a minimum number of terminal columns and rows
- Refactor `main.rs` and `logger.rs`

// 611K (625.328)

## 0.0.1 / 2026-04-04

- Add lints rules
- Add basic `main.rs`/`app.rs` scaffolding and main loop
- Add logger
- Binary size optimizations:
  - Shave off unnecessary dependencies and/or their features
  - Replace `color-eyre` with`anyhow`

// 602K (615.592)

## 0.0.0 / 2025-05-02

- Binary size/performance optimization of a very basic Ratatui app

// 3.4M (built in Debug mode)
