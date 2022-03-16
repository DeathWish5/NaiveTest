use gettime::*;
use std::thread;

const TIMES: usize = 10000000;

fn main() {
    proc_set_prio();
    let t1 = thread::spawn(move || {
        thread::yield_now();
        println!("work on buf[{}]", 0);
        let start = get_ns();
        for i in 0..TIMES {
            gettime::work_load(i, 0);
            thread::yield_now();
        }
        let end = get_ns();
        return end - start;
    });
    let t2 = thread::spawn(move || {
        thread::yield_now();
        println!("work on buf[{}]", 1);
        let start = get_ns();
        for i in 0..TIMES {
            gettime::work_load(i, 1);
            thread::yield_now();
        }
        let end = get_ns();
        return end - start;
    });
    let delta1 = t1.join().unwrap();
    let delta2 = t2.join().unwrap();
    println!(
        "TIMES {} delta1 {} delta2 {}",
        TIMES,
        delta1 / TIMES as i32,
        delta2 / TIMES as i32
    );
}
