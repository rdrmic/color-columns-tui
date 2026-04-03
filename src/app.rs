use std::time::{Duration, Instant};

use anyhow::Context;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::Color,
    symbols::border,
    widgets::{Block, Paragraph},
};

pub struct App {
    is_running: bool,
}

impl App {
    const EVENT_LISTEN_RATE: Duration = Duration::from_millis(30);
    const TICK_RATE: Duration = Duration::from_millis(1000);

    pub fn new() -> anyhow::Result<Self> {
        Ok(Self { is_running: true })
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> anyhow::Result<()> {
        terminal.clear()?;

        let mut next_tick = Instant::now() + Self::TICK_RATE;

        log::info!("Main loop is starting...");
        while self.is_running {
            let is_user_input = if event::poll(Self::EVENT_LISTEN_RATE)? {
                self.handle_events()?;
                true
            } else {
                false
            };

            if is_user_input || Instant::now() >= next_tick {
                self.refresh(&mut terminal)?;
                next_tick = Instant::now() + Self::TICK_RATE;
            }
        }

        Ok(())
    }

    fn handle_events(&mut self) -> anyhow::Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.handle_key_event(key)?,
            _ => {}
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> anyhow::Result<()> {
        if let KeyCode::Char('q' | 'Q') = key.code {
            self.quit()
        }

        Ok(())
    }

    // TODO refactor? tick handling is removed..
    fn refresh(&mut self, terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
        terminal
            .draw(|frame| {
                self.draw(frame);
            })
            .context("Failed to draw to terminal")?;

        Ok(())
    }

    // TODO optimize (only on changed states)?
    fn draw(&mut self, frame: &mut Frame) {
        let x = 3;
        let y = 1;
        let width = frame.area().width - 6;
        let height = frame.area().height - 1;

        let app_area = Rect::new(x, y, width, height);

        let vertical_layout =
            Layout::vertical([Constraint::Percentage(100), Constraint::Length(2)]).margin(0).spacing(0);
        let [main_area, footer_area] = vertical_layout.areas(app_area);

        // border
        Self::draw_border(frame, main_area);

        // footer
        Self::draw_footer(frame, footer_area);
    }

    fn draw_border(frame: &mut Frame, area: Rect) {
        let contents_outer_area = Rect { x: area.x - 1, y: area.y, width: area.width + 2, height: area.height };
        let contents_block = Block::bordered().border_set(border::ONE_EIGHTH_TALL).border_style(Color::Green);
        frame.render_widget(contents_block, contents_outer_area);
    }

    fn draw_footer(frame: &mut Frame, area: Rect) {
        let footer = Paragraph::new("Press <q> to quit").centered();
        frame.render_widget(footer, area);
    }

    const fn quit(&mut self) {
        self.is_running = false;
    }
}
