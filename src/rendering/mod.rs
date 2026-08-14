use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
};

#[cfg(feature = "dev-console")]
use crate::logging;
use crate::{
    blocks::{Gem, GemBlock},
    game_state::{BOARD_HEIGHT, BOARD_WIDTH, GameState},
    palette,
    stage_handlers::Stage,
};

mod buffer_helpers;
mod instructions;

#[cfg(feature = "dev-console")]
pub const MIN_WINDOW_WIDTH: u16 = 176;
#[cfg(not(feature = "dev-console"))]
pub const MIN_WINDOW_WIDTH: u16 = 29;
pub const MIN_WINDOW_HEIGHT: u16 = 27;

// =============================================================================
// Static styles
// =============================================================================
#[rustfmt::skip]
mod styles {
    use super::palette;
    use ratatui::style::Style;

    pub(super) const DEFAULT_BG_AND_FG_COLORS: Style = Style::new().fg(palette::UI_CANVAS_FG).bg(palette::UI_CANVAS_BG);
    pub(super) const FG_RESET_COLOR: Style           = Style::new().fg(palette::UI_CANVAS_BG);

    pub(super) const LEVEL: Style                    = Style::new().fg(palette::STAT_LEVEL).bold().italic();
    pub(super) const STATS_VALUES: Style             = Style::new().fg(palette::STATS_VALUE);

    #[cfg(not(target_os = "macos"))]
    pub(super) const GAME_BORDER: Style              = Style::new().fg(palette::UI_GAME_BORDER);
    #[cfg(target_os = "macos")]
    pub(super) const GAME_BORDER: Style              = Style::new().bg(palette::UI_GAME_BORDER);

    pub(super) const LEGEND_BORDER: Style            = Style::new().fg(palette::UI_LEGEND_BORDER);
    pub(super) const LEGEND_KEY: Style               = Style::new().fg(palette::UI_LEGEND_KEY).bold();
    pub(super) const LEGEND_ACTION: Style            = Style::new().fg(palette::UI_LEGEND_ACTION);

    pub(super) const CONSOLE_TEXT_INFO: Style        = Style::new().fg(palette::CONSOLE_TEXT_INFO);
    pub(super) const CONSOLE_TEXT_ERROR: Style       = Style::new().fg(palette::CONSOLE_TEXT_ERROR);
}

// =============================================================================
// Rendering entry point
// =============================================================================
pub fn render(frame: &mut Frame, stage: &Stage, game: &GameState) {
    let frame_area = frame.area();

    set_default_bg_and_fg_colors(frame, frame_area);

    if is_terminal_window_too_small(frame_area) {
        render_message_terminal_window_too_small(frame, frame_area);
        return;
    }

    let layout_areas = get_layout_areas(frame_area);

    if matches!(stage, Stage::Instructions(_)) {
        instructions::draw_instructions(frame, layout_areas.instructions);
    } else {
        draw_shared_areas(frame, &layout_areas, game, stage);
    }
    draw_footer(frame, layout_areas.keys_legend, stage);

    #[cfg(feature = "dev-console")]
    logging::dev_console::draw(frame, layout_areas.dev_console);
}

fn set_default_bg_and_fg_colors(frame: &mut Frame, frame_area: Rect) {
    frame.buffer_mut().set_style(frame_area, styles::DEFAULT_BG_AND_FG_COLORS);
}

fn draw_shared_areas(frame: &mut Frame, layout_areas: &LayoutAreas, game: &GameState, stage: &Stage) {
    draw_level(frame, layout_areas.level, game, stage);
    draw_message(frame, layout_areas.message, game);
    draw_next_column(frame, layout_areas.next_column, game, stage);
    draw_stats(frame, layout_areas.stats, game, stage);
    draw_board(frame, layout_areas.board, game, stage);
}

fn draw_footer(frame: &mut Frame, area: Rect, stage: &Stage) {
    draw_keys_legend(frame, area, stage);
}

// =============================================================================
// Terminal window size check
// =============================================================================
const fn is_terminal_window_too_small(area: Rect) -> bool {
    area.width < MIN_WINDOW_WIDTH || area.height < MIN_WINDOW_HEIGHT
}

