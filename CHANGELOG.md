### 0.0.0 / 2025-05-02

- Binary size/performance optimization of a very basic Ratatui app

// 3.4M

### 0.0.1 / 2026-04-04

- Add lints rules
- Add basic `main`/`app` scaffolding and main loop
- Add logger
- Shave off unnecessary dependencies and/or their features
- Replace `color-eyre` with`anyhow`

// 602K (615.592)

### 0.0.2 / 2026-04-04

- Introduce the Main Buffer `stderr` messages and improve error handling
- Put log file in a more appropriate location: ~/.local/state/env!("CARGO_PKG_NAME")
- Check a minimum number of terminal columns and rows
- Refactor `main` and `logger`

// 611K (625.328)

### 0.0.3 / 2026-04-05

- First draft of layout
- Add multicolored in-console logging, lines are scrollable using keyboard and mouse, toggleable via `[features]`
- Put in-file and in-console logging in a separate module
- Improve frame rate and responsiveness

// 622K (636.384)
