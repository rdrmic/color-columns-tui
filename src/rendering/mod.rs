use ratatui::{
    Frame,
    layout::{Alignment, HorizontalAlignment, Rect},
    style::{Modifier, Style},
    text::{Line, Text},
    widgets::{Block, Borders, Padding, Paragraph, Wrap},
};

#[cfg(feature = "dev-console")]
use crate::logging;
use crate::{
    blocks::{Gem, GemBlock},
    game_state::{BOARD_HEIGHT, BOARD_WIDTH, GameState},
    palette,
    stage_handlers::Stage,
};

mod gameover;
mod gameplay;
mod instructions;
mod paused;
mod ready;

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

    pub(super) const BG_AND_FG_COLORS: Style = Style::new().fg(palette::UI_CANVAS_FG).bg(palette::UI_CANVAS_BG);
    pub(super) const FG_AS_BG_COLOR: Style   = Style::new().fg(palette::UI_CANVAS_BG);
    #[cfg(not(target_os = "macos"))]
    pub(super) const GAME_BORDER: Style      = Style::new().fg(palette::UI_GAME_BORDER);
    pub(super) const LEGEND_BORDER: Style    = Style::new().fg(palette::UI_LEGEND_BORDER);
    pub(super) const LEGEND_KEYS: Style      = Style::new().fg(palette::UI_LEGEND_KEY).bold();
    pub(super) const LEGEND_ACTIONS: Style   = Style::new().fg(palette::UI_LEGEND_VALUE);
    pub(super) const LEVEL: Style            = Style::new().fg(palette::STAT_LEVEL).bold().italic();
    pub(super) const STATS_VALUES: Style     = Style::new().fg(palette::STATS_VALUE);
}

// =============================================================================
// Rendering entry point
// =============================================================================
pub fn render(frame: &mut Frame, stage: &Stage, game: &GameState) {
    let frame_area = frame.area();

    set_bg_and_fg_colors(frame, frame_area);

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
    draw_footer(frame, layout_areas.key_legend, stage);

    #[cfg(feature = "dev-console")]
    logging::dev_console::draw(frame, layout_areas.dev_console);
}

fn set_bg_and_fg_colors(frame: &mut Frame, frame_area: Rect) {
    frame.buffer_mut().set_style(frame_area, styles::BG_AND_FG_COLORS);
}

fn draw_shared_areas(frame: &mut Frame, layout_areas: &LayoutAreas, game: &GameState, stage: &Stage) {
    draw_level(frame, layout_areas.level, game, stage);
    draw_message(frame, layout_areas.message, game);
    draw_next_column(frame, layout_areas.next_column, game, stage);
    draw_stats(frame, layout_areas.stats, game, stage);
    draw_board(frame, layout_areas.board, game, stage);
}

fn draw_footer(frame: &mut Frame, area: Rect, stage: &Stage) {
    match stage {
        Stage::Ready(_) => ready::draw_footer(frame, area),
        Stage::Gameplay(_) => gameplay::draw_footer(frame, area),
        Stage::Paused(_) => paused::draw_footer(frame, area),
        Stage::Instructions(_) => instructions::draw_footer(frame, area),
        Stage::GameOver(_) => gameover::draw_footer(frame, area),
    }
}

// =============================================================================
// Terminal window size check
// =============================================================================
const fn is_terminal_window_too_small(area: Rect) -> bool {
    area.width < MIN_WINDOW_WIDTH || area.height < MIN_WINDOW_HEIGHT
}

fn render_message_terminal_window_too_small(frame: &mut Frame, mut area: Rect) {
    area.y += 1;

    let msg = vec![
        Line::styled("Terminal window too small!", palette::CONSOLE_TEXT_ERROR),
        Line::default(),
        Line::styled(format!("Required: {MIN_WINDOW_WIDTH}x{MIN_WINDOW_HEIGHT}"), palette::CONSOLE_TEXT_INFO),
        Line::styled(format!("Current:  {}x{}", area.width, area.height), palette::CONSOLE_TEXT_ERROR),
        Line::default(),
        Line::styled("Please resize the window to play.", palette::CONSOLE_TEXT_INFO),
    ];
    frame.render_widget(Paragraph::new(msg).alignment(Alignment::Center).wrap(Wrap { trim: true }), area);
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
    key_legend: Rect,
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
        stats:        Rect { x: left_side_area.x,         y: left_side_area.y + 4,      width: left_side_area.width,     height: left_side_area.height - 4 },
        board:        Rect { x: game_area.x + 13,         y: game_area.y,               width: game_area.width - 13,     height: game_area.height },
        key_legend:   Rect { x: entire_area_padded.x,     y: entire_area_padded.y + 19, width: entire_area_padded.width, height: entire_area_padded.height - 19 },
        instructions: entire_area,
        #[cfg(feature = "dev-console")]
        dev_console:  Rect { x: entire_area.right() + 13, y: frame_area.y,              width: frame_area.width - 42,    height: frame_area.height }
    }
}

