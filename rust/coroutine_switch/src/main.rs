use gettime::*;
use nix::sys::time::TimeSpec;
use spin::Mutex;
use std::future::Future;
use std::mem;
use std::pin::Pin;
use std::task::{Context, Poll};

mod executor;
use executor::*;

const MAX_TASKS: usize = 1000 * 1000;

const NUM: usize = 384;
const TIMES: usize = 100;
const CACHE_SIZE: usize = 384 * 1024;

const BUF_SIZE: usize = CACHE_SIZE / NUM * 10;

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

lazy_static::lazy_static! {
    static ref SIG: Mutex<usize> = Mutex::new(0);
    // static ref BUFS: [Mutex<[u8; BUF_SIZE]>; 16] = [
    //     Mutex::new([0; BUF_SIZE]),
    //     Mutex::new([0; BUF_SIZE]),
    //     Mutex::new([0; BUF_SIZE]),
    //     Mutex::new([0; BUF_SIZE]),
    //     Mutex::new([0; BUF_SIZE]),
    //     Mutex::new([0; BUF_SIZE]),
    //     Mutex::new([0; BUF_SIZE]),
    //     Mutex::new([0; BUF_SIZE]),
    //     Mutex::new([0; BUF_SIZE]),
    //     Mutex::new([0; BUF_SIZE]),
    //     Mutex::new([0; BUF_SIZE]),
    //     Mutex::new([0; BUF_SIZE]),
    //     Mutex::new([0; BUF_SIZE]),
    //     Mutex::new([0; BUF_SIZE]),
    //     Mutex::new([0; BUF_SIZE]),
    //     Mutex::new([0; BUF_SIZE]),
    // ];
}

// fn init_buf() {
//     let mut random: usize = 123456487;
//     for i in 0..NUM {
//         let mut buf = BUFS[i].lock();
//         for n in buf.iter_mut() {
//             *n = random as u8;
//             random = random * 10001 + 7;
//         }
//     }
// }

impl Future for MyCountor {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let inner = self.get_mut();
        if inner.count == 0 {
            inner.start.write(get_ns());
            // println!("work on buf[{}]", inner.id);
        }
        let mut sig = SIG.lock();
        if (*sig % NUM) == inner.id {
            *sig += 1;
            let ncount = inner.count + 1;
            let _ = mem::replace(&mut inner.count, ncount);
            let mut buf = [0; BUF_SIZE];
            for (idx, c) in buf.iter_mut().enumerate() {
                *c = idx & 0xF;
            }
            let _total = buf.iter().fold(0, |acc, x| (acc + x) & 0xFFFF);
            // let buf = BUFS[inner.id].lock();
            // let _ = buf.iter().fold(0, |acc, x| acc + x);
            // drop(buf);
        }
        drop(sig);
        if inner.count >= TIMES {
            let delta = get_ns() - unsafe { inner.start.assume_init() };
            println!("TIMES {} delta = {}", TIMES, delta / TIMES as i32);
            Poll::Ready(())
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

fn main() {
    proc_set_prio();
    // init_buf();
    for i in 0..NUM {
        let c = MyCountor::new(i);
        executor::spawn(c);
    }
    executor::run_until_idle();
}
