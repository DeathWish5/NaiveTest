use gettime::*;
use nix::sys::time::TimeSpec;
use spin::Mutex;
use std::future::Future;
use std::mem;
use std::pin::Pin;
use std::task::{Context, Poll};

const TIMES: usize = 10000000;

struct MyCountor {
    id: usize,
    count: usize,
    start: mem::MaybeUninit<TimeSpec>,
}

impl MyCountor {
    fn new(id: usize) -> Self {
        MyCountor {
            id: id,
            count: 0,
            start: mem::MaybeUninit::uninit(),
        }
    }
}

const NUM: usize = 16;
const BUF_SIZE: usize = 64 * 1024;

lazy_static::lazy_static! {
    static ref SIG: Mutex<usize> = Mutex::new(0);
    static ref BUFS: [Mutex<[u8; BUF_SIZE]>; 16] = [
        Mutex::new([0; BUF_SIZE]),
        Mutex::new([0; BUF_SIZE]),
        Mutex::new([0; BUF_SIZE]),
        Mutex::new([0; BUF_SIZE]),
        Mutex::new([0; BUF_SIZE]),
        Mutex::new([0; BUF_SIZE]),
        Mutex::new([0; BUF_SIZE]),
        Mutex::new([0; BUF_SIZE]),
        Mutex::new([0; BUF_SIZE]),
        Mutex::new([0; BUF_SIZE]),
        Mutex::new([0; BUF_SIZE]),
        Mutex::new([0; BUF_SIZE]),
        Mutex::new([0; BUF_SIZE]),
        Mutex::new([0; BUF_SIZE]),
        Mutex::new([0; BUF_SIZE]),
        Mutex::new([0; BUF_SIZE]),
    ];
}

fn init_buf() {
    let mut random: usize = 123456487;
    for i in 0..NUM {
        let mut buf = BUFS[i].lock();
        for n in buf.iter_mut() {
            *n = random as u8;
            random = random * 10001 + 7;
        }
    }
}

impl Future for MyCountor {
    type Output = TimeSpec;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let inner = self.get_mut();
        if inner.count == 0 {
            inner.start.write(get_ns());
            println!("work on buf[{}]", inner.id);
        }
        let mut sig = SIG.lock();
        if (*sig % NUM) == inner.id {
            *sig += 1;
            let ncount = inner.count + 1;
            let _ = mem::replace(&mut inner.count, ncount);
            let buf = BUFS[inner.id].lock();
            let _ = buf.iter().fold(0, |acc, x| acc + x);
            drop(buf);
        }
        drop(sig);
        if inner.count >= TIMES {
            Poll::Ready(get_ns() - unsafe { inner.start.assume_init() })
        } else {
            cx.waker().clone().wake();
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    proc_set_prio();
    init_buf();
    let c1 = MyCountor::new(0);
    let c2 = MyCountor::new(1);
    let c3 = MyCountor::new(2);
    let c4 = MyCountor::new(3);
    let c5 = MyCountor::new(4);
    let c6 = MyCountor::new(5);
    let c7 = MyCountor::new(6);
    let c8 = MyCountor::new(7);
    let c9 = MyCountor::new(8);
    let c10 = MyCountor::new(9);
    let c11 = MyCountor::new(10);
    let c12 = MyCountor::new(11);
    let c13 = MyCountor::new(12);
    let c14 = MyCountor::new(13);
    let c15 = MyCountor::new(14);
    let c16 = MyCountor::new(15);
    let (delta1, delta2, ..) =
        tokio::join!(c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16);
    println!(
        "TIMES {} delta1 {} delta2 {}",
        TIMES,
        delta1 / TIMES as i32 / NUM as i32,
        delta2 / TIMES as i32 / NUM as i32,
    );
}
