## 1.1.12 / 2026-07-14

- Add support for macOS and Windows terminals
- Fix game board rendering in Widows "legacy" and macOS terminals
- Introduce and apply color palettes for macOS + other OSs
- Add recognition of terminals' Emoji support
- Render Instruction screen depending of terminal Emoji support
- Make creating / fetching data directory optional
- Add terminal message if there are issues with the data directory
- Bump Ratatui version: 0.30.1 -> 0.30.2
- Split `main.rs` into `data_dir.rs` and `terminal.rs`
- Add `LICENSE.md`

```bash
455 KiB (465,280 B)
```

## 0.1.11 / 2026-06-16

- Binary size optimizations: (-6,528 B)
  - Replace `HashSet` with a 128-bit scalar bitmask (`[u64; 2]`)
  - Replace `Vec` buffers with fixed-size stack arrays
  - Switch to native integer math (`usize`) to remove cast overhead
  - Remove implicit bounds-checking branches via slice iteration

```bash
453 KiB (463,376 B)
```

## 0.1.10 / 2026-06-09

- Adjust blink durations for matches and labels
- Isolate ticking for fading messages
- Drain any remaining events in the buffer before drawing
- Macroize `StageHandler` static dispatch delegation
- Binary size optimizations:
  - Use toolchain `nightly-2026-06-09` (-80 B)

```bash
459 KiB (469,904 B)
```

## 0.1.9 / 2026-06-09

- Blink matches
- Use toolchain `nightly-2026-06-08`

```bash
459 KiB (469,456 B)
```

## 0.1.8 / 2026-06-06

- Set terminal background color to black
- Decrease FPS from 62,5 to ~30 (it reduced CPU usage by nearly 3x)
- Improve and simplify clearing matches
- Improve and simplify labels' blink time calculation
- Eliminate `clone()` calls
- Remove unnecessary constants
- Reorganize top-most files' statements
- Improve `.cargo/config.nightly.toml`
- Bump Ratatui version: 0.30.0 -> 0.30.1

```bash
458 KiB (468,480 B)
```

## 0.1.7 / 2026-06-04

- Fix blinking labels after `Paused` stage
- Fix blinking labels glitch
- Adjust labels' `BLINK_DURATION`
- Adjust "Level up!" message fading out

```bash
459 KiB (469,536 B)
```

## 0.1.6 / 2026-06-03

- Blink labels
- Adjust messages' fading out
- Derive `Copy` and `Clone` on appropriate structs and enums (which hold "plain" values and are <= 8 bytes B)
- Rename `Game` to `GameState`
- Tidy up comments and remove unnecessary clippy allow annotation

```bash
459 KiB (469,504 B)
```

## 0.1.5 / 2026-06-01

- Improve messages' fading out
- Clean up TODOs and FIXMEs
- Bump Rust version: 1.95.0 -> 1.96.0
- Binary size optimizations:
  - Use toolchain `nightly-2026-06-01` (-1,712 B)

```bash
459 KiB (469,120 B)
```

## 0.1.4 / 2026-05-31

- Reorganize layout to show level and in-game messages
- In-game messages
- Make `dev_console` take string literals AND strings to be formatted

```bash
460 KiB (470,816 B)
```

## 0.1.3 / 2026-05-25

- Adjust leveling; refactor scoring
- Fix missing score updating when there are no hanging gems
- For falling columns, replace `Option` with `FallingColumnPlaceholder`
- Extract `get_game_state_after_matches_search` function
- Refactor

```bash
458 KiB (468,320 B)
```

## 0.1.2 / 2026-05-09

- Binary size optimizations:
  - Use linker garbage collection and ICF (Identical Code Folding): `-Clink-arg=-Wl,--gc-sections` and `-Clink-arg=-Wl,--icf=all` (-6,528 B)
  - Enable `-Zlocation-detail=none`  (-37,424 B)
  - Enable `-Zfmt-debug=none` (-9,920 B)
  - Reduce inline threshold for monomorphization control (`-Cllvm-args=--inline-threshold=45`) (-11,376 B)
  - Add `stderr_warning` utility function in `main.rs` (-224 B)

```bash
458 KiB (468,640 B)
```

## 0.1.1 / 2026-05-08

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

```bash
522 KiB (534,112 B)
```

## 0.1.0 / 2026-05-06    --> Fully playable game

- Persist and read high score from file
- Use common application state directory (for log and high score files)
- Improve/optimize and refactor

