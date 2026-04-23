use std::sync::LazyLock;

use ratatui::{Frame, layout::Rect, text::Text};

use super::{LegendItem, compile_legend, draw_keys_legend};

#[rustfmt::skip]
const LEGEND_ITEMS: &[LegendItem] = &[
    LegendItem { key: "Enter", action: "Start" },
    LegendItem { key: "F1", action: "How to play" },
    LegendItem { key: "Q",     action: "Quit" },
];

static LEGEND: LazyLock<(Text<'_>, Text<'_>)> = LazyLock::new(|| compile_legend(LEGEND_ITEMS));

pub(super) fn draw_footer(frame: &mut Frame, area: Rect) {
    draw_keys_legend(frame, area, &LEGEND);
}
