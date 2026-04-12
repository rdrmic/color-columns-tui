use ratatui::{Frame, text::Line};

use super::{draw_board, draw_keys_legend, draw_stats, get_layout_areas};
use crate::game::Game;

pub(super) fn render(frame: &mut Frame, game: &Game) {
    let layout_areas = get_layout_areas(frame.area());

    draw_board(frame, layout_areas.board, game);
    draw_stats(frame, layout_areas.stats, game);
    draw_keys_legend(frame, layout_areas.footer, create_legend_items());

    #[cfg(feature = "dev-console")]
    {
        use crate::logging;
        logging::dev_console::draw(frame, layout_areas.dev_console);
    }
}

#[rustfmt::skip]
fn create_legend_items<'a>() -> [Vec<Line<'a>>; 2] {
    [
        vec![Line::from("Enter"),   Line::from("Q")],
        vec![Line::from("Restart"), Line::from("Quit")]
    ]
}
