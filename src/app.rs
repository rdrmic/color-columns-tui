use std::time::{Duration, Instant};

use anyhow::Context;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::{
        self,
        event::{Event, KeyCode, KeyEvent, KeyEventKind},
    },
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

pub struct App {
    is_running: bool,
}

impl App {
    const TICK_RATE: Duration = Duration::from_millis(16);

    pub fn new() -> anyhow::Result<Self> {
        Ok(Self { is_running: true })
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> anyhow::Result<()> {
        log::info!("Main loop is starting...");

        let mut last_tick = Instant::now();

        while self.is_running {
            // 1. VIEW: Delegate all drawing to a UI module
            terminal
                .draw(|frame| {
                    self.draw(frame);
                })
                .context("Failed to draw to terminal")?;

            // 2. INPUT: Handle events
            // Event Handling - stays here or in input.rs?
            let event_waiting_time = Self::TICK_RATE.checked_sub(last_tick.elapsed()).unwrap_or(Duration::ZERO);
            if crossterm::event::poll(event_waiting_time)? {
                self.handle_events(&crossterm::event::read()?)?;
            }

            // 3. MODEL: Game logic updates (blocks falling, stats updating, ...)
            if last_tick.elapsed() >= Self::TICK_RATE {
                //game_state.update(); // Move piece down based on internal timer
                last_tick = Instant::now();
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
            self.quit();
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
        //self.game_state.handle_input(key);

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        #[cfg(feature = "dev-console")]
        let horizontal_constraints =
            [Constraint::Length(15), Constraint::Length(24), Constraint::Length(17), Constraint::Min(0)];
        #[cfg(not(feature = "dev-console"))]
        let horizontal_constraints = [Constraint::Length(15), Constraint::Length(24)];

        let main_layout =
            Layout::default().direction(Direction::Horizontal).constraints(horizontal_constraints).split(frame.area());

        let left_area = main_layout[0];
        let game_area = main_layout[1];

        // 2. Sub-split Left Area for "Next Piece" and "Stats"
        let left_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(16), Constraint::Length(20)])
            .split(left_area);
        self.draw_stats(frame, left_layout[1]);

        // 3. Sub-split Game Area for Notifications and Game Board
        let game_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(4), Constraint::Length(22)])
            .split(game_area);
        self.draw_game_board(frame, game_layout[1]);

        #[cfg(feature = "dev-console")]
        {
            use crate::logging;
            let dev_console_area = main_layout[3];
            logging::dev_console::draw(frame, dev_console_area);
        }
    }

    fn draw_stats(&self, frame: &mut Frame, area: Rect) {
        let stats_text = vec![
            Line::from(""),
            Line::from(vec!["SCORE".into()]).style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Line::from("329").style(Style::default().fg(Color::Gray)),
            Line::from(""),
            Line::from(vec!["MAX COMBO".into()]).style(Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
            Line::from("63").style(Style::default().fg(Color::Gray)),
            Line::from(""),
            Line::from(vec!["HIGHSCORE".into()]).style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Line::from("3495").style(Style::default().fg(Color::Gray)),
        ];

        let stats =
            Paragraph::new(stats_text).block(Block::default().padding(ratatui::widgets::Padding::horizontal(2)));

        frame.render_widget(stats, area);
    }

    fn draw_game_board(&self, frame: &mut Frame, area: Rect) {
        let board_block = Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Indexed(245))); // Gray border

        // We render a block to define the play area
        frame.render_widget(board_block, area);

        // Note: To render the colored blocks, you would iterate over your game grid
        // and render tiny 1x2 Rects or specialized widgets inside 'area'.
    }

    const fn quit(&mut self) {
        self.is_running = false;
    }
}
