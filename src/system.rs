use crate::Cpu;

pub struct System {
    cpu: Cpu
}

impl<'a> System {
    pub fn init() -> Self {
        Self {
            cpu: Cpu::init()
        }
    }

    pub fn run(self) {
        let mut frame_count = 0;
        loop {
            self.cpu.calculate_frame_data();
            println!("[BYTE-PUSHER] frame count: {}", frame_count);
            frame_count += 1;
        }
    }
}
