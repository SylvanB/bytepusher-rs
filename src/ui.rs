use minifb::{Window, WindowOptions};
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct DisplayBuff {
    window: Window,
    palette: Vec<u32>
}

impl DisplayBuff {
    pub fn init() -> Self {
        Self {
            window: Window::new(
                ".: BYTE-PUSHER :.",
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                WindowOptions::default()
                ).unwrap_or_else(|e| {
                    panic!("{}", e);
                }),
            palette: DisplayBuff::build_palette()
        }
    }

    pub fn build_palette() -> Vec<u32> {
        let mut palette = vec![0; 256];

        let mut i = 0;
        for r in (0..=0xff).step_by(0x33) {
            for g in (0..=0xff).step_by(0x33) {
                for b in (0..=0xff).step_by(0x33) {
                    palette[i] = 0xff000000 | b << 16 | g << 8 | r;
                    i += 1;
                    println!("[BYTE-PUSHER] {} R: {:x} G: {:x} B: {:x}", i, r << 16, g << 8, b);
                }
            }
        }

        println!("[BYTE-PUSHER] i == {}", i);
        while i <= 255 {
            palette[i] = 0xff000000;
            i += 1;
        }

        palette
    }

    pub fn render(&mut self, buffer: &[u8]) {
        let converted: Vec<u32> = buffer.iter().map(|a| self.palette[*a as usize]).collect();
        // self.window.update_with_buffer()
        self.window.update_with_buffer(&converted, SCREEN_WIDTH, SCREEN_HEIGHT).unwrap();
    }
}