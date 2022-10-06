use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
use minifb::{Window, WindowOptions};
use cpu::Cpu;
use memory::Memory;
use system::System;
use crate::ui::DisplayBuff;
use clap::{load_yaml, App};

mod memory;
mod cpu;
mod system;
mod ui;

const SCREEN_HEIGHT: usize = 256;
const SCREEN_WIDTH: usize = 256;

fn main() {
    println!(".: BytePusher.rs :.");

    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let img_path = matches.value_of("INPUT").unwrap();

    let mut mem = init_memory_from_disk_image(img_path);
    let mut display_buff = DisplayBuff::init();
    let sys = System::init(&mut mem, &mut display_buff);
    sys.run();
}

fn init_memory_from_disk_image(path: &str)  -> Memory {
    let disk = fs::read(path).unwrap();
    Memory::init_from_disk_img(disk)
}