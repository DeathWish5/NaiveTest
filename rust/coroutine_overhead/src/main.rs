use gettime::*;
use std::result::Result;

pub struct Matrix {
    pub data: Box<[usize]>,
    pub n: usize,
}

impl Matrix {
    pub fn new(n: usize) -> Self {
        Matrix {
            data: {
                let data = vec![0usize; N * N];
                data.into_boxed_slice()
            },
            n,
        }
    }
}

pub fn random() -> usize {
    static mut seed: usize = 123456;
    unsafe {
        seed = (seed * 10007 + 3) & 0xFFFFFFFF;
        seed
    }
}

pub fn matrix_dot_plus(m1: &Matrix, m2: &Matrix, m3: &mut Matrix) -> Result<(), &'static str> {
    if m1.n != m2.n || m1.n != m3.n {
        return Err("matrix not aligned");
    }
    let NN = m1.n;
    for i in 0..NN {
        for j in 0..NN {
            let mut sum: usize = 0;
            for k in 0..NN {
                sum += (m1.data[i * NN + k] & 0xFFFF) * (m2.data[k * NN + j] & 0xFFFF);
            }
            m3.data[i * NN + j] = (m3.data[i * NN + j] & 0xFFFF) + (sum & 0xFFFF);
        }
    }
    Ok(())
}

pub fn matrix_random(m: &mut Matrix) -> Result<(), &'static str> {
    if m.data.len() < m.n * m.n {
        return Err("matrix data size invalid");
    }
    let NN = m.n;
    for i in 0..NN {
        for j in 0..NN {
            m.data[i * NN + j] = random();
        }
    }
    Ok(())
}

const CACHE_SIZE: usize = 384 * 1000;
const N: usize = 1000;
const TIMES: usize = 50;

mod coroutine;
mod function;

use coroutine::{dot1 as dot21, dot2 as dot22};
use function::{dot_fast as dot2, matrix_dot_plus as dot1};

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

// #[tokio::main(flavor = "current_thread")]
// async fn main() {
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
            dot2(&m1, &m2, &mut m3).unwrap();
        },
        "FUNC2",
    );
    test(
        || {
            matrix_dot_plus(&m1, &m2, &mut m3).unwrap();
        },
        "BASE",
    );
    // test(
    //     || {
    //         dot1(&m1, &m2, &mut m3).unwrap();
    //     },
    //     "FUNC",
    // );
    // let t1 = get_ns();
    // for _ in 0..TIMES {
    //     dot21(&m1, &m2, &mut m3).await.unwrap();
    // }
    // let t2 = get_ns();
    // println!(
    //     "COROUTINE 1: TIMES {} delta = {}",
    //     TIMES,
    //     (t2 - t1) / TIMES as i32
    // );
    // let t1 = get_ns();
    // for _ in 0..TIMES {
    //     dot22(&m1, &m2, &mut m3).await.unwrap();
    // }
    // let t2 = get_ns();
    // println!(
    //     "COROUTINE 2: TIMES {} delta = {}",
    //     TIMES,
    //     (t2 - t1) / TIMES as i32
    // );
}
