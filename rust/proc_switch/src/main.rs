use nix::sched::sched_yield;
use nix::unistd::{fork, ForkResult};

use gettime::*;

const TIMES: u64 = 10000000;

fn swtich_test(name: &'static str) {
    let id = if name == "F" { 0 } else { 1 };
    sched_yield().unwrap();
    println!("work on buf[{}]", id);
    let start = get_ns();
    for i in 0..TIMES {
        gettime::work_load(i as _, id);
        sched_yield().unwrap();
    }
    let end = get_ns();
    println!(
        "{} TIMES {} delta {}",
        name,
        TIMES,
        (end - start) / TIMES as i32
    );
}

fn main() {
    init_logger();
    proc_set_prio();
    match unsafe { fork() } {
        Ok(ForkResult::Parent { .. }) => swtich_test("F"),
        Ok(ForkResult::Child) => swtich_test("C"),
        Err(_) => println!("Fork failed"),
    }
}
