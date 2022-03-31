use spin::Mutex;
use std::result::Result;
use user_thread_switch::*;
type MyResult = Result<(), &'static str>;

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
    static mut SEED: usize = 123456;
    unsafe {
        SEED = (SEED * 10007 + 3) & 0xFFFFFFFF;
        SEED
    }
}

pub fn matrix_random(m: &mut Matrix) -> Result<(), &'static str> {
    if m.data.len() < m.n * m.n {
        return Err("matrix data size invalid");
    }
    let nn = m.n;
    for i in 0..nn {
        for j in 0..nn {
            m.data[i * nn + j] = random();
        }
    }
    Ok(())
}

#[inline(never)]
pub fn l3(nn: usize, i: usize, j: usize, m1: &Matrix, m2: &Matrix, m3: &mut Matrix) {
    let mut sum: usize = 0;
    for k in 0..nn {
        sum = sum + (m1.data[i * nn + k] & 0xFFFF) * (m2.data[k * nn + j] & 0xFFFF);
    }
    m3.data[i * nn + j] = (m3.data[i * nn + j] & 0xFFFF) + (sum & 0xFFFF);
}

#[inline(never)]
pub fn l2(nn: usize, i: usize, m1: &Matrix, m2: &Matrix, m3: &mut Matrix) {
    for j in 0..nn {
        l3(nn, i, j, m1, m2, m3);
    }
}

#[inline(never)]
pub fn l1(nn: usize, m1: &Matrix, m2: &Matrix, m3: &mut Matrix) {
    for i in 0..nn {
        l2(nn, i, m1, m2, m3);
    }
}

#[inline(never)]
pub fn dot(m1: &Matrix, m2: &Matrix, m3: &mut Matrix) -> MyResult {
    size_check(m1, m2, m3)?;
    let nn = m1.n;
    l1(nn, m1, m2, m3);
    Ok(())
}

#[inline(never)]
pub fn size_check(m1: &Matrix, m2: &Matrix, m3: &Matrix) -> MyResult {
    if m1.n != m2.n || m1.n != m3.n {
        return Err("matrix not aligned");
    }
    Ok(())
}

#[inline(never)]
pub fn spawn_l1() {
    for i in 0..N {
        spawn(l2_thread, i)
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
pub fn spawn_l2() {
    for i in 0..N {
        for j in 0..N {
            spawn(l3_thread, i << 32 | j);
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
    let mut sum: usize = 0;
    for k in 0..nn {
        sum = sum + (m1.data[i * nn + k] & 0xFFFF) * (m2.data[k * nn + j] & 0xFFFF);
    }
    m3.data[i * nn + j] = (m3.data[i * nn + j] & 0xFFFF) + (sum & 0xFFFF);
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

const N: usize = 1000;
const TIMES: usize = 50;

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
    // let mut ave: TimeSpec = zero();
    // for _ in 0..TIMES {
    //     spawn_l2();
    //     ave = ave + test(run_until_idle, "1000 * 1000 threads");
    // }
    // println!("1000*1000 threads delta = {}", ave / TIMES as _);
}
