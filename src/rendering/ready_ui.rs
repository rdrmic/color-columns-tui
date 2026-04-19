use std::sync::LazyLock;

use ratatui::{Frame, text::Text};

use super::{LayoutAreas, LegendItem, compile_legend, draw_board, draw_keys_legend, draw_stats};
use crate::game::Game;

#[rustfmt::skip]
const LEGEND_ITEMS: &[LegendItem] = &[
    LegendItem { key: "Enter", action: "Start" },
    LegendItem { key: "F1", action: "How to play" },
    LegendItem { key: "Q",     action: "Quit" },
];

static LEGEND: LazyLock<(Text<'_>, Text<'_>)> = LazyLock::new(|| compile_legend(LEGEND_ITEMS));

pub(super) fn render(frame: &mut Frame, game: &Game, layout_areas: &LayoutAreas) {
    draw_board(frame, layout_areas.board, game);
    draw_stats(frame, layout_areas.stats, game);
    draw_keys_legend(frame, layout_areas.footer, &LEGEND);

    #[cfg(feature = "dev-console")]
    {
        use crate::logging;
        logging::dev_console::draw(frame, layout_areas.dev_console);
    }
}
