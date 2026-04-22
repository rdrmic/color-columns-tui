mod gameover_ui;
mod gameplay_ui;
mod ready_ui;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::{game::Game, stage_handlers::Stage};

// TODO calculate terminal window minimum sizes dynamically?
#[cfg(feature = "dev-console")]
pub const MIN_WINDOW_WIDTH: u16 = 160;
#[cfg(not(feature = "dev-console"))]
pub const MIN_WINDOW_WIDTH: u16 = 29;

pub const MIN_WINDOW_HEIGHT: u16 = 29;

const BOARD_WIDTH: u16 = Game::BOARD_WIDTH as u16 * 2 + 2;
const BOARD_HEIGHT: u16 = Game::BOARD_HEIGHT as u16 + 2;

pub fn render(frame: &mut Frame, stage: &Stage, game: &Game) {
    let frame_area = frame.area();

    if is_terminal_window_too_small(frame_area) {
        render_message_terminal_window_too_small(frame, frame_area);
        return;
    }

    let layout_areas = get_layout_areas(frame_area);
    match stage {
        Stage::Ready(_) => ready_ui::render(frame, game, &layout_areas),
        Stage::Gameplay(_) => gameplay_ui::render(frame, game, &layout_areas),
        Stage::GameOver(_) => gameover_ui::render(frame, game, &layout_areas),
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
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Red))
            .block(Block::bordered().border_style(Style::default().fg(Color::Indexed(240))))
            .wrap(Wrap { trim: true }),
        area,
    );
}

// ============================================================================
// Layout areas
// ============================================================================
struct LayoutAreas {
    next_column: Rect,
    stats: Rect,
    board: Rect,
    footer: Rect,
    #[cfg(feature = "dev-console")]
    dev_console: Rect,
}

fn get_layout_areas(area: Rect) -> LayoutAreas {
    #[cfg(feature = "dev-console")]
    let horizontal_constraints = [Constraint::Length(BOARD_WIDTH + 15), Constraint::Length(17), Constraint::Min(0)];
    #[cfg(not(feature = "dev-console"))]
    let horizontal_constraints = [Constraint::Length(BOARD_WIDTH + 15)];

    let main_horizontal_layout = Layout::default().direction(Direction::Horizontal).constraints(horizontal_constraints).split(area);
    let game_area = main_horizontal_layout[0];

    let game_vertical_layout =
        Layout::default().direction(Direction::Vertical).constraints([Constraint::Length(19), Constraint::Length(1), Constraint::Min(0)]).split(game_area);
    let game_area = game_vertical_layout[0];
    let footer_area = game_vertical_layout[2];

    let game_horizontal_layout =
        Layout::default().direction(Direction::Horizontal).constraints([Constraint::Length(14), Constraint::Length(BOARD_WIDTH)]).split(game_area);

    let left_side_area = game_horizontal_layout[0];
    let board_area = game_horizontal_layout[1];

    let left_side_vertical_layout =
        Layout::default().direction(Direction::Vertical).constraints([Constraint::Length(5), Constraint::Length(4), Constraint::Min(0)]).split(left_side_area);
    let next_column_area = left_side_vertical_layout[1];
    let stats_area = left_side_vertical_layout[2];

    LayoutAreas {
        next_column: next_column_area,
        stats: stats_area,
        board: board_area,
        footer: footer_area,
        #[cfg(feature = "dev-console")]
        dev_console: main_horizontal_layout[2],
    }
}

// ============================================================================
// Game: left side (next column and stats) and board
// ============================================================================
// fn draw_next_column(frame: &mut Frame, area: Rect, game: &Game) {
//     frame.render_widget(game.get_next_column(), area);
// }

fn draw_next_column(frame: &mut Frame, area: Rect, game: &Game) {
    let right_aligned_area = Layout::horizontal([Constraint::Min(0), Constraint::Length(2), Constraint::Length(1)]).split(area)[1];

    frame.render_widget(game.get_next_column(), right_aligned_area);
}

fn draw_stats(frame: &mut Frame, area: Rect, _game: &Game) {
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

    let stats = Paragraph::new(stats_text).block(Block::default().padding(ratatui::widgets::Padding::horizontal(2)));
    frame.render_widget(stats, area);
}

fn draw_board(frame: &mut Frame, area: Rect, game: &Game) {
    let board_vertical_layout =
        Layout::default().direction(Direction::Vertical).constraints([Constraint::Length(4), Constraint::Max(BOARD_HEIGHT)]).split(area);
    let target_area = board_vertical_layout[1];

    draw_board_border(frame, target_area);

    let board_inner_area = Rect { x: target_area.x + 1, y: target_area.y + 1, width: target_area.width - 2, height: target_area.height - 2 };

    frame.render_widget(game.get_falling_column(), board_inner_area);
    frame.render_widget(game.get_pile(), board_inner_area);
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
const STYLE_KEYS: Style = Style::new().fg(Color::Indexed(150)).add_modifier(Modifier::BOLD);
const STYLE_ACTIONS: Style = Style::new().fg(Color::Indexed(152));

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

fn draw_keys_legend(frame: &mut Frame, area: Rect, legend: &(Text<'_>, Text<'_>)) {
    let legend_block = Block::default().borders(Borders::TOP).border_style(Style::default().fg(Color::Indexed(245)));
    frame.render_widget(&legend_block, area);

    let horizontal_layout = Layout::horizontal([Constraint::Length(1), Constraint::Length(14), Constraint::Min(0)]).split(legend_block.inner(area));
    let keys_area = horizontal_layout[1];
    let actions_area = horizontal_layout[2];

    let (keys, actions) = legend;
    frame.render_widget(Paragraph::new(keys.clone()), keys_area);
    frame.render_widget(Paragraph::new(actions.clone()), actions_area);
}
