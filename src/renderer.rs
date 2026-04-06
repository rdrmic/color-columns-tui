use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

use crate::game::GameState;

pub fn render(frame: &mut Frame, state: &GameState) {
    #[cfg(feature = "dev-console")]
    let horizontal_constraints =
        [Constraint::Length(15), Constraint::Length(24), Constraint::Length(17), Constraint::Min(0)];
    #[cfg(not(feature = "dev-console"))]
    let horizontal_constraints = [Constraint::Length(15), Constraint::Length(24)];

    let main_layout =
        Layout::default().direction(Direction::Horizontal).constraints(horizontal_constraints).split(frame.area());

    let left_area = main_layout[0];
    let game_area = main_layout[1];

    // 2. Sub-split Left Area for "Next Piece" and "Stats"
    let left_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(16), Constraint::Length(20)])
        .split(left_area);
    draw_stats(frame, left_layout[1], state);

    // 3. Sub-split Game Area for Notifications and Game Board
    let game_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(4), Constraint::Length(22)])
        .split(game_area);
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

fn draw_board(frame: &mut Frame, area: Rect, _state: &GameState) {
    let board_block = Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Indexed(245))); // Gray border

    // We render a block to define the play area
    frame.render_widget(board_block, area);

    // Note: To render the colored blocks, you would iterate over your game grid
    // and render tiny 1x2 Rects or specialized widgets inside 'area'.
}
