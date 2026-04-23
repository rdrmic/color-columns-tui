use std::sync::LazyLock;

use ratatui::{Frame, layout::Rect, text::Text};

use super::{LegendItem, compile_legend, draw_keys_legend};

#[rustfmt::skip]
const LEGEND_ITEMS: &[LegendItem] = &[
    LegendItem { key: "Arrows",   action: "Move/Rotate" },
    LegendItem { key: "0 (Zero)", action: "Accelerate" },
    LegendItem { key: "Space",    action: "Drop" },
    LegendItem { key: "Esc",      action: "Pause" },
    LegendItem { key: "Q",        action: "Quit" },
];

static LEGEND: LazyLock<(Text<'_>, Text<'_>)> = LazyLock::new(|| compile_legend(LEGEND_ITEMS));

pub(super) fn draw_footer(frame: &mut Frame, area: Rect) {
    draw_keys_legend(frame, area, &LEGEND);
}