#[rustfmt::skip]
fn render_message_terminal_window_too_small(frame: &mut Frame, area: Rect) {
    let buf = frame.buffer_mut();
    let mut y = area.y + 1;

    y = buffer_helpers::draw_string_centered_wrapped(buf, area, y, "Terminal window too small!", styles::CONSOLE_TEXT_ERROR);
    y += 1;
    y = buffer_helpers::draw_dimensions_centered(
        buf,
        area,
        y,
        "Required: ",
        u32::from(MIN_WINDOW_WIDTH),
        u32::from(MIN_WINDOW_HEIGHT),
        styles::CONSOLE_TEXT_INFO,
    );
    y = buffer_helpers::draw_dimensions_centered(
        buf,
        area,
        y,
        "Current:  ",
        u32::from(area.width),
        u32::from(area.height),
        styles::CONSOLE_TEXT_ERROR,
    );
    y += 1;
    buffer_helpers::draw_string_centered_wrapped(buf, area, y, "Please resize the window to play.", styles::CONSOLE_TEXT_INFO);
}

// =============================================================================
// Layout areas
// =============================================================================
struct LayoutAreas {
    level: Rect,
    message: Rect,
    next_column: Rect,
    stats: Rect,
    board: Rect,
    keys_legend: Rect,
    instructions: Rect,
    #[cfg(feature = "dev-console")]
    dev_console: Rect,
}

#[rustfmt::skip]
const fn get_layout_areas(frame_area: Rect) -> LayoutAreas {
    let entire_area =        Rect { x: frame_area.x,         y: frame_area.y,           width: 29,                       height: frame_area.height };
    let entire_area_padded = Rect { x: entire_area.x + 1,    y: entire_area.y + 1,      width: entire_area.width - 2,    height: entire_area.height - 2 };
    let entire_game_area =   Rect { x: entire_area_padded.x, y: entire_area_padded.y,   width: entire_area_padded.width, height: 18 };
    let game_area =          Rect { x: entire_game_area.x,   y: entire_game_area.y + 3, width: entire_game_area.width,   height: entire_game_area.height - 3 };
    let left_side_area =     Rect { x: game_area.x,          y: game_area.y,            width: 11,                       height: game_area.height };

    LayoutAreas {
        level:        Rect { x: entire_game_area.x + 1,   y: entire_game_area.y,        width: entire_game_area.width,   height: 1 },
        message:      Rect { x: entire_game_area.x - 1,   y: entire_game_area.y + 1,    width: entire_game_area.width,   height: 2 },
        next_column:  Rect { x: left_side_area.x,         y: left_side_area.y + 1,      width: left_side_area.width,     height: 3 },
        stats:        Rect { x: left_side_area.x + 1,     y: left_side_area.y + 5,      width: left_side_area.width,     height: left_side_area.height - 4 },
        board:        Rect { x: game_area.x + 13,         y: game_area.y,               width: game_area.width - 13,     height: game_area.height },
        keys_legend:  Rect { x: entire_area_padded.x,     y: entire_area_padded.y + 19, width: entire_area_padded.width, height: entire_area_padded.height - 19 },
        instructions: entire_area,
        #[cfg(feature = "dev-console")]
        dev_console:  Rect { x: entire_area.right() + 13, y: frame_area.y,              width: frame_area.width - 42,    height: frame_area.height }
    }
}

// =============================================================================
// Level and user messages
// =============================================================================
fn draw_level(frame: &mut Frame, area: Rect, game: &GameState, stage: &Stage) {
    let style = match stage {
        Stage::Gameplay(gameplay_handler) if !gameplay_handler.blinking_labels().is_level_visible() => styles::FG_RESET_COLOR,
        _ => styles::LEVEL,
    };

    let buf = frame.buffer_mut();
    buf.set_string(area.x, area.y, "LEVEL ", style);
    buffer_helpers::draw_u32(buf, area.x + 6, area.y, game.scoring().level(), style);
}

pub fn draw_message(frame: &mut Frame, area: Rect, game: &GameState) {
    let Some(msg) = game.message() else {
        return;
    };
    if let Some(blinking) = msg.blinking()
        && !blinking.is_visible_phase()
    {
        return;
    }

    let buf = frame.buffer_mut();
    buffer_helpers::draw_string_right_aligned(buf, area, msg.text(), Style::from((msg.color(), Modifier::BOLD | Modifier::ITALIC)));
}

