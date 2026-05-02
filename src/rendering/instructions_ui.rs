use std::sync::LazyLock;

use ratatui::{
    Frame,
    layout::Rect,
    text::Text,
    widgets::{Block, Padding, Paragraph, Wrap},
};

use super::{LegendItem, compile_legend, draw_keys_legend};

#[rustfmt::skip]
const LEGEND_ITEMS: &[LegendItem] = &[
    LegendItem { key: "Enter", action: "Return to menu" },
    LegendItem { key: "Q",     action: "Quit" },
];

static LEGEND: LazyLock<(Text<'_>, Text<'_>)> = LazyLock::new(|| compile_legend(LEGEND_ITEMS));

pub(super) fn draw_footer(frame: &mut Frame, area: Rect) {
    draw_keys_legend(frame, area, &LEGEND);
}

pub(super) fn draw_instructions(frame: &mut Frame, area: Rect) {
    let instructions = r"
        👉 Gain points by matching colors in all four directions.

        ✨ The more blocks matched in a line, the more points you earn.

        🎯 Multiple matches multiply gained points.

        🚀 Sequential, cascading matches earn huge bonuses.

        --
        ⚙  Created by Rade Drmic
        📬 rdrmic@gmail.com
    ";

    let instructions = Paragraph::new(instructions).block(Block::default().padding(Padding::new(1, 1, 1, 0))).wrap(Wrap { trim: true });
    frame.render_widget(instructions, area);
}
