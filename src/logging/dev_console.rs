#![cfg(feature = "dev-console")]

use std::{
    borrow::Cow,
    collections::VecDeque,
    sync::{
        LazyLock, Mutex, MutexGuard,
        mpsc::{self, Receiver, Sender},
    },
};

use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyEvent, MouseEvent, MouseEventKind},
    layout::Rect,
    style::Style,
    text::Line,
    widgets::{Block, Padding, Paragraph},
};

use crate::palette;

// =============================================================================
// Dev console
// =============================================================================
const MAX_CONSOLE_LOG_LINES: usize = 1024;

struct DevConsoleState {
    pub lines: VecDeque<Line<'static>>,
    pub auto_scroll: bool,
    pub scroll_offset: u16,
    pub last_known_inner_height: u16,
}

static DEV_CONSOLE: LazyLock<Mutex<DevConsoleState>> = LazyLock::new(|| {
    Mutex::new(DevConsoleState { lines: VecDeque::with_capacity(MAX_CONSOLE_LOG_LINES), auto_scroll: true, scroll_offset: 0, last_known_inner_height: 0 })
});

pub fn handle_key_pressed_event(key_event: &KeyEvent) {
    let mut console = acquire_console_mutex();
    match key_event.code {
        KeyCode::PageUp => handle_scrolling_up(&mut console),
        KeyCode::PageDown => handle_scrolling_down(&mut console),
        KeyCode::Home => {
            console.auto_scroll = false;
            console.scroll_offset = 0;
        }
        KeyCode::End => console.auto_scroll = true,
        _ => {
            let max_scroll_possible = calculate_max_scroll_possible(&console);
            if console.scroll_offset >= max_scroll_possible {
                console.auto_scroll = true;
            }
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

        let block = Block::bordered()
            .title(format!(" Navigate history (last {MAX_CONSOLE_LOG_LINES} lines): <Page Up/Down> / <Home> (oldest line in buffer) / <End> (most recent line; resets auto-scroll) "))
            .style(Style::from(palette::DEV_CONSOLE_BORDER))
            .padding(Padding::horizontal(1));

        // Save this for the input handler to use later
        console.last_known_inner_height = block.inner(area).height;

        let max_scroll_possible = calculate_max_scroll_possible(&console);

        console.scroll_offset = if console.auto_scroll { max_scroll_possible } else { console.scroll_offset.min(max_scroll_possible) };

        let lines_vec = console.lines.iter().cloned().collect::<Vec<_>>();
        Paragraph::new(lines_vec).block(block).scroll((console.scroll_offset, 0))
    };

    frame.render_widget(paragraph, area);
}

fn acquire_console_mutex() -> MutexGuard<'static, DevConsoleState> {
    DEV_CONSOLE.lock().expect("Acquiring Mutex failed")
}

/// Calculate the actual bottom.
fn calculate_max_scroll_possible(console: &DevConsoleState) -> u16 {
    let log_len = console.lines.len() as u16;
    log_len.saturating_sub(console.last_known_inner_height)
}

const fn handle_scrolling_up(console: &mut DevConsoleState) {
    console.auto_scroll = false;
    console.scroll_offset = console.scroll_offset.saturating_sub(1);
}

fn handle_scrolling_down(console: &mut DevConsoleState) {
    let max_scroll_possible = calculate_max_scroll_possible(console);
    console.scroll_offset = console.scroll_offset.saturating_add(1);

    if console.scroll_offset >= max_scroll_possible {
        console.auto_scroll = true;
    }
}

// =============================================================================
// Channeling log messages
// =============================================================================
struct LogMessage {
    msg: Cow<'static, str>,
    color: PrintColor,
}

/// Channel to decouple log messages' sending from processing.
static LOG_CHANNEL: LazyLock<(Sender<LogMessage>, Mutex<Receiver<LogMessage>>)> = LazyLock::new(|| {
    let (tx, rx) = mpsc::channel();
    (tx, Mutex::new(rx))
});

/// Sends message to the log channel (used by `dev_*!` macros in mod.rs).
#[allow(dead_code)]
pub fn send_log_message(msg: Cow<'static, str>, color: PrintColor) {
    let (tx, _) = &*LOG_CHANNEL;
    let _ = tx.send(LogMessage { msg, color });
}

/// Moves all pending messages (as styled Lines) from the channel into the console state
fn flush_messages_as_lines(console: &mut DevConsoleState) {
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

// =============================================================================
// Print colors
// =============================================================================
#[derive(Copy, Clone)]
pub enum PrintColor {
    Gray,
    Cyan,
    Yellow,
    Red,
}

impl From<PrintColor> for Style {
    fn from(color: PrintColor) -> Self {
        match color {
            PrintColor::Gray => Self::new().fg(palette::DEV_CONSOLE_GRAY),
            PrintColor::Cyan => Self::new().fg(palette::DEV_CONSOLE_CYAN),
            PrintColor::Yellow => Self::new().fg(palette::DEV_CONSOLE_YELLOW),
            PrintColor::Red => Self::new().fg(palette::DEV_CONSOLE_RED),
        }
    }
}
