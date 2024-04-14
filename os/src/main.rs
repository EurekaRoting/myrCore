#![no_std]
#![no_main] 
#![feature(panic_info_message)]
#[macro_use]
mod console;
mod lang_items;
mod sbi;
use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));
#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    init();
    log::set_max_level(LevelFilter::Info);
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        // fn boot_stack();
        // fn boot_stack_top();
    }
    log::error!("Hello, world!");
    log::warn!("Hello, world!");
    log::info!("Hello, world!");
    log::debug!("Hello, world!");
    log::trace!("Hello, world!");
    println!("Hello, world!");
    log::error!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    log::warn!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    log::info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    // println!(
    //     "boot_stack [{:#x}, {:#x})",
    //     boot_stack as usize, boot_stack_top as usize
    // );
    log::info!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    panic!("Shutdown machine!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}


// use log::{self, Level, LevelFilter, Log, Metadata, Record};
// struct SimpleLogger;
// impl Log for SimpleLogger {
//     fn enabled(&self, _metadata: &Metadata) -> bool {
//         true
//     }

//     fn log(&self, record: &Record) {
//         if !self.enabled(record.metadata()) {
//             return;
//         }
//         let color = match record.level() {
//             Level::Error => 31, // Red
//             Level::Warn => 93,  // BrightYellow
//             Level::Info => 34,  // Blue
//             Level::Debug => 32, // Green
//             Level::Trace => 90, // BrightBlack
//         };
//         println!(
//             "\u{1B}[{}m[{:>5}] {}\u{1B}[0m",
//             color,
//             record.level(),
//             record.args(),
//         );
//     }

//     fn flush(&self) {}
// }

// pub fn init() {
//     static LOGGER: SimpleLogger = SimpleLogger;
//     log::set_logger(&LOGGER).unwrap();
//     log::set_max_level(match option_env!("LOG") {
//         Some("ERROR") => LevelFilter::Error,
//         Some("WARN") => LevelFilter::Warn,
//         Some("INFO") => LevelFilter::Info,
//         Some("DEBUG") => LevelFilter::Debug,
//         Some("TRACE") => LevelFilter::Trace,
//         _ => LevelFilter::Off,
//     });
// }

use core::fmt;
//use crate::console;
use log::{self, Level, LevelFilter, Log, Metadata, Record};

pub fn init() {
    static LOGGER: Logger = Logger;
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(match option_env!("LOG") {
        Some("error") => LevelFilter::Error,
        Some("warn") => LevelFilter::Warn,
        Some("info") => LevelFilter::Info,
        Some("debug") => LevelFilter::Debug,
        Some("trace") => LevelFilter::Trace,
        _ => LevelFilter::Off,
    });
}

struct Logger;
impl Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return
        }

        print!("\x1b[{}m", level_to_color_code(record.level()));
        println!("[{}] {}", record.level(), record.args());
        print!("\x1b[0m")
    }

    fn flush(&self) {}
}

fn level_to_color_code(level: Level) -> u8 {
    match level {
        Level::Error => 31, // Red
        Level::Warn => 93,  // BrightYellow
        Level::Info => 34,  // Blue
        Level::Debug => 32, // Green
        Level::Trace => 90, // BrightBlack
    }
}


