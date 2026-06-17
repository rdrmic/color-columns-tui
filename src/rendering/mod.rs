use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, HorizontalAlignment, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Text},
    widgets::{Block, Borders, Padding, Paragraph, Wrap},
};

#[cfg(feature = "dev-console")]
use crate::logging;
use crate::{
    blocks::{Gem, GemBlock},
    game_state::{BOARD_HEIGHT, BOARD_WIDTH, GameState},
    stage_handlers::Stage,
};

mod gameover_ui;
mod gameplay_ui;
mod instructions_ui;
mod paused_ui;
mod ready_ui;

#[cfg(feature = "dev-console")]
pub const MIN_WINDOW_WIDTH: u16 = 160;
#[cfg(not(feature = "dev-console"))]
pub const MIN_WINDOW_WIDTH: u16 = 29;
pub const MIN_WINDOW_HEIGHT: u16 = 27;

// ============================================================================
// Entry point for rendering
// ============================================================================
pub fn render(frame: &mut Frame, stage: &Stage, game: &GameState) {
    let frame_area = frame.area();

    set_bg_and_fg_colors(frame, frame_area);

    if is_terminal_window_too_small(frame_area) {
        render_message_terminal_window_too_small(frame, frame_area);
        return;
    }

    let layout_areas = get_layout_areas(frame_area);

    if matches!(stage, Stage::Instructions(_)) {
        instructions_ui::draw_instructions(frame, layout_areas.instructions);
    } else {
        draw_shared_areas(frame, &layout_areas, game, stage);
    }
    draw_footer(frame, layout_areas.key_legend, stage);

    #[cfg(feature = "dev-console")]
    logging::dev_console::draw(frame, layout_areas.dev_console);
}

fn set_bg_and_fg_colors(frame: &mut Frame, frame_area: Rect) {
    frame.buffer_mut().set_style(frame_area, Style::default().bg(Color::Black).fg(Color::Rgb(0, 225, 0)));
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
        Stage::Ready(_) => ready_ui::draw_footer(frame, area),
        Stage::Gameplay(_) => gameplay_ui::draw_footer(frame, area),
        Stage::Paused(_) => paused_ui::draw_footer(frame, area),
        Stage::Instructions(_) => instructions_ui::draw_footer(frame, area),
        Stage::GameOver(_) => gameover_ui::draw_footer(frame, area),
    }
}

// ============================================================================
// Terminal window size check
// ============================================================================
const fn is_terminal_window_too_small(area: Rect) -> bool {
    area.width < MIN_WINDOW_WIDTH || area.height < MIN_WINDOW_HEIGHT
}

fn render_message_terminal_window_too_small(frame: &mut Frame, area: Rect) {
    let msg = vec![
        Line::styled("Terminal window too small!", Color::Red),
        Line::from(""),
        Line::styled(format!("Required: {MIN_WINDOW_WIDTH}x{MIN_WINDOW_HEIGHT}"), Color::Green),
        Line::styled(format!("Current:  {}x{}", area.width, area.height), Color::Red),
        Line::from(""),
        Line::styled("Please resize the window to play.", Color::Green),
    ];
    frame.render_widget(
        Paragraph::new(msg)
            .style(Style::default().fg(Color::Red))
            .block(Block::bordered().border_style(Style::default().fg(Color::Indexed(240))))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true }),
        area,
    );
}

// ============================================================================
// Layout areas
// ============================================================================
struct LayoutAreas {
    level: Rect,
    message: Rect,
    next_column: Rect,
    stats: Rect,
    board: Rect,
    instructions: Rect,
    key_legend: Rect,
    #[cfg(feature = "dev-console")]
    dev_console: Rect,
}

fn get_layout_areas(area: Rect) -> LayoutAreas {
    #[cfg(feature = "dev-console")]
    let horizontal_constraints = [Constraint::Length(29), Constraint::Length(13), Constraint::Min(0)];
    #[cfg(not(feature = "dev-console"))]
    let horizontal_constraints = [Constraint::Length(29)];

    let main_horizontal_layout = Layout::default().direction(Direction::Horizontal).constraints(horizontal_constraints).split(area);
    let entire_area = main_horizontal_layout[0];

    let main_vertical_layout =
        Layout::default().direction(Direction::Vertical).constraints([Constraint::Length(18), Constraint::Min(0)]).margin(1).spacing(1).split(entire_area);
    let entire_game_area = main_vertical_layout[0];
    let key_legend_area = main_vertical_layout[1];

    let game_vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(2), Constraint::Min(0)])
        .split(entire_game_area);
    let level_area = game_vertical_layout[0];
    let message_area = game_vertical_layout[1];
    let game_area = game_vertical_layout[2];

    let game_horizontal_layout = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Length(13), Constraint::Min(0)]).split(game_area);

    let left_side_area = game_horizontal_layout[0];
    let board_area = game_horizontal_layout[1];

    let left_side_vertical_layout =
        Layout::default().direction(Direction::Vertical).constraints([Constraint::Length(1), Constraint::Length(3), Constraint::Min(0)]).split(left_side_area);
    let next_column_area = left_side_vertical_layout[1];
    let stats_area = left_side_vertical_layout[2];

    LayoutAreas {
        level: level_area,
        message: message_area,
        next_column: next_column_area,
        stats: stats_area,
        board: board_area,
        instructions: entire_area,
        key_legend: key_legend_area,
        #[cfg(feature = "dev-console")]
        dev_console: main_horizontal_layout[2],
    }
}

