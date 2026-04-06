use std::time::{Duration, Instant};

use anyhow::Context;
use ratatui::{
    DefaultTerminal,
    crossterm::{
        self,
        event::{Event, KeyCode, KeyEvent, KeyEventKind},
    },
};

use crate::{game::GameState, renderer};

pub struct App {
    is_running: bool,
    game_state: GameState,
}

impl App {
    const RENDER_TICK_RATE: Duration = Duration::from_millis(16);

    pub fn new() -> anyhow::Result<Self> {
        Ok(Self { is_running: true, game_state: GameState::new() })
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> anyhow::Result<()> {
        log::info!("Main loop is starting...");

        // This tracks the last time the block moved down (gravity)
        let mut last_logic_tick = Instant::now();

        while self.is_running {
            terminal
                .draw(|frame| {
                    renderer::render(frame, &self.game_state);
                })
                .context("Failed to draw to terminal")?;

            // 1. Calculate time until the NEXT intended logic update
            let time_to_next_logic =
                self.game_state.tick_rate().checked_sub(last_logic_tick.elapsed()).unwrap_or(Duration::ZERO);

            // 3. The poll timeout is the MINIMUM of our frame rate and logic rate
            // This ensures we wake up exactly when the block needs to fall
            let event_waiting_time = Self::RENDER_TICK_RATE.min(time_to_next_logic);

            // 4. INPUT: Handle events
            // Event Handling - stays here or in input.rs?
            //let event_waiting_time = Self::TICK_RATE.checked_sub(last_tick.elapsed()).unwrap_or(Duration::ZERO);
            if crossterm::event::poll(event_waiting_time)? {
                self.handle_events(&crossterm::event::read()?)?;
            }

            // 5. MODEL: Game logic updates (blocks falling, stats updating, ...)
            // Use 'while' instead of 'if' to catch up if the computer hitched
            while last_logic_tick.elapsed() >= self.game_state.tick_rate() {
                self.game_state.tick();
                last_logic_tick += self.game_state.tick_rate();
            }
        }

        Ok(())
    }

    fn handle_events(&mut self, event: &Event) -> anyhow::Result<()> {
        match event {
            // 1. Keyboard Events
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self.handle_key_pressed_event(key_event),

            // 2. Mouse Events for DevConsole scrolling
            #[cfg(feature = "dev-console")]
            Event::Mouse(mouse_event)
                if matches!(
                    mouse_event.kind,
                    crossterm::event::MouseEventKind::ScrollUp | crossterm::event::MouseEventKind::ScrollDown
                ) =>
            {
                use crate::logging;
                logging::dev_console::handle_mouse_scroll_event(*mouse_event);
                Ok(())
            }

            _ => Ok(()),
        }
    }

    fn handle_key_pressed_event(&mut self, key_event: &KeyEvent) -> anyhow::Result<()> {
        // --- PRIORITY 1: GLOBAL SYSTEM KEYS ---
        // TODO: Add other system keys here (e.g., Pause, ...) or they should be handled by the game's key handler?
        if let KeyCode::Char('q' | 'Q') = key_event.code {
            self.is_running = false;
            return Ok(());
        }

        // --- PRIORITY 2: DEV CONSOLE KEYS ---
        #[cfg(feature = "dev-console")]
        {
            use crate::logging;
            if logging::dev_console::handle_key_pressed_event(key_event) {
                return Ok(());
            }
        }

        // --- PRIORITY 3: GAME LOGIC KEYS ---
        // Pass the key to the game state (moving blocks, etc.)
        match key_event.code {
            KeyCode::Left => self.game_state.move_left(),
            KeyCode::Right => self.game_state.move_right(),
            _ => {}
        }

        Ok(())
    }
}
