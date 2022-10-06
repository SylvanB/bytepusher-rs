use crate::{Cpu, Memory};
use crate::ui::DisplayBuff;


pub struct System<'system> {
    cpu: Cpu<'system>,
    display: &'system mut DisplayBuff
}

impl<'system> System<'system> {
    pub fn init(mem: &'system mut Memory, display: &'system mut DisplayBuff) -> Self {
        Self {
            cpu: Cpu::init(mem),
            display,
        }
    }

    pub fn run(mut self) {
        let mut frame_count = 0;
        loop {
            self.cpu.calculate_frame_data();
            println!("[BYTE-PUSHER] frame count: {}", frame_count);
            frame_count += 1;
            let pixeldata = self.cpu.get_pixeldata_block(0x5);
            self.display.render(pixeldata);
        }
    }
}
