#![feature(prelude_import)]
#![feature(concat_idents)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;

use gettime::*;
use std::arch::asm;

const CACHE_SIZE: usize = 384 * 1000;
const N: usize = 1000;
const TIMES: usize = 1;

// mod coroutine;
// mod function;

const LAYER: usize = 5;

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
                                            asm!("add {0}, 1", inout(reg) n8, options(nostack))
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
        sum = (sum + n1) & 0xFFFFFFFF;
    }
    sum
}

pub fn func0() -> usize { func_inner(8) }

#[inline(never)]
pub fn func_inner(layer: isize) -> usize {
    if layer >= 0 {
            let mut n = 0;
            for _ in 0..LAYER { n += func_inner(layer - 1); }
            n
        } else {
           let mut n: usize = 0;
           unsafe { asm!("mov {0}, 1", inout(reg) n, options(nostack)); }
           n
       }
}

pub fn test(f: fn() -> usize, name: &'static str) -> usize {
    let mut ret: usize = 0;
    let t1 = get_ns();
    for _ in 0..TIMES { ret = f(); }
    let t2 = get_ns();





    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(&["", ": TIMES ",
                            " delta = ", "\n"],
                &[::core::fmt::ArgumentV1::new_display(&name),
                            ::core::fmt::ArgumentV1::new_display(&TIMES),
                            ::core::fmt::ArgumentV1::new_display(&((t2 - t1) /
                                        TIMES as i32))]));
    };
    ret
}
macro_rules! coroutine_func {
    ($name : ident, $name2 : ident) =>
    {
        #[inline(never)] pub async fn concat_idents! (cor_inner_, $name) () ->
        usize
        {
            let mut n : usize = 0 ; for _ in 0 .. LAYER
            { n += concat_idents! (cor_inner_, $name2) ().await ; } n
        }
    } ;
}
pub async fn coroutine() -> usize { cor_inner_f8().await }
#[inline(never)]
pub async fn cor_inner_f0() -> usize { 1 }
fn main() {
    let n = test(func, "BASE");
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(&["n = ", "\n"],
                &[::core::fmt::ArgumentV1::new_display(&n)]));
    };
    let n = test(func0, "FUNC");
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(&["n = ", "\n"],
                &[::core::fmt::ArgumentV1::new_display(&n)]));
    };
}
