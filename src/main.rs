use std::cell::RefCell;
use std::rc::Rc;
use minifb::{Window, WindowOptions};
use cpu::Cpu;
use memory::Memory;
use system::System;

mod memory;
mod cpu;
mod system;
mod ui;

const SCREEN_HEIGHT: usize = 256;
const SCREEN_WIDTH: usize = 256;


fn main() {
    println!(".: BytePusher.rs :.");

    let window: Rc<RefCell<_>> = Rc::new(RefCell::new(
        Window::new(
            ".: BytePusher.rs :.",
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            WindowOptions {
                scale: minifb::Scale::X8,
                scale_mode: minifb::ScaleMode::Stretch,
                ..WindowOptions::default()
            },
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        }),
    ));

    let sys = System::init();
    sys.run();
}
