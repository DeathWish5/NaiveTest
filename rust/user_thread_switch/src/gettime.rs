use nix::sched::{sched_setaffinity, CpuSet};
use nix::sys::time::TimeSpec;
use nix::time::{clock_gettime, ClockId};
use nix::unistd::Pid;

#[allow(unsafe_code)]
extern crate libc;

// unlikely to fail
pub fn get_ns() -> TimeSpec {
    clock_gettime(ClockId::CLOCK_MONOTONIC).unwrap()
}

pub fn proc_set_prio() {
    let para: libc::sched_param = libc::sched_param {
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
    let cpu = unsafe { libc::sched_getcpu() };
    let mut cpu_set = CpuSet::new();
    cpu_set.set(cpu as _).unwrap();
    sched_setaffinity(Pid::from_raw(0), &cpu_set).unwrap();
    println!("Pin cpu @ {}", cpu);
}
