#![feature(async_closure)]
use spin::Mutex;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::{Context, Poll};
use nix::sys::time::TimeSpec;
use matrix::{*, function::*, coroutine::l1 as cl1};
use coroutine_switch::*;
use gettime::*;

pub fn spawn_l1() {
    for i in 0..N {
        spawn(l2_coroutine(i))
    }
}

pub fn spawn_l2() {
    for i in 0..N {
        for j in 0..N {
            spawn(l3_coroutine(i, j));
        }
    }
}

pub async fn l1_coroutine() {
    let m1 = M1.lock();
    let m2 = M2.lock();
    let mut m3 = M3.lock();
    let NN = m1.n;
    cl1(NN, &m1, &m2, &mut m3).await;
}

pub async fn l2_coroutine(i: usize) {
    let nn: usize = N;
    for j in 0..nn {
        l3_coroutine(i, j).await;
        let y = YieldOnce {
            y: AtomicBool::new(false),
        };
        y.await;
    }
}

pub async fn l3_coroutine(i: usize, j: usize) {
    let nn: usize = N;
    let m1 = M1.lock();
    let m2 = M2.lock();
    let mut m3 = M3.lock();
    l3(nn, i, j, &m1, &m2, &mut m3);
}

struct YieldOnce {
    y: AtomicBool,
}

impl Future for YieldOnce {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        if self.y.fetch_xor(true, Ordering::Relaxed) == true {
            return Poll::Ready(());
        }
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

lazy_static::lazy_static! {
    static ref M1: Mutex<Matrix> = Mutex::new(Matrix::new(N));
    static ref M2: Mutex<Matrix> = Mutex::new(Matrix::new(N));
    static ref M3: Mutex<Matrix> = Mutex::new(Matrix::new(N));
}

const TIMES: usize = 50;
pub fn zero() -> TimeSpec {
    TimeSpec::from(std::time::Duration::from_secs(0))
}

pub fn test(f: fn(), _name: &'static str) -> TimeSpec {
    let t1 = get_ns();
    f();
    let t2 = get_ns();
    t2 - t1
}

fn main() {
    proc_set_prio();
    let mut m1 = M1.lock();
    matrix_random(&mut m1).unwrap();
    let mut m2 = M2.lock();
    matrix_random(&mut m2).unwrap();
    drop(m1);
    drop(m2);

    // let mut ave: TimeSpec = zero();
    // for _ in 0..TIMES {
    //     spawn(l1_coroutine());
    //     ave = ave + test(run_until_idle, "1 c");
    // }
    // println!("1 coroutines delta = {}", ave / TIMES as _);

    // let mut ave: TimeSpec = zero();
    // for _ in 0..TIMES {
    //     spawn_l1();
    //     ave = ave + test(run_until_idle, "1000 c");
    // }
    // println!("1000 coroutines delta = {}", ave / TIMES as _);
    
    
    let mut ave: TimeSpec = zero();
    for _ in 0..TIMES {
        spawn_l2();
        ave = ave + test(run_until_idle, "1000 * 1000 c");
    }
    println!("1000*1000 threads delta = {}", ave / TIMES as _);
}