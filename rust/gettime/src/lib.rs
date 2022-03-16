use libc::sched_param;
use nix::sys::time::TimeSpec;
use nix::time::{clock_gettime, ClockId};
use numeric_enum_macro::*;
use spin::Mutex;
use std::io::Write;
#[allow(unsafe_code)]
extern crate libc;

#[macro_use]
extern crate log;

// numeric_enum! {
//     #[repr(u32)]
//     #[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
//     enum PRIO_WHICH {
//         PRIO_PROCESS = 0,
//         PRIO_PGRP = 1,
//         PRIO_USER = 2,
//     }
// }

// unlikely to fail
pub fn get_ns() -> TimeSpec {
    clock_gettime(ClockId::CLOCK_MONOTONIC).unwrap()
}

pub fn proc_set_prio() {
    let para: sched_param = sched_param {
        sched_priority: unsafe { libc::sched_get_priority_max(libc::SCHED_FIFO) },
    };
    if unsafe { libc::sched_setscheduler(0, libc::SCHED_FIFO, &para) } != 0 {
        println!("Set scheduler failed. Plz run in root mode.");
    }
    println!(
        "scheduler = {}, priority = {}",
        "SCHED_FIFO", para.sched_priority
    );
    pin_cpu();
}

pub fn pin_cpu() {
    use nix::sched::{sched_setaffinity, CpuSet};
    use nix::unistd::Pid;
    let cpu = unsafe { libc::sched_getcpu() };
    let mut cpu_set = CpuSet::new();
    cpu_set.set(cpu as _).unwrap();
    sched_setaffinity(Pid::from_raw(0), &cpu_set).unwrap();
    println!("Pin cpu @ {}", cpu);
}

const BUF_SIZE: usize = 0x100 * 64;

lazy_static::lazy_static! {
    static ref BUF0: Mutex<[u8; BUF_SIZE]> = Mutex::new([0; BUF_SIZE]);
    static ref BUF1: Mutex<[u8; BUF_SIZE]> = Mutex::new([0; BUF_SIZE]);
}

pub fn work_load(idx: usize, id: usize) {
    let mut buf = if id == 0 { BUF0.lock() } else { BUF1.lock() };
    let idx = idx % 64;
    for i in 0..0x100 {
        buf[idx * 0x100 + i] += 1;
    }
}

/// init the env_logger
pub fn init_logger() {
    env_logger::builder()
        .format(|buf, record| {
            use env_logger::fmt::Color;
            use log::Level;

            let tid = 0;
            let mut style = buf.style();
            match record.level() {
                Level::Trace => style.set_color(Color::Black).set_intense(true),
                Level::Debug => style.set_color(Color::White),
                Level::Info => style.set_color(Color::Green),
                Level::Warn => style.set_color(Color::Yellow),
                Level::Error => style.set_color(Color::Red).set_bold(true),
            };
            let level = style.value(record.level());
            writeln!(buf, "[{:>5}][{}] {}", level, tid, record.args())
        })
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gettime() {
        init_logger();
        proc_set_prio();
        let time1 = get_ns();
        let time2 = get_ns();
        println!("diff = {}", time2 - time1);
    }
}
