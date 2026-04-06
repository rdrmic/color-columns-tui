#![allow(unused)]
#![cfg(feature = "dev-console")]
use std::collections::VecDeque;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{LazyLock, Mutex, MutexGuard};

use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyEvent, MouseEvent, MouseEventKind},
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Padding, Paragraph},
};

// ============================================================================
// Dev Console
// ============================================================================
const MAX_CONSOLE_LOG_LINES: usize = 1024;

struct DevConsole {
    pub lines: VecDeque<Line<'static>>,
    pub auto_scroll: bool,
    pub scroll_offset: u16,
    pub last_known_inner_height: u16,
}

static DEV_CONSOLE: LazyLock<Mutex<DevConsole>> = LazyLock::new(|| {
    Mutex::new(DevConsole {
        lines: VecDeque::with_capacity(MAX_CONSOLE_LOG_LINES),
        auto_scroll: true,
        scroll_offset: 0,
        last_known_inner_height: 0,
    })
});

pub fn handle_key_pressed_event(key_event: &KeyEvent) -> bool {
    let mut console = acquire_console_mutex();
    match key_event.code {
        KeyCode::PageUp => {
            handle_scrolling_up(&mut console);
            true
        }
        KeyCode::PageDown => {
            handle_scrolling_down(&mut console);
            true
        }
        KeyCode::Home => {
            console.auto_scroll = false;
            console.scroll_offset = 0;
            true
        }
        KeyCode::End => {
            console.auto_scroll = true;
            true
        }
        _ => {
            let max_scroll_possible = calculate_max_scroll_possible(&console);
            if console.scroll_offset >= max_scroll_possible {
                console.auto_scroll = true;
            }
            false
        }
    }
}

pub fn handle_mouse_scroll_event(mouse_event: MouseEvent) {
    let mut console = acquire_console_mutex();
    match mouse_event.kind {
        MouseEventKind::ScrollUp => handle_scrolling_up(&mut console),
        MouseEventKind::ScrollDown => handle_scrolling_down(&mut console),
        _ => {}
    }
}

pub fn draw(frame: &mut Frame, area: Rect) {
    let paragraph = {
        let mut console = acquire_console_mutex();

        // Process any logs queued by macros before drawing
        flush_messages_as_lines(&mut console);

        let block = Block::default()
            .borders(Borders::ALL)
            // " Navigate history ({MAX_CONSOLE_LOG_LINES} lines): <Page Up/Down> / <Home> / <End> (reset auto-scroll) "
            .title(format!(" Navigate history ({MAX_CONSOLE_LOG_LINES} lines): <Page Up/Down> / <Home> (oldest line in buffer) / <End> (most recent line, it re-enables auto-scroll) "))
            .padding(Padding::horizontal(1));

        // Save this for the input handler to use later
        console.last_known_inner_height = block.inner(area).height;

        let max_scroll_possible = calculate_max_scroll_possible(&console);

        console.scroll_offset =
            if console.auto_scroll { max_scroll_possible } else { console.scroll_offset.min(max_scroll_possible) };

        let lines_vec = console.lines.iter().cloned().collect::<Vec<_>>();
        Paragraph::new(lines_vec).block(block).scroll((console.scroll_offset, 0))
    };

    frame.render_widget(paragraph, area);
}

fn acquire_console_mutex() -> MutexGuard<'static, DevConsole> {
    DEV_CONSOLE.lock().expect("Acquiring Mutex failed")
}

/// Calculate the actual bottom
// TODO verify and rename
fn calculate_max_scroll_possible(console: &DevConsole) -> u16 {
    let log_len = console.lines.len() as u16;
    log_len.saturating_sub(console.last_known_inner_height)
}

const fn handle_scrolling_up(console: &mut DevConsole) {
    console.auto_scroll = false;
    console.scroll_offset = console.scroll_offset.saturating_sub(1);
}

fn handle_scrolling_down(console: &mut DevConsole) {
    let max_scroll_possible = calculate_max_scroll_possible(console);
    console.scroll_offset = console.scroll_offset.saturating_add(1);

    if console.scroll_offset >= max_scroll_possible {
        console.auto_scroll = true;
    }
}

// ============================================================================
// Channeling Log Messages
// ============================================================================
struct LogMessage {
    msg: String,
    color: PrintColor,
}

/// Channel to decouple log messages sending and processing
static LOG_CHANNEL: LazyLock<(Sender<LogMessage>, Mutex<Receiver<LogMessage>>)> = LazyLock::new(|| {
    let (tx, rx) = mpsc::channel();
    (tx, Mutex::new(rx))
});

/// Sends message to the log channel (used by `dev_*!` macros in mod.rs)
pub fn send_log_message(msg: String, color: PrintColor) {
    let (tx, _) = &*LOG_CHANNEL;
    let _ = tx.send(LogMessage { msg, color });
}

/// Moves all pending messages (as styled Lines) from the channel into the console struct
fn flush_messages_as_lines(console: &mut DevConsole) {
    let (_, rx_lock) = &*LOG_CHANNEL;
    if let Ok(rx) = rx_lock.lock() {
        while let Ok(log) = rx.try_recv() {
            if console.lines.len() >= MAX_CONSOLE_LOG_LINES {
                console.lines.pop_front();
            }
            console.lines.push_back(Line::styled(log.msg, Style::from(log.color)));
        }
    }
}

// ============================================================================
// Print Colors
// ============================================================================
const STYLE_COLOR_GRAY: Style = Style::new().fg(Color::Gray);
const STYLE_COLOR_CYAN: Style = Style::new().fg(Color::Cyan);
const STYLE_COLOR_YELLOW: Style = Style::new().fg(Color::Yellow);
const STYLE_COLOR_RED: Style = Style::new().fg(Color::Red);

pub enum PrintColor {
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