// =============================================================================
// Left side (next column and stats) and game board
// =============================================================================
fn draw_next_column(frame: &mut Frame, area: Rect, game: &GameState, stage: &Stage) {
    let right_aligned_area = Rect { x: area.right() - 2, y: area.y, width: 2, height: area.height };

    if let Stage::Paused(pause_handler) = stage {
        // Next column with random colors
        let flicker_tick = pause_handler.flicker_tick();
        for gem_block in game.get_next_column().gem_blocks() {
            let seed = seed_for_randomizing_next_column_blocks(flicker_tick, gem_block.x, gem_block.y);
            let flickered_gem = Gem::random_for_pause(seed);
            frame.render_widget(GemBlock::new(0, gem_block.y, flickered_gem), right_aligned_area);
        }
    } else {
        frame.render_widget(game.get_next_column(), right_aligned_area);
    }
}

#[rustfmt::skip]
fn draw_stats(frame: &mut Frame, mut area: Rect, game: &GameState, stage: &Stage) {
    let (max_combo_label_color, highscore_label_color) = match stage {
        Stage::Gameplay(handler) => (
            if handler.blinking_labels().is_max_combo_visible() { palette::STAT_LABEL_MAX_COMBO } else { palette::UI_CANVAS_BG },
            if handler.blinking_labels().is_highscore_visible() { palette::STAT_LABEL_HIGHSCORE } else { palette::UI_CANVAS_BG },
        ),
        _ => (palette::STAT_LABEL_MAX_COMBO, palette::STAT_LABEL_HIGHSCORE),
    };

    let stats = [
        ("SCORE",     game.scoring().score(),     palette::STAT_LABEL_SCORE),
        ("MAX COMBO", game.scoring().max_combo(), max_combo_label_color),
        ("HIGHSCORE", game.scoring().highscore(), highscore_label_color),
    ];

    let buf = frame.buffer_mut();
    for &(label, value, color) in &stats {
        area.y += 1;

        buf.set_string(area.x, area.y, label, Style::from((color, Modifier::BOLD)));
        area.y += 1;

        buffer_helpers::draw_u32(buf, area.x, area.y, value, styles::STATS_VALUES);
        area.y += 1;
    }
}

fn draw_board(frame: &mut Frame, area: Rect, game: &GameState, stage: &Stage) {
    draw_board_border(frame, area);

    let board_inner_area = Rect { x: area.x + 1, y: area.y + 1, width: area.width - 2, height: area.height - 2 };

    if let Stage::Paused(pause_handler) = stage {
        let flicker_tick = pause_handler.flicker_tick();

        // Falling column with random colors
        for gem_block in game.get_falling_column().gem_blocks() {
            if gem_block.y >= 0 {
                let seed = seed_for_randomizing_falling_column_blocks(flicker_tick, gem_block.x, gem_block.y);
                let flickered_gem = Gem::random_for_pause(seed);
                frame.render_widget(GemBlock::new(gem_block.x, gem_block.y, flickered_gem), board_inner_area);
            }
        }

        // Pile with random colors
        for y in 0..BOARD_HEIGHT {
            #[allow(clippy::cast_possible_wrap)]
            let y_i8 = y as i8;
            for x in 0..BOARD_WIDTH {
                if game.get_pile().get(x, y).is_some() {
                    let seed = seed_for_randomizing_pile_blocks(flicker_tick, x, y);
                    let flickered_gem = Gem::random_for_pause(seed);
                    frame.render_widget(GemBlock::new(x, y_i8, flickered_gem), board_inner_area);
                }
            }
        }
    } else {
        frame.render_widget(game.get_falling_column(), board_inner_area);
        frame.render_widget(game.get_pile(), board_inner_area);
    }
}

