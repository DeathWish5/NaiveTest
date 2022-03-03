use libc::sched_param;
use nix::sys::time::TimeSpec;
use nix::time::{clock_gettime, ClockId};
use numeric_enum_macro::*;
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
        sched_priority: unsafe { libc::sched_get_priority_max(libc::SCHED_RR) },
    };
    if unsafe { libc::sched_setscheduler(0, libc::SCHED_RR, &para) } != 0 {
        println!("Set scheduler failed. Plz run in root mode.");
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
