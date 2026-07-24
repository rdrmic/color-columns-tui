use ratatui::{Frame, layout::Rect};

use super::{LegendItem, compile_legend, draw_keys_legend};

#[rustfmt::skip]
const LEGEND_ITEMS: &[LegendItem] = &[
    LegendItem { key: "Arrows",   action: "Move/Rotate" },
    LegendItem { key: "0 (Zero)", action: "Accelerate" },
    LegendItem { key: "Space",    action: "Drop" },
    LegendItem { key: "Esc",      action: "Pause" },
    LegendItem { key: "Q",        action: "Quit" },
];

pub(super) fn draw_footer(frame: &mut Frame, area: Rect) {
    draw_keys_legend(frame, area, compile_legend(LEGEND_ITEMS));
}