fn draw_board_border(frame: &mut Frame, area: Rect) {
    let buf = frame.buffer_mut();

    let left = area.left();
    let right = area.right() - 1;
    let top = area.top();
    let bottom = area.bottom() - 1;

    #[cfg(not(target_os = "macos"))]
    {
        buf.set_style(area, styles::GAME_BORDER);

        // Draw thick top and bottom rows
        for x in left..=right {
            if let Some(cell) = buf.cell_mut((x, top)) {
                cell.set_symbol("▄");
            }
            if let Some(cell) = buf.cell_mut((x, bottom)) {
                cell.set_symbol("▀");
            }
        }

        // Draw thin sides (skipping the corners we just drew)
        for y in (top + 1)..bottom {
            if let Some(cell) = buf.cell_mut((left, y)) {
                cell.set_symbol("▐");
            }
            if let Some(cell) = buf.cell_mut((right, y)) {
                cell.set_symbol("▌");
            }
        }
    }

    #[cfg(target_os = "macos")]
    // Draw game borders using background fill to bypass macOS line-spacing gaps
    {
        // Draw solid thick horizontal rows (including the corners)
        for x in left..=right {
            if let Some(cell) = buf.cell_mut((x, top)) {
                cell.set_symbol(" ").set_style(styles::GAME_BORDER);
            }
            if let Some(cell) = buf.cell_mut((x, bottom)) {
                cell.set_symbol(" ").set_style(styles::GAME_BORDER);
            }
        }

        // Draw solid thick vertical sides
        for y in (top + 1)..bottom {
            for x in [left, right] {
                if let Some(cell) = buf.cell_mut((x, y)) {
                    cell.set_symbol(" ").set_style(styles::GAME_BORDER);
                }
            }
        }
    }
}

// =============================================================================
// Keys legend
// =============================================================================

fn draw_keys_legend(frame: &mut Frame, area: Rect, stage: &Stage) {
    // Delimiter line
    let buf = frame.buffer_mut();
    buf.set_style(area, styles::LEGEND_BORDER);

    #[cfg(target_os = "macos")]
    let delimiter_line_symbol = "—"; // em dash
    #[cfg(not(target_os = "macos"))]
    let delimiter_line_symbol = "─"; // hyphen

    for x in area.left()..area.right() {
        if let Some(cell) = buf.cell_mut((x, area.y)) {
            cell.set_symbol(delimiter_line_symbol);
        }
    }

    // Keys and actions
    let x = area.x + 1;
    let mut y = area.y + 1;
    let buf = frame.buffer_mut();

    match stage {
        Stage::Ready(_) => {
            y = draw_legend_item(buf, x, y, "Enter", "Start");
            y = draw_legend_item(buf, x, y, "F1", "How to play");
        }
        Stage::Gameplay(_) => {
            y = draw_legend_item(buf, x, y, "Arrows", "Move/Rotate");
            y = draw_legend_item(buf, x, y, "0 (Zero)", "Accelerate");
            y = draw_legend_item(buf, x, y, "Space", "Drop");
            y = draw_legend_item(buf, x, y, "Esc", "Pause");
        }
        Stage::Paused(_) => y = draw_legend_item(buf, x, y, "Enter", "Resume"),
        Stage::Instructions(_) => y = draw_legend_item(buf, x, y, "Enter", "Back to game"),
        Stage::GameOver(_) => y = draw_legend_item(buf, x, y, "Enter", "Restart"),
    }

    draw_legend_item(buf, x, y, "Q", "Quit");
}

fn draw_legend_item(buf: &mut ratatui::buffer::Buffer, x: u16, y: u16, key: &str, action: &str) -> u16 {
    buf.set_string(x, y, key, styles::LEGEND_KEY);
    buf.set_string(x + 13, y, action, styles::LEGEND_ACTION);
    y + 1
}

// =============================================================================
// Seed generators for randomizing Gem colors
// =============================================================================
fn seed_for_randomizing_next_column_blocks(tick: u64, x: u8, y: i8) -> u64 {
    generate_seed(1, tick, u64::from(y.unsigned_abs()), u64::from(x))
}

fn seed_for_randomizing_falling_column_blocks(tick: u64, x: u8, y: i8) -> u64 {
    generate_seed(2, tick, u64::from(y.unsigned_abs()), u64::from(x))
}

fn seed_for_randomizing_pile_blocks(tick: u64, x: u8, y: u8) -> u64 {
    generate_seed(3, tick, u64::from(y), u64::from(x))
}

/// Generates a unique randomizing seed by bit-packing type (flag), timing (tick) and coordinates (x and y).
///
/// bits:   | 63 ........ 48 | 47 .... 32 | 31 .... 16 | 15 .... 0 |
/// chunks: | type           | tick       | y          | x         |
const fn generate_seed(flag: u64, tick: u64, y: u64, x: u64) -> u64 {
    (flag << 48) | (tick << 32) | (y << 16) | x
}
