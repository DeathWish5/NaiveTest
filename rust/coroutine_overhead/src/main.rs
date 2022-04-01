use gettime::*;
use matrix::{
    dot, dot_stack,
    function::{fdot, fdot_stack},
    matrix_random, Matrix, N,
};

const CACHE_SIZE: usize = 384 * 1000;
const TIMES: usize = 10;

pub fn test<F: FnMut()>(mut f: F, name: &'static str) {
    let t1 = get_ns();
    for _ in 0..TIMES {
        f();
    }
    let t2 = get_ns();
    println!(
        "{}: TIMES {} delta = {}",
        name,
        TIMES,
        (t2 - t1) / TIMES as i32
    );
}

fn main() {
    proc_set_prio();
    assert!(3 * N * N > CACHE_SIZE * 2);
    let mut m1 = Matrix::new(N);
    let mut m2 = Matrix::new(N);
    let mut m3 = Matrix::new(N);
    matrix_random(&mut m1).unwrap();
    matrix_random(&mut m2).unwrap();
    test(
        || {
            dot(&m1, &m2, &mut m3).unwrap();
        },
        "BASE",
    );
    test(
        || {
            fdot(&m1, &m2, &mut m3).unwrap();
        },
        "FUNC",
    );
    test(
        || {
            dot_stack(&m1, &m2, &mut m3).unwrap();
        },
        "BASE STACK",
    );
    test(
        || {
            fdot_stack(&m1, &m2, &mut m3).unwrap();
        },
        "FUNC STACK",
    );
}
