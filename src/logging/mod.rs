pub mod file;

#[cfg(feature = "dev-console")]
pub mod dev_console;

#[cfg(not(feature = "dev-console"))]
pub mod dev_console {
    use ratatui::{
        Frame,
        crossterm::event::{KeyEvent, MouseEvent},
        layout::Rect,
    };

    #[derive(Default)]
    pub struct DevConsole {}

    #[allow(unused, clippy::unused_self, clippy::needless_pass_by_ref_mut)]
    impl DevConsole {
        pub fn gray(&mut self, _msg: String) {}
        pub fn cyan(&mut self, _msg: String) {}
        pub fn yellow(&mut self, _msg: String) {}
        pub fn red(&mut self, _msg: String) {}
        pub const fn handle_key(&mut self, _key: KeyEvent) {}
        pub const fn handle_mouse_scroll(&mut self, _mouse: MouseEvent) {}
        pub const fn draw(&mut self, _frame: &mut Frame, _area: Rect) {}
    }
}
