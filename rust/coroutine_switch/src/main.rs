
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use nix::sys::time::TimeSpec;
use gettime::*;
use std::mem;

const TIMES: u32 = 10000000;

struct MyCountor {
    count: u32,
    start: mem::MaybeUninit<TimeSpec>,
}

impl Default for MyCountor {
    fn default() -> Self {
        MyCountor {
            count: 0,
            start: mem::MaybeUninit::uninit(),
        }
    }
}

impl Future for MyCountor {
    type Output = TimeSpec;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let inner = self.get_mut();
        if inner.count == 0 {
            inner.start.write(get_ns());
        }
        let ncount = inner.count + 1;
        let _ = mem::replace(&mut inner.count, ncount);
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
    let c1 = MyCountor::default();
    let c2 = MyCountor::default();
    let (delta1, delta2) = tokio::join!(c1, c2);
    println!(
        "TIMES {} delta1 {} delta2 {}",
        TIMES,
        delta1 / TIMES as i32,
        delta2 / TIMES as i32
    );
}