```bash
525 KiB (536,872 B)
```

## 0.0.15 / 2026-05-06

- Make clearing matches and "gravity" effect visually sequential

```bash
520 KiB (532,128 B)
```

## 0.0.14 / 2026-05-02

- Find and clear the matches
- Add scoring calculations
- Fix randomizing Gems in `Paused` stage
- Move listening to `F1` key to `ReadyHandler` only
- Adjust instructions text and improve layout

```bash
521 KiB (533,352 B)
```

## 0.0.13 / 2026-04-28

- Binary size optimizations:
  - Change `lto = true` to `lto = "fat"`
  - Add `features = ["release_max_level_off"]` to `log`
  - Replace `env_logger` with a simple custom logger
  - Remove `time`
  - Replace `anyhow` with "manual" error handling
  - Replace unnecessary `{:?}`/`{:#?}` with `{}`
  - Introduce `rust-toolchain.toml` with `channel = "nightly-2026-04-26"`

```bash
511 KiB (523,080 B)
```

## 0.0.12 / 2026-04-23

- `Instructions` stage
- Further refactoring of `rendering/mod.rs` and sub-modules

```bash
630 KiB (644,272 B)
```

## 0.0.11 / 2026-04-23

- `Paused` stage with "flickering" Gems
- Extract rendering of shared areas into the `stage_handlers/mod.rs`

```bash
628 KiB (642,728 B)
```

## 0.0.10 / 2026-04-22

- Show next column
- Update timings because of the next column
- Pile up columns until the `Gameover` stage
- Bump Rust version: 1.94.1 -> 1.95.0

```bash
627 KiB (641,376 B)
```

## 0.0.9 / 2026-04-19

- Add terminal size checking and accompanying in-console user message
- Improve layout areas handling
- Optimize and improve keys legend mechanism

```bash
630 KiB (644,232 B)
```

## 0.0.8 / 2026-04-12

- Delegated state pattern: stages with handlers and separate renders
- Keys legend at the bottom
- Add stages: `Ready`, `Gameplay` and `Game Over`

```bash
629 KiB (643,128 B)
```

## 0.0.7 / 2026-04-11

- Respawn columns in a loop
- Pile up columns
- Detect piled blocks for a falling column
- Detect game over
- Log panic error to file

```bash
625 KiB (639,392 B)
```

## 0.0.6 / 2026-04-08

- Create game board border "manually" (writting directly to terminal buffer)
- Add `fastrand` dependency
- Creating blocks in random colors
- Falling, manipulable column (handling user input), spawning at random x position

```bash
623 KiB (637,496 B)
```

## 0.0.5 / 2026-04-06

- Basic MVC

```bash
614 KiB (627,768 B)
```

## 0.0.4 / 2026-04-06

- Rearrange and clean up `app.rs` as a preparation for MVC
- Improve usage of "dev-console" feature
- Improve `dev_console.rs` by introducing `std::sync::mpsc` for log messages
- Make crate-wide available macros for colored logging to dev console
- Greatly improve `main.rs`: robustness, flexibility regarding non-essential conditions for app starting, error handling, panic protection, terminal restoration, logging
- Binary size optimizations:
  - opt-level = "z"

```bash
613 KiB (627,624 B)
```

## 0.0.3 / 2026-04-05

- First draft of layout
- Add multicolored in-console logging, lines are scrollable using keyboard and mouse, toggleable via `[features]`
- Put in-file and in-console logging in a separate module
- Improve frame rate and responsiveness

```bash
622 KiB (636,384 B)
```

## 0.0.2 / 2026-04-04

- Introduce the Main Buffer `stderr` messages and improve error handling
- Put log file in a more appropriate location: ~/.local/state/env!("CARGO_PKG_NAME")
- Check a minimum number of terminal columns and rows
- Refactor `main.rs` and `logger.rs`

```bash
611 KiB (625,328 B)
```

## 0.0.1 / 2026-04-04

- Add lints rules
- Add basic `main.rs`/`app.rs` scaffolding and main loop
- Add logger
- Binary size optimizations:
  - Shave off unnecessary dependencies and/or their features
  - Replace `color-eyre` with`anyhow`

```bash
602 KiB (615,592 B)
```

## 0.0.0 / 2025-05-02

- Create a basic Ratatui app

```bash
3,4 MiB   # built in Debug mode
```
