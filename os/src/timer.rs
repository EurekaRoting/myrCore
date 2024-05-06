// //! RISC-V timer-related functionality

// use crate::config::CLOCK_FREQ;
// use crate::sbi::set_timer;
// use riscv::register::time;

// const TICKS_PER_SEC: usize = 100;
// const MSEC_PER_SEC: usize = 1000;

// /// read the `mtime` register
// pub fn get_time() -> usize {
//     time::read()
// }

// /// get current time in milliseconds
// pub fn get_time_ms() -> usize {
//     time::read() / (CLOCK_FREQ / MSEC_PER_SEC)
// }

// /// set the next timer interrupt
// pub fn set_next_trigger() {
//     set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
// }
//! RISC-V timer-related functionality

use riscv::register;

use crate::{config, sbi};

const TICKS_PER_SEC: usize = 100;
pub const MSEC_PER_SEC: usize = 1000;
pub const MICRO_PER_SEC: usize = 1_000_000;

/// read the `mtime` register
pub fn get_time() -> usize {
    register::time::read()
}

/// 获取处理器自上电以来经过的时间，单位为 `ms`
pub fn get_time_ms() -> usize {
    register::time::read() / (config::CLOCK_FREQ / MSEC_PER_SEC)
}

/// 获取处理器自上电以来经过的时间，单位为 `us`
pub fn get_time_us() -> usize {
    register::time::read() / (config::CLOCK_FREQ / MICRO_PER_SEC)
}

/// set the next timer interrupt
pub fn set_next_trigger() {
    sbi::set_timer(get_time() + config::CLOCK_FREQ / TICKS_PER_SEC);
}