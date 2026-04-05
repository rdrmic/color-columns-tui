use std::time::{Duration, Instant};

use anyhow::Context;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::{
        self,
        event::{self, Event, KeyCode, KeyEvent, MouseEventKind},
    },
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

use crate::logging;

pub struct App {
    is_running: bool,
    dev_console: logging::dev_console::DevConsole,
}

impl App {
    const EVENT_LISTEN_RATE: Duration = Duration::from_secs(0);
    const TICK_RATE: Duration = Duration::from_millis(16);

    pub fn new() -> anyhow::Result<Self> {
        Ok(Self { is_running: true, dev_console: logging::dev_console::DevConsole::default() })
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> anyhow::Result<()> {
        let mut last_tick = Instant::now();

        log::info!("Main loop is starting...");
        while self.is_running {
            // 1. Draw the UI
            terminal
                .draw(|frame| {
                    self.draw(frame);
                })
                .context("Failed to draw to terminal")?;

            // 2. Wait for input OR a tick timeout
            let timeout = Self::TICK_RATE.checked_sub(last_tick.elapsed()).unwrap_or(Self::EVENT_LISTEN_RATE);

            if crossterm::event::poll(timeout)? {
                match event::read()? {
                    Event::Key(key_event) if key_event.kind == event::KeyEventKind::Press => {
                        self.handle_key_pressed(key_event)?;
                    }
                    Event::Mouse(mouse_event)
                        if matches!(mouse_event.kind, MouseEventKind::ScrollUp | MouseEventKind::ScrollDown) =>
                    {
                        self.dev_console.handle_mouse_scroll(mouse_event);
                    }
                    _ => {}
                }
            }

            // 3. Logic Update (Gravity/Falling)
            if last_tick.elapsed() >= Self::TICK_RATE {
                //game_state.update(); // Move piece down based on internal timer
                last_tick = Instant::now();
            }
        }

        Ok(())
    }

    pub fn handle_key_pressed(&mut self, key: KeyEvent) -> anyhow::Result<()> {
        if let KeyCode::Char('q' | 'Q') = key.code {
            self.quit();
        }
        self.dev_console.handle_key(key);

        Ok(())
    }

    // TODO optimize - draw only on changed states?
    fn draw(&mut self, frame: &mut Frame) {
        let constraints = if cfg!(feature = "dev-console") {
            [Constraint::Length(15), Constraint::Length(24), Constraint::Percentage(100)]
        } else {
            [Constraint::Length(15), Constraint::Length(24), Constraint::Length(0)]
        };

        // 1. Split screen into Left (Stats) and Right (Game Board)
        let main_layout =
            Layout::default().direction(Direction::Horizontal).constraints(constraints).split(frame.area());

        let left_area = main_layout[0];
        let game_area = main_layout[1];
        let dev_log_area = main_layout[2];

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

        self.dev_console.draw(frame, dev_log_area);
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
        // Create the bounding box for the columns
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
