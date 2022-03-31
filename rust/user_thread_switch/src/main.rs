#![feature(naked_functions)]

use spin::Mutex;

mod gettime;
mod runtime;

use gettime::*;
use runtime::*;

// used for runtime
const DEFAULT_STACK_SIZE: usize = BUF_SIZE * 10;
const MAX_TASKS: usize = NUM;
// Some constants config
const NUM: usize = 384;
const TIMES: usize = 100;
const CACHE_SIZE: usize = 384 * 1024;
const BUF_SIZE: usize = CACHE_SIZE / NUM * 10;

lazy_static::lazy_static! {
    static ref SIG: Mutex<usize> = Mutex::new(0);
}

fn thread_fn(arg: usize) {
    let id = arg;
    let mut count = 0;
    let start = get_ns();
    loop {
        let mut sig = SIG.lock();
        if (*sig % NUM) == id {
            *sig += 1;
            count += 1;
            let mut buf = [0; BUF_SIZE];
            for (idx, c) in buf.iter_mut().enumerate() {
                *c = idx & 0xF;
            }
            let _total = buf.iter().fold(0, |acc, x| (acc + x) & 0xFFFF);
        }
        drop(sig);
        if count >= TIMES {
            let end = get_ns();
            println!("TIMES {} delta = {}", TIMES, (end - start) / TIMES as i32);
            exit();
        } else {
            sched_yield();
        }
    }
}

fn main() {
    proc_set_prio();
    for i in 0..NUM {
        spawn(thread_fn, i);
    }
    run_until_idle();
}