// =============================================================================
// Level and user messages
// =============================================================================
fn draw_level(frame: &mut Frame, area: Rect, game: &GameState, stage: &Stage) {
    if let Stage::Instructions(_) = stage {
        return;
    }

    let style = match stage {
        Stage::Gameplay(gameplay_handler) if !gameplay_handler.blinking_labels().is_level_visible() => styles::FG_AS_BG_COLOR,
        _ => styles::LEVEL,
    };

    let level = Paragraph::new(format!("LEVEL {}", game.scoring().level())).style(style);
    frame.render_widget(level, area);
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

    //let area = area.offset(Offset::new(-1, 0));

    let message = Paragraph::new(msg.text()).style(Style::from((msg.color(), Modifier::BOLD | Modifier::ITALIC))).alignment(HorizontalAlignment::Right);
    frame.render_widget(message, area);
}

// =============================================================================
// Left side (next column and stats) and game board
// =============================================================================
fn draw_next_column(frame: &mut Frame, area: Rect, game: &GameState, stage: &Stage) {
    let right_aligned_area = Rect { x: area.right().saturating_sub(2), y: area.y, width: 2, height: area.height };

    // FIXME remove
    //let background = Block::new().style(Style::new().bg(ratatui::style::Color::Blue));
    //frame.render_widget(background, area);

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
fn draw_stats(frame: &mut Frame, area: Rect, game: &GameState, stage: &Stage) {
    let (max_combo_label_color, highscore_label_color) = match stage {
        Stage::Gameplay(handler) => (
            if handler.blinking_labels().is_max_combo_visible() { palette::STAT_MAX_COMBO } else { palette::UI_CANVAS_BG },
            if handler.blinking_labels().is_highscore_visible() { palette::STAT_HIGHSCORE } else { palette::UI_CANVAS_BG },
        ),
        _ => (palette::STAT_MAX_COMBO, palette::STAT_HIGHSCORE)
    };

    let stats = [
        ("SCORE",       game.scoring().score(),                   palette::STAT_SCORE),
        ("MAX COMBO",   u32::from(game.scoring().max_combo()),    max_combo_label_color),
        ("HIGHSCORE",   game.scoring().highscore(),               highscore_label_color),
    ];

    let lines = stats.into_iter().fold(Vec::with_capacity(stats.len() * 3), |mut lines, (label, value, color)| {
        lines.push(Line::default());
        lines.push(Line::styled(label, Style::from((color, Modifier::BOLD))));
        lines.push(Line::styled(value.to_string(), styles::STATS_VALUES));
        lines
    });

    let stats = Paragraph::new(lines).block(Block::default().padding(Padding::new(1, 1, 1, 0)));
        //.style(Style::new().bg(ratatui::style::Color::DarkGray)); // FIXME remove
    frame.render_widget(stats, area);
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
        let border_color = palette::UI_GAME_BORDER;

        // Draw solid thick horizontal rows (including the corners)
        for x in left..=right {
            if let Some(cell) = buf.cell_mut((x, top)) {
                cell.set_symbol(" ").set_bg(border_color);
            }
            if let Some(cell) = buf.cell_mut((x, bottom)) {
                cell.set_symbol(" ").set_bg(border_color);
            }
        }

        // Draw solid thick vertical sides
        for y in (top + 1)..bottom {
            for x in [left, right] {
                if let Some(cell) = buf.cell_mut((x, y)) {
                    cell.set_symbol(" ").set_bg(border_color);
                }
            }
        }
    }
}

// =============================================================================
// Keys legend
// =============================================================================
struct LegendItem {
    key: &'static str,
    action: &'static str,
}

#[rustfmt::skip]
fn compile_legend(legend: &[LegendItem]) -> (Text<'static>, Text<'static>) {
    let keys = Text::from(
        legend.iter()
            .map(|item| Line::styled(item.key, styles::LEGEND_KEYS))
            .collect::<Vec<_>>()
    ).alignment(Alignment::Left);

    let actions = Text::from(
        legend.iter()
            .map(|item| Line::styled(item.action, styles::LEGEND_ACTIONS))
            .collect::<Vec<_>>()
    ).alignment(Alignment::Left);

    (keys, actions)
}

fn draw_keys_legend(frame: &mut Frame, area: Rect, legend: (Text<'_>, Text<'_>)) {
    #[cfg(not(target_os = "macos"))]
    {
        let delimiting_line_block = Block::default().borders(Borders::TOP).border_style(styles::LEGEND_BORDER);
        frame.render_widget(delimiting_line_block, area);
    }

    #[cfg(target_os = "macos")]
    {
        let buf = frame.buffer_mut();

        let line_style = styles::LEGEND_BORDER.add_modifier(Modifier::UNDERLINED);

        let line_y = area.top().saturating_sub(1);
        for x in area.left()..area.right() {
            if let Some(cell) = buf.cell_mut((x, line_y)) {
                cell.set_symbol(" ").set_style(line_style);
            }
        }
    }

    let inner_area = Block::default().borders(Borders::TOP).inner(area);

    let keys_area = Rect { x: inner_area.x + 1, y: inner_area.y, width: 13, height: inner_area.height };

    let actions_area = Rect { x: inner_area.x + 14, y: inner_area.y, width: inner_area.width.saturating_sub(14), height: inner_area.height };

    let (keys, actions) = legend;
    frame.render_widget(Paragraph::new(keys), keys_area);
    frame.render_widget(Paragraph::new(actions), actions_area);
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
