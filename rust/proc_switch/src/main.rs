use nix::sched::sched_yield;
use nix::unistd::{fork, ForkResult};

use gettime::*;

const TIMES: u64 = 10000000;

fn swtich_test(name: &'static str) {
    proc_set_prio();
    sched_yield().unwrap();
    let start = get_ns();
    for _ in 0..TIMES {
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
    match unsafe { fork() } {
        Ok(ForkResult::Parent { .. }) => swtich_test("F"),
        Ok(ForkResult::Child) => swtich_test("C"),
        Err(_) => println!("Fork failed"),
    }
}
