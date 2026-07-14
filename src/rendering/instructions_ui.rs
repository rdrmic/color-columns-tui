use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Padding, Paragraph, Wrap},
};

use crate::{blocks::Gem, terminal};

use super::{LegendItem, compile_legend, draw_keys_legend};

#[rustfmt::skip]
const LEGEND_ITEMS: &[LegendItem] = &[
    LegendItem { key: "Enter", action: "Back to game" },
    LegendItem { key: "Q",     action: "Quit" },
];

const TIP_1: &str = " Gain points by matching colors in all four directions.";
const TIP_2: &str = " The more blocks matched in a line, the more points you earn.";
const TIP_3: &str = " Multiple matches multiply gained points.";
const TIP_4: &str = " Sequential, cascading matches earn huge bonuses.";
const CREATED_BY: &str = " Created by Rade Drmic";
const EMAIL: &str = " rdrmic@gmail.com";

pub(super) fn draw_footer(frame: &mut Frame, area: Rect) {
    draw_keys_legend(frame, area, compile_legend(LEGEND_ITEMS));
}

// If terminal supports Emoji, use them; otherwise, fall back to Gems
pub(super) fn draw_instructions(frame: &mut Frame, area: Rect) {
    let paragraph = if terminal::has_emoji_support() {
        let string = format!("\n👉{TIP_1}\n\n✨{TIP_2}\n\n🎯{TIP_3}\n\n🚀{TIP_4}\n\n--\n🔧{CREATED_BY}\n📬{EMAIL}");
        Paragraph::new(string)
    } else {
        let lines = vec![
            Line::from(""),
            Line::from(vec![Span::styled("  ", Style::from(Gem::Amethyst)), Span::raw(TIP_1)]),
            Line::from(""),
            Line::from(vec![Span::styled("  ", Style::from(Gem::Ruby)), Span::raw(TIP_2)]),
            Line::from(""),
            Line::from(vec![Span::styled("  ", Style::from(Gem::Amber)), Span::raw(TIP_3)]),
            Line::from(""),
            Line::from(vec![Span::styled("  ", Style::from(Gem::Topaz)), Span::raw(TIP_4)]),
            Line::from(""),
            Line::from(""),
            Line::from(vec![Span::styled("  ", Style::from(Gem::Emerald)), Span::raw(CREATED_BY)]),
            Line::from(vec![Span::styled("  ", Style::from(Gem::Sapphire)), Span::raw(EMAIL)]),
        ];
        Paragraph::new(lines)
    };

    let instructions = paragraph.block(Block::default().padding(Padding::horizontal(2))).wrap(Wrap { trim: false });
    frame.render_widget(instructions, area);
}
