use matrix::{function::*, *};
use spin::Mutex;
use user_thread_switch::*;

#[inline(never)]
pub fn spawn_l1() {
    for i in 0..N {
        spawn(l2_thread, i)
    }
}

#[inline(never)]
pub fn spawn_l1_stack() {
    for i in 0..N {
        spawn(l2_thread_stack, i)
    }
}

#[inline(never)]
pub fn l2_thread(i: usize) {
    let nn: usize = N;
    for j in 0..nn {
        let m1 = M1.lock();
        let m2 = M2.lock();
        let mut m3 = M3.lock();
        l3(nn, i, j, &m1, &m2, &mut m3);
        drop(m1);
        drop(m2);
        drop(m3);
        sched_yield();
    }
    exit();
}

#[inline(never)]
pub fn l2_thread_stack(i: usize) {
    let nn: usize = N;
    for j in 0..nn {
        let m1 = M1.lock();
        let m2 = M2.lock();
        let mut m3 = M3.lock();
        l3_stack(nn, i, j, &m1, &m2, &mut m3);
        drop(m1);
        drop(m2);
        drop(m3);
        sched_yield();
    }
    exit();
}

#[inline(never)]
pub fn spawn_l2() {
    for i in 0..N {
        for j in 0..N {
            spawn(l3_thread, i << 32 | j);
        }
    }
}

#[inline(never)]
pub fn spawn_l2_stack() {
    for i in 0..N {
        for j in 0..N {
            spawn(l3_thread_stack, i << 32 | j);
        }
    }
}

#[inline(never)]
pub fn l3_thread(idx: usize) {
    let i = idx >> 32;
    let j = idx & 0xFFFFFFFF;
    let nn: usize = N;
    let m1 = M1.lock();
    let m2 = M2.lock();
    let mut m3 = M3.lock();
    l3(nn, i, j, &m1, &m2, &mut m3);
    drop(m1);
    drop(m2);
    drop(m3);
    exit();
}

#[inline(never)]
pub fn l3_thread_stack(idx: usize) {
    let i = idx >> 32;
    let j = idx & 0xFFFFFFFF;
    let nn: usize = N;
    let m1 = M1.lock();
    let m2 = M2.lock();
    let mut m3 = M3.lock();
    l3_stack(nn, i, j, &m1, &m2, &mut m3);
    drop(m1);
    drop(m2);
    drop(m3);
    exit();
}

lazy_static::lazy_static! {
    static ref M1: Mutex<Matrix> = Mutex::new(Matrix::new(N));
    static ref M2: Mutex<Matrix> = Mutex::new(Matrix::new(N));
    static ref M3: Mutex<Matrix> = Mutex::new(Matrix::new(N));
}

use nix::sys::time::TimeSpec;

pub fn test(f: fn(), _name: &'static str) -> TimeSpec {
    let t1 = get_ns();
    f();
    let t2 = get_ns();
    t2 - t1
}

const TIMES: usize = 1;

pub fn zero() -> TimeSpec {
    TimeSpec::from(std::time::Duration::from_secs(0))
}

fn main() {
    proc_set_prio();
    let mut m1 = M1.lock();
    matrix_random(&mut m1).unwrap();
    let mut m2 = M2.lock();
    matrix_random(&mut m2).unwrap();
    drop(m1);
    drop(m2);

    let mut ave: TimeSpec = zero();
    for _ in 0..TIMES {
        spawn_l1();
        ave = ave + test(run_until_idle, "1000 threads");
    }
    println!("1000 threads delta = {}", ave / TIMES as _);

    let mut ave: TimeSpec = zero();
    for _ in 0..TIMES {
        spawn_l2();
        ave = ave + test(run_until_idle, "1000 * 1000 threads");
    }
    println!("1000*1000 threads delta = {}", ave / TIMES as _);

    // will segment fault ....

    // let mut ave: TimeSpec = zero();
    // for _ in 0..TIMES {
    //     spawn_l1_stack();
    //     ave = ave + test(run_until_idle, "1000 threads");
    // }
    // println!("1000 threads delta = {}", ave / TIMES as _);

    // let mut ave: TimeSpec = zero();
    // for _ in 0..TIMES {
    //     spawn_l2_stack();
    //     ave = ave + test(run_until_idle, "1000 * 1000 threads");
    // }
    // println!("1000*1000 threads delta = {}", ave / TIMES as _);
}
