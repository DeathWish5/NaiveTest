#![feature(concat_idents)]

use gettime::*;
use std::arch::asm;

const TIMES: usize = 4;

// mod coroutine;
// mod function;

const LAYER: usize = 8;

pub fn func() -> usize {
    let mut sum = 0;
    let (mut n1, mut n2, mut n3, mut n4, mut n5, mut n6, mut n7, mut n8);
    for _1 in 0..LAYER {
        n1 = 0;
        for _2 in 0..LAYER {
            n2 = 0;
            for _3 in 0..LAYER {
                n3 = 0;
                for _4 in 0..LAYER {
                    n4 = 0;
                    for _5 in 0..LAYER {
                        n5 = 0;
                        for _6 in 0..LAYER {
                            n6 = 0;
                            for _7 in 0..LAYER {
                                n7 = 0;
                                for _8 in 0..LAYER {
                                    n8 = 0;
                                    for _9 in 0..LAYER {
                                        unsafe {
                                            asm!(
                                                "add {0}, 1",
                                                inout(reg) n8,
                                                options(nostack)
                                            )
                                        }
                                    }
                                    n7 += n8;
                                }
                                n6 += n7;
                            }
                            n5 += n6;
                        }
                        n4 += n5;
                    }
                    n3 += n4;
                }
                n2 += n3;
            }
            n1 += n2;
        }
        sum = sum + n1;
    }
    sum
}

pub fn func0() -> usize {
    func_inner(8)
}

#[inline(never)]
pub fn func_inner(layer: isize) -> usize {
    if layer >= 0 {
        let mut n = 0;
        for _ in 0..LAYER {
            n += func_inner(layer - 1);
        }
        n
    } else {
        let mut n: usize = 0;
        unsafe {
            asm!(
                "mov {0}, 1",
                inout(reg) n,
                options(nostack)
            );
        }
        n
    }
}

pub fn test(f: fn() -> usize, name: &'static str) -> usize {
    let mut ret: usize = 0;
    let t1 = get_ns();
    for _ in 0..TIMES {
        ret = f();
    }
    let t2 = get_ns();
    println!(
        "{}: TIMES {} delta = {}",
        name,
        TIMES,
        (t2 - t1) / TIMES as i32
    );
    ret
}

macro_rules! coroutine_func {
    ($name:ident, $name2:ident) => {
        #[inline(never)]
        pub async fn $name() -> usize {
            let mut n: usize = 0;
            for _ in 0..LAYER {
                n += $name2().await;
            }
            n
        }
    };
}

coroutine_func!(f11, f10);
coroutine_func!(f10, f9);
coroutine_func!(f9, f8);
coroutine_func!(f8, f7);
coroutine_func!(f7, f6);
coroutine_func!(f6, f5);
coroutine_func!(f5, f4);
coroutine_func!(f4, f3);
coroutine_func!(f3, f2);
coroutine_func!(f2, f1);
coroutine_func!(f1, f0);

pub async fn coroutine() -> usize {
    f9().await
}

#[inline(never)]
pub async fn f0() -> usize {
    let mut n: usize = 0;
    unsafe {
        asm!(
            "mov {0}, 1",
            inout(reg) n,
            options(nostack)
        );
    }
    n
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let n = test(func, "BASE");
    println!("n = {}", n);
    let n = test(func0, "FUNC");
    println!("n = {}", n);
    let mut n = 0;
    let t1 = get_ns();
    for _ in 0..TIMES {
        n = coroutine().await;
    }
    let t2 = get_ns();
    println!("CORO: TIMES {} delta = {}", TIMES, (t2 - t1) / TIMES as i32);
    println!("n = {}", n);
}

// #[tokio::main(flavor = "current_thread")]
// async fn main() {
//     let n = test(func, "BASE");
//     println!("n = {}", n);
//     let n = test(func0, "FUNC");
//     println!("n = {}", n);
//     let mut n = 0;
//     let t1 = get_ns();
//     for _ in 0..TIMES {
//         n = coroutine().await;
//     }
//     let t2 = get_ns();
//     println!("CORO: TIMES {} delta = {}", TIMES, (t2 - t1) / TIMES as i32);
//     println!("n = {}", n);
// }
