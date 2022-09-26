use minifb::{Window, WindowOptions};
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct Display {
    window: Window,
}

impl Display {
    pub fn init() -> Self {
        Self {
            window: Window::new(
                ".: BYTE-PUSHER :.",
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                WindowOptions::default()
                ).unwrap_or_else(|e| {
                    panic!("{}", e);
                })
        }
    }

    pub fn render(&mut self, buffer: &Vec<u32>) {
        self.window.update_with_buffer(buffer, SCREEN_WIDTH, SCREEN_HEIGHT).unwrap();
    }
}