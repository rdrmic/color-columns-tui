use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};

// ============================================================================
// Block variants (Gems)
// ============================================================================
macro_rules! define_block_variants {
    ($($gem:ident => $style:expr),*) => {
        #[derive(Copy, Clone, PartialEq, Eq)]
        pub enum Gem {
            $($gem),*
        }

        impl Gem {
            pub const ALL: &[Self] = &[ $(Self::$gem),* ];
            pub const COUNT: usize = Self::ALL.len();

            pub fn random(rng: &mut fastrand::Rng) -> Self {
                Self::ALL[rng.usize(..Self::COUNT)]
            }

            /// `MurmurHash3` 64-bit mixer (fmix64).
            ///
            /// Acts as a deterministic scrambler that shatters the bit-packed seed.
            /// It utilizes the "Avalanche Effect" to ensure that minor shifts in position
            /// or time result in major shifts in color, disrupting the geometric
            /// patterns (like vertical stripes) that naturally emerge when mapping
            /// linear coordinates onto a small set of `Gem` variants.
            pub fn random_for_pause(seed: u64) -> Self {
                let mut hash = seed;
                hash ^= hash >> 33;
                hash = hash.wrapping_mul(0xf_f51_afd_7ed_558_ccd);
                hash ^= hash >> 33;
                hash = hash.wrapping_mul(0xc_4ceb_9fe_1a8_5ec_53);
                hash ^= hash >> 33;

                Self::ALL[(hash as usize) % Self::COUNT]
            }
        }

        impl From<Gem> for Style {
            fn from(gem: Gem) -> Self {
                match gem {
                    $(Gem::$gem => $style),*
                }
            }
        }
    };
}

define_block_variants!(
    Ruby     => Style::new().bg(Color::Red),
    Amber    => Style::new().bg(Color::Rgb(255, 165, 0)),
    Topaz    => Style::new().bg(Color::Yellow),
    Emerald  => Style::new().bg(Color::Green),
    Sapphire => Style::new().bg(Color::Blue),
    Amethyst => Style::new().bg(Color::Magenta)
);

// ============================================================================
// Block
// ============================================================================
#[derive(Copy, Clone)]
pub struct GemBlock {
    pub x: u8,
    pub y: i8,
    pub gem: Gem,
}

impl GemBlock {
    pub const fn new(x: u8, y: i8, gem: Gem) -> Self {
        Self { x, y, gem }
    }
}

// ============================================================================
// Widget rendering
// ============================================================================
impl Widget for GemBlock {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let x = area.x + u16::from(self.x) * 2;
        let Some(y) = area.y.checked_add_signed(i16::from(self.y)) else {
            return;
        };

        let style = Style::from(self.gem);

        if let Some(cell) = buf.cell_mut((x, y)) {
            cell.set_symbol(" ").set_style(style);
            if let Some(cell) = buf.cell_mut((x + 1, y)) {
                cell.set_symbol(" ").set_style(style);
            }
        }
    }
}