// ============================================================================
// Level and user messages
// ============================================================================
fn draw_level(frame: &mut Frame, area: Rect, game: &GameState, stage: &Stage) {
    let style = if let Stage::Gameplay(gameplay_handler) = stage
        && gameplay_handler.blinking_labels().has_level_blinked()
    {
        Style::default().fg(Color::Black)
    } else {
        Style::default().fg(Color::Indexed(208)).add_modifier(Modifier::BOLD | Modifier::ITALIC)
    };

    let level = Paragraph::new(format!("LEVEL {}", game.scoring().level())).block(Block::default().padding(Padding::left(1)).style(style));
    frame.render_widget(level, area);
}

pub fn draw_message(frame: &mut Frame, area: Rect, game: &GameState) {
    let Some(msg) = game.message() else {
        return;
    };

    let message = Paragraph::new(msg.text())
        .alignment(HorizontalAlignment::Right)
        .block(Block::default().padding(Padding::horizontal(1)).style(Style::default().fg(msg.color()).add_modifier(Modifier::BOLD | Modifier::ITALIC)));
    frame.render_widget(message, area);
}

// ============================================================================
// Left side (next column and stats) and game board
// ============================================================================
fn draw_next_column(frame: &mut Frame, area: Rect, game: &GameState, stage: &Stage) {
    let right_aligned_area = Layout::horizontal([Constraint::Min(0), Constraint::Length(2), Constraint::Length(1)]).split(area)[1];

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
            if handler.blinking_labels().has_max_combo_blinked() { Color::Black } else { Color::Magenta },
            if handler.blinking_labels().has_highscore_blinked() { Color::Black } else { Color::Red }
        ),
        _ => (Color::Magenta, Color::Red)
    };

    let stats = [
        ("SCORE",       game.scoring().score(),                   Color::Green),
        ("MAX COMBO",   u32::from(game.scoring().max_combo()),    max_combo_label_color),
        ("HIGHSCORE",   game.scoring().highscore(),               highscore_label_color),
    ];

    let lines = stats.into_iter().fold(Vec::with_capacity(stats.len() * 3), |mut lines, (label, value, color)| {
        lines.push(Line::default());
        lines.push(Line::from(label).style(Style::default().fg(color).bold()));
        lines.push(Line::from(value.to_string()).style(Style::default().fg(Color::Indexed(152))));
        lines
    });

    let stats = Paragraph::new(lines).block(Block::default().padding(ratatui::widgets::Padding::new(1, 1, 1, 0)));
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

#[rustfmt::skip]
fn draw_board_border(frame: &mut Frame, area: Rect) {
    let buf = frame.buffer_mut();
    buf.set_style(area, Style::default().fg(Color::Indexed(245)));

    let left = area.left();
    let right = area.right() - 1;
    let top = area.top();
    let bottom = area.bottom() - 1;
    for x in left..=right {
        for y in top..=bottom {
            if let Some(cell) = buf.cell_mut((x, y))
                && (x == left || x == right || y == top || y == bottom)
            {
                let symbol = match (x == left, x == right, y == top, y == bottom) {
                    (true, _, true, _)  => "▗", // LT corner
                    (_, true, true, _)  => "▖", // RT corner
                    (true, _, _, true)  => "▝", // LB corner
                    (_, true, _, true)  => "▘", // RB corner
                    (true, _, _, _)     => "🭵", // Left edge
                    (_, true, _, _)     => "🭰", // Right edge
                    (_, _, true, _)     => "▂", // Top edge
                    (_, _, _, true)     => "▀", // Bottom edge
                    _ => " ",
                };
                cell.set_symbol(symbol);
            }
        }
    }
}

// ============================================================================
// Keys legend
// ============================================================================
const STYLE_KEYS: Style = Style::new().fg(Color::Indexed(150)).bold();
const STYLE_ACTIONS: Style = Style::new().fg(Color::Gray);

struct LegendItem {
    key: &'static str,
    action: &'static str,
}

#[rustfmt::skip]
fn compile_legend(legend: &[LegendItem]) -> (Text<'static>, Text<'static>) {
    let keys = Text::from(
        legend.iter()
            .map(|item| Line::from(item.key).style(STYLE_KEYS))
            .collect::<Vec<_>>()
    ).alignment(Alignment::Left);

    let actions = Text::from(
        legend.iter()
            .map(|item| Line::from(item.action).style(STYLE_ACTIONS))
            .collect::<Vec<_>>()
    ).alignment(Alignment::Left);

    (keys, actions)
}

fn draw_keys_legend(frame: &mut Frame, area: Rect, legend: (Text<'_>, Text<'_>)) {
    let legend_block = Block::default().borders(Borders::TOP).border_style(Style::default().fg(Color::Indexed(245)));
    frame.render_widget(&legend_block, area);

    let horizontal_layout = Layout::horizontal([Constraint::Length(1), Constraint::Length(13), Constraint::Min(0)]).split(legend_block.inner(area));
    let keys_area = horizontal_layout[1];
    let actions_area = horizontal_layout[2];

    let (keys, actions) = legend;
    frame.render_widget(Paragraph::new(keys), keys_area);
    frame.render_widget(Paragraph::new(actions), actions_area);
}

// ============================================================================
// Seed generators for randomizing Gem colors
// ============================================================================
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
