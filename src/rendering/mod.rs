mod gameover_ui;
mod gameplay_ui;
mod ready_ui;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

use crate::{game::Game, stage_handlers::Stage};

const BOARD_WIDTH: u16 = Game::BOARD_WIDTH as u16 * 2 + 2;
const BOARD_HEIGHT: u16 = Game::BOARD_HEIGHT as u16 + 2;

pub fn render(frame: &mut Frame, stage: &Stage, game: &Game) {
    match stage {
        Stage::Ready(_) => ready_ui::render(frame, game),
        Stage::Gameplay(_) => gameplay_ui::render(frame, game),
        Stage::GameOver(_) => gameover_ui::render(frame, game),
    }
}

struct LayoutAreas {
    pub board: Rect,
    pub stats: Rect,
    pub footer: Rect,
    #[cfg(feature = "dev-console")]
    pub dev_console: Rect,
}

/// Calculates the standard game layout to be used by all stages
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

    let stats_area = game_horizontal_layout[0];
    let board_area = game_horizontal_layout[1];

    LayoutAreas {
        board: board_area,
        stats: stats_area,
        footer: footer_area,
        #[cfg(feature = "dev-console")]
        dev_console: main_horizontal_layout[2],
    }
}

fn draw_stats(frame: &mut Frame, area: Rect, _game: &Game) {
    let stats_vertical_layout = Layout::default().direction(Direction::Vertical).constraints([Constraint::Length(9), Constraint::Length(10)]).split(area);
    let target_area = stats_vertical_layout[1];

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
    frame.render_widget(stats, target_area);
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
                    (true, _, true, _) => "▗", // LT corner
                    (_, true, true, _) => "▖", // RT corner
                    (true, _, _, true) => "▝", // LB corner
                    (_, true, _, true) => "▘", // RB corner
                    (true, _, _, _) => "🭵",    // Left edge
                    (_, true, _, _) => "🭰",    // Right edge
                    (_, _, true, _) => "▂",    // Top edge
                    (_, _, _, true) => "▀",    // Bottom edge
                    _ => " ",
                };
                cell.set_symbol(symbol);
            }
        }
    }
}

fn draw_keys_legend(frame: &mut Frame, area: Rect, items: [Vec<Line<'_>>; 2]) {
    let legend_block = Block::default().borders(Borders::TOP).border_style(Style::default().fg(Color::Indexed(245)));
    frame.render_widget(&legend_block, area);

    let horizontal_layout = Layout::horizontal([Constraint::Length(1), Constraint::Length(14), Constraint::Min(0)]).split(legend_block.inner(area));
    let keys_area = horizontal_layout[1];
    let actions_area = horizontal_layout[2];

    let style_keys = Style::default().fg(Color::Indexed(150)).add_modifier(Modifier::BOLD);
    let style_actions = Style::default().fg(Color::Indexed(152));

    let [keys, actions] = items;
    frame.render_widget(Paragraph::new(keys).style(style_keys).alignment(Alignment::Left), keys_area);
    frame.render_widget(Paragraph::new(actions).style(style_actions).alignment(Alignment::Left), actions_area);
}
