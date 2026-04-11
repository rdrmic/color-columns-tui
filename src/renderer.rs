use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Paragraph},
};

use crate::game::GameState;

const BOARD_WIDTH: u16 = GameState::BOARD_WIDTH as u16 * 2 + 2;
const BOARD_HEIGHT: u16 = GameState::BOARD_HEIGHT as u16 + 2;

pub fn render(frame: &mut Frame, state: &GameState) {
    #[cfg(feature = "dev-console")]
    let horizontal_constraints = [Constraint::Length(14), Constraint::Max(BOARD_WIDTH), Constraint::Length(17), Constraint::Min(0)];
    #[cfg(not(feature = "dev-console"))]
    let horizontal_constraints = [Constraint::Length(15), Constraint::Max(BOARD_WIDTH)];

    let main_layout = Layout::default().direction(Direction::Horizontal).constraints(horizontal_constraints).split(frame.area());

    let left_area = main_layout[0];
    let game_area = main_layout[1];

    // 2. Sub-split Left Area for "Next Piece" and "Stats"
    let left_layout = Layout::default().direction(Direction::Vertical).constraints([Constraint::Length(9), Constraint::Length(10)]).split(left_area);
    draw_stats(frame, left_layout[1], state);

    // 3. Sub-split Game Area for Notifications and Game Board
    let game_layout = Layout::default().direction(Direction::Vertical).constraints([Constraint::Length(4), Constraint::Max(BOARD_HEIGHT)]).split(game_area);
    draw_board(frame, game_layout[1], state);

    #[cfg(feature = "dev-console")]
    {
        use crate::logging;
        let dev_console_area = main_layout[3];
        logging::dev_console::draw(frame, dev_console_area);
    }
}

fn draw_stats(frame: &mut Frame, area: Rect, _state: &GameState) {
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

fn draw_board(frame: &mut Frame, area: Rect, game: &GameState) {
    // Game board border
    draw_board_border(frame, area);

    let board_area = Rect { x: area.x + 1, y: area.y + 1, width: area.width - 2, height: area.height - 2 };

    // Falling column
    frame.render_widget(game.get_falling_column(), board_area);
    // Pile
    frame.render_widget(game.get_pile(), board_area);
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
