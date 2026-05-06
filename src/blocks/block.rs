use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};

// ============================================================================
// Block Variants (Gems)
// ============================================================================
// TODO Change the colors
// TODO Rename `Gem`
// FIXME Use `Color::Red` style or `Color::Indexed(0..255)` throughout the app
const STYLE_RUBY: Style = Style::new().bg(Color::Red);
const STYLE_AMBER: Style = Style::new().bg(Color::Rgb(255, 165, 0));
const STYLE_TOPAZ: Style = Style::new().bg(Color::Yellow);
const STYLE_EMERALD: Style = Style::new().bg(Color::Green);
const STYLE_SAPPHIRE: Style = Style::new().bg(Color::Blue);
const STYLE_AMETHYST: Style = Style::new().bg(Color::Magenta);

macro_rules! define_block_variants {
    ($($gem:ident => $style:ident),*) => {
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub enum Gem {
            $($gem),*
        }

        impl Gem {
            pub const ALL: &[Self] = &[ $(Self::$gem),* ];
            pub const COUNT: usize = Self::ALL.len();

            pub fn random(rng: &mut fastrand::Rng) -> Self {
                Self::ALL[rng.usize(..Self::COUNT)]
            }

            pub fn random_for_pause(seed: u64) -> Self {
                // MurmurHash3 64-bit mixer (fmix64).
                //
                // Acts as a deterministic scrambler that shatters the bit-packed seed.
                // It utilizes the "Avalanche Effect" to ensure that minor shifts in position
                // or time result in major shifts in color, disrupting the geometric
                // patterns (like vertical stripes) that naturally emerge when mapping
                // linear coordinates onto a small set of Gem variants.
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
    Ruby     => STYLE_RUBY,
    Amber    => STYLE_AMBER,
    Topaz    => STYLE_TOPAZ,
    Emerald  => STYLE_EMERALD,
    Sapphire => STYLE_SAPPHIRE,
    Amethyst => STYLE_AMETHYST
);

// ============================================================================
// Block
// ============================================================================
pub struct Block {
    x: u8,
    y: i8,
    gem: Gem,
}

impl Block {
    pub const fn new(x: u8, y: i8, gem: Gem) -> Self {
        Self { x, y, gem }
    }
}

// ============================================================================
// Widget rendering
// ============================================================================
impl Widget for &Block {
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
