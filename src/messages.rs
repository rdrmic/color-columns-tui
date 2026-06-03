use ratatui::style::Color;

pub struct Message {
    text: &'static str,
    rgb: [u8; 3],
    opacity_percent: u8,
    num_ticks_while_opaque: u8,
    fade_percent_per_tick: u8,
}

impl Message {
    pub const fn new_fading(text: &'static str, rgb: [u8; 3], num_ticks_while_opaque: u8, fade_percent_per_tick: u8) -> Self {
        Self { text, rgb, opacity_percent: 100, num_ticks_while_opaque, fade_percent_per_tick }
    }

    pub const fn new_permanent(text: &'static str, rgb: [u8; 3]) -> Self {
        Self { text, rgb, opacity_percent: 100, num_ticks_while_opaque: 0, fade_percent_per_tick: 0 }
    }

    pub const fn text(&self) -> &'static str {
        self.text
    }

    pub fn color(&self) -> Color {
        let opacity_percent = u16::from(self.opacity_percent);

        let [r, g, b] = self.rgb.map(|channel| ((u16::from(channel) * opacity_percent) / 100) as u8);
        Color::Rgb(r, g, b)
    }

    pub const fn tick(&mut self) -> bool {
        if self.fade_percent_per_tick == 0 {
            return true;
        }

        if self.num_ticks_while_opaque > 0 {
            self.num_ticks_while_opaque -= 1;
            return true;
        }

        if self.opacity_percent > self.fade_percent_per_tick {
            self.opacity_percent -= self.fade_percent_per_tick;
            true
        } else {
            false
        }
    }
}
