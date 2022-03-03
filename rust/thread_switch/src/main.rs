use gettime::*;
use std::thread;

const TIMES: u32 = 10000000;

fn main() {
    let switch_test = move || {
        // proc_set_prio(PRIO_MAX);
        thread::yield_now();
        let start = get_ns();
        for _ in 0..TIMES {
            thread::yield_now();
        }
        let end = get_ns();
        return end - start;
    };
    let t1 = thread::spawn(switch_test);
    let t2 = thread::spawn(switch_test);
    let delta1 = t1.join().unwrap();
    let delta2 = t2.join().unwrap();
    println!(
        "TIMES {} delta1 {} delta2 {}",
        TIMES,
        delta1 / TIMES as i32,
        delta2 / TIMES as i32
    );
}
