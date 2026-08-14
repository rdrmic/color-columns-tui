//! All text arguments must be ASCII because alignment and wrapping use byte
//! lengths and byte indices as terminal-cell widths and string boundaries.

use ratatui::{buffer::Buffer, layout::Rect, style::Style};

pub(super) fn draw_dimensions_centered(buf: &mut Buffer, area: Rect, mut y: u16, prefix: &str, first: u32, second: u32, style: Style) -> u16 {
    let prefix_width = prefix.len() as u16;
    let dimensions_width = decimal_width(first) + 1 + decimal_width(second);
    let full_width = prefix_width + dimensions_width;

    let mut x;
    if full_width <= area.width {
        x = area.x + (area.width - full_width) / 2;

        buf.set_string(x, y, prefix, style);
        x += prefix_width;
    } else {
        let label_width = prefix.trim_ascii_end().len() as u16;

        x = area.x + area.width.saturating_sub(label_width) / 2;

        // Omit the prefix's trailing spaces when it is on its own line
        buf.set_stringn(x, y, prefix, usize::from(label_width), style);
        x = area.x + area.width.saturating_sub(dimensions_width) / 2;
        y += 1;
    }

    x = draw_u32(buf, x, y, first, style);
    buf.set_string(x, y, "x", style);
    draw_u32(buf, x + 1, y, second, style);

    y + 1
}

pub(super) fn draw_u32(buf: &mut Buffer, x: u16, y: u16, mut value: u32, style: Style) -> u16 {
    let mut digits = [0; 10];
    let mut start = digits.len();

    loop {
        start -= 1;
        digits[start] = b'0' + (value % 10) as u8;
        value /= 10;

        if value == 0 {
            break;
        }
    }

    #[allow(unsafe_code)]
    // PURPOSE: Avoiding UTF-8 validation reduces the binary size.
    // SAFETY: Every byte in `digits[start..]` was assigned a value in
    // `b'0'..=b'9'`, so the slice contains valid ASCII and therefore valid UTF-8.
    let digits_as_str = unsafe { std::str::from_utf8_unchecked(&digits[start..]) };
    buf.set_string(x, y, digits_as_str, style);

    x + (digits.len() - start) as u16
}

const fn decimal_width(mut value: u32) -> u16 {
    let mut width = 1;
    while value >= 10 {
        value /= 10;
        width += 1;
    }
    width
}

pub(super) fn draw_string_right_aligned(buf: &mut Buffer, area: Rect, text: &str, style: Style) {
    let width = text.len().min(usize::from(area.width)) as u16;
    let x = area.right() - width;

    buf.set_stringn(x, area.y, text, usize::from(area.width), style);
}

pub(super) fn draw_string_centered_wrapped(buf: &mut Buffer, area: Rect, mut y: u16, text: &str, style: Style) -> u16 {
    let bytes = text.as_bytes();

    let max_width = usize::from(area.width);
    if max_width == 0 {
        return y;
    }

    let mut start = 0;
    while start < bytes.len() && y < area.bottom() {
        while start < bytes.len() && bytes[start].is_ascii_whitespace() {
            start += 1;
        }
        if start == bytes.len() {
            break;
        }

        let mut end = (start + max_width).min(bytes.len());
        if end < bytes.len()
            && let Some(relative_break) = bytes[start..end].iter().rposition(u8::is_ascii_whitespace)
            && relative_break != 0
        {
            end = start + relative_break;
        }
        while end > start && bytes[end - 1].is_ascii_whitespace() {
            end -= 1;
        }
        if end == start {
            end = (start + max_width).min(bytes.len());
        }

        draw_string_centered(buf, Rect { y, height: 1, ..area }, &text[start..end], style);
        y += 1;

        start = end;
    }
    y
}

fn draw_string_centered(buf: &mut Buffer, area: Rect, text: &str, style: Style) {
    let width = text.len().min(usize::from(area.width)) as u16;
    let x = area.x + (area.width - width) / 2;

    buf.set_stringn(x, area.y, text, usize::from(area.width), style);
}
