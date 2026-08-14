use ratatui::{Frame, layout::Rect, style::Style};

use crate::{blocks::Gem, terminal};

const EMOJI_BULLETS: &str = "👉✨🎯🚀🔧📬";
const GEM_BULLETS: &[Gem] = &[Gem::Amethyst, Gem::Ruby, Gem::Amber, Gem::Topaz, Gem::Emerald, Gem::Sapphire];

// Every bullet is four UTF-8 bytes except `✨` at index 1, which is three.
fn emoji_bullet(index: usize) -> &'static str {
    let start = index * 4 - usize::from(index > 1);
    let end = (index + 1) * 4 - usize::from(index > 0);
    &EMOJI_BULLETS[start..end]
}

const INSTRUCTIONS: &str = "\
@Gain points by
matching colors in all
four directions.

@The more blocks
matched in a line, the
more points you earn.

@Multiple matches
multiply gained points.

@Sequential, cascading
matches earn huge
bonuses.

--
@Created by Rade Drmic
@rdrmic@gmail.com";

#[allow(clippy::manual_strip)]
pub(super) fn draw_instructions(frame: &mut Frame, area: Rect) {
    let buf = frame.buffer_mut();
    let has_emoji_support = terminal::has_emoji_support();

    let x = area.x + 2;
    let mut y = area.y + 1;

    let mut bullet_area = Rect { x, y, width: 2, height: 1 };
    let mut bullet_idx = 0;

    let bytes = INSTRUCTIONS.as_bytes();
    let mut line_start = 0;
    loop {
        let mut line_end = line_start;
        while line_end < bytes.len() && bytes[line_end] != b'\n' {
            line_end += 1;
        }
        let line = &INSTRUCTIONS[line_start..line_end];

        if line.starts_with('@') {
            if has_emoji_support {
                buf.set_string(x, y, emoji_bullet(bullet_idx), Style::default());
            } else {
                bullet_area.y = y;
                buf.set_style(bullet_area, Style::from(GEM_BULLETS[bullet_idx]));
            }
            buf.set_string(x + 3, y, &line[1..], Style::default());
            bullet_idx += 1;
        } else if !line.is_empty() {
            buf.set_string(x, y, line, Style::default());
        }
        y += 1;

        if line_end == bytes.len() {
            break;
        }
        line_start = line_end + 1;
    }
}

// =============================================================================
// TESTS
// =============================================================================
#[cfg(test)]
mod tests {
    use super::{EMOJI_BULLETS, emoji_bullet};

    #[test]
    fn packed_emoji_bullets_preserve_the_original_entries() {
        let expected = ["👉", "✨", "🎯", "🚀", "🔧", "📬"];

        for (index, expected) in expected.into_iter().enumerate() {
            assert_eq!(emoji_bullet(index), expected);
        }
        assert_eq!(EMOJI_BULLETS.len(), 23);
    }

    #[test]
    fn packed_emoji_bullets_reject_an_invalid_index() {
        assert!(std::panic::catch_unwind(|| emoji_bullet(6)).is_err());
    }
}
