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
        #[derive(Clone, Copy, PartialEq, Eq)]
        pub enum Gem {
            $($gem),*
        }

        impl Gem {
            pub const ALL: &[Self] = &[ $(Self::$gem),* ];
            pub const COUNT: usize = Self::ALL.len();

            pub fn random(rng: &mut fastrand::Rng) -> Self {
                Self::ALL[rng.usize(..Self::COUNT)]
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
