#![cfg(feature = "dev-console")]
use std::collections::VecDeque;

use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyEvent, MouseEvent, MouseEventKind},
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Padding, Paragraph},
};

// ============================================================================
// Print Colors
// ============================================================================
const STYLE_COLOR_GRAY: Style = Style::new().fg(Color::Gray);
const STYLE_COLOR_CYAN: Style = Style::new().fg(Color::Cyan);
const STYLE_COLOR_YELLOW: Style = Style::new().fg(Color::Yellow);
const STYLE_COLOR_RED: Style = Style::new().fg(Color::Red);

pub(super) enum PrintColor {
    Gray,
    Cyan,
    Yellow,
    Red,
}

impl From<PrintColor> for Style {
    fn from(color: PrintColor) -> Self {
        match color {
            PrintColor::Gray => STYLE_COLOR_GRAY,
            PrintColor::Cyan => STYLE_COLOR_CYAN,
            PrintColor::Yellow => STYLE_COLOR_YELLOW,
            PrintColor::Red => STYLE_COLOR_RED,
        }
    }
}

// ============================================================================
// Dev Console
// ============================================================================
const MAX_CONSOLE_LOG_LINES: usize = 1024;

pub struct DevConsole {
    pub lines: VecDeque<Line<'static>>,
    pub auto_scroll: bool,
    pub scroll_offset: u16,
    pub last_known_inner_height: u16,
}

impl Default for DevConsole {
    fn default() -> Self {
        Self {
            lines: VecDeque::with_capacity(MAX_CONSOLE_LOG_LINES),
            auto_scroll: true,
            scroll_offset: 0,
            last_known_inner_height: 0,
        }
    }
}

impl DevConsole {
    #[allow(unused)]
    pub fn gray(&mut self, msg: String) {
        self.append_line(msg, PrintColor::Gray.into());
    }

    #[allow(unused)]
    pub fn cyan(&mut self, msg: String) {
        self.append_line(msg, PrintColor::Cyan.into());
    }

    #[allow(unused)]
    pub fn yellow(&mut self, msg: String) {
        self.append_line(msg, PrintColor::Yellow.into());
    }

    #[allow(unused)]
    pub fn red(&mut self, msg: String) {
        self.append_line(msg, PrintColor::Red.into());
    }

    fn append_line(&mut self, msg: String, style: Style) {
        if self.lines.len() >= MAX_CONSOLE_LOG_LINES {
            self.lines.pop_front();
        }
        self.lines.push_back(Line::styled(msg, style));
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Home => {
                self.auto_scroll = false;
                self.scroll_offset = 0;
            }
            KeyCode::PageUp => self.handle_scrolling_up(),
            KeyCode::PageDown => self.handle_scrolling_down(),
            KeyCode::End => self.auto_scroll = true,
            _ => {}
        }
    }

    pub fn handle_mouse_scroll(&mut self, mouse: MouseEvent) {
        match mouse.kind {
            MouseEventKind::ScrollUp => self.handle_scrolling_up(),
            MouseEventKind::ScrollDown => self.handle_scrolling_down(),
            _ => {}
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, mut area: Rect) {
        area.x += 17;

        let block = Block::default()
            .borders(Borders::ALL)
            .title(format!(" Navigate the history ({MAX_CONSOLE_LOG_LINES} lines) using: <Page Up> / <Page Down> / <Home> (oldest line in buffer) / <End> (most recent line, it re-enables automatic scrolling) "))
            .padding(Padding::horizontal(1));

        // Save this for the input handler to use later
        self.last_known_inner_height = block.inner(area).height;

        let max_scroll_possible = self.calculate_max_scroll_possible();

        self.scroll_offset =
            if self.auto_scroll { max_scroll_possible } else { self.scroll_offset.min(max_scroll_possible) };

        let lines_vec = self.lines.iter().cloned().collect::<Vec<_>>();
        let paragraph = Paragraph::new(lines_vec).block(block).scroll((self.scroll_offset, 0));

        frame.render_widget(paragraph, area);
    }

    /// Calculate the actual bottom
    fn calculate_max_scroll_possible(&self) -> u16 {
        let log_len = self.lines.len() as u16;
        log_len.saturating_sub(self.last_known_inner_height)
    }

    const fn handle_scrolling_up(&mut self) {
        self.auto_scroll = false;
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

    fn handle_scrolling_down(&mut self) {
        let max_scroll_possible = self.calculate_max_scroll_possible();

        if self.scroll_offset < max_scroll_possible {
            self.scroll_offset += 1;
        }
        self.auto_scroll = self.scroll_offset >= max_scroll_possible;
    }
}
