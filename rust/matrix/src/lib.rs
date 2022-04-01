pub mod coroutine;
pub mod function;

pub const N: usize = 1000;

pub struct Matrix {
    pub data: Box<[usize]>,
    pub n: usize,
}

impl Matrix {
    pub fn new(n: usize) -> Self {
        Matrix {
            data: {
                let data = vec![0usize; n * n];
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

pub fn dot(m1: &Matrix, m2: &Matrix, m3: &mut Matrix) -> Result<(), &'static str> {
    if m1.n != m2.n || m1.n != m3.n {
        return Err("matrix not aligned");
    }
    let nn = m1.n;
    for i in 0..nn {
        for j in 0..nn {
            let mut sum: usize = 0;
            for k in 0..nn {
                sum += (m1.data[i * nn + k] & 0xFFFF) * (m2.data[k * nn + j] & 0xFFFF);
            }
            m3.data[i * nn + j] = (m3.data[i * nn + j] & 0xFFFF) + (sum & 0xFFFF);
        }
    }
    Ok(())
}

// use std::mem::MaybeUninit;

pub fn dot_stack(m1: &Matrix, m2: &Matrix, m3: &mut Matrix) -> Result<(), &'static str> {
    if m1.n != m2.n || m1.n != m3.n {
        return Err("matrix not aligned");
    }
    let nn = m1.n;
    for i in 0..nn {
        for j in 0..nn {
            let mut row: [usize; N] = [0; N]; // unsafe { MaybeUninit::<[usize; N]>::uninit().assume_init() };
            let mut col: [usize; N] = [0; N]; // unsafe { MaybeUninit::<[usize; N]>::uninit().assume_init() };
            for k in 0..nn {
                row[k] = m1.data[i * nn + k] & 0xFFFF;
                col[k] = m2.data[k * nn + j] & 0xFFFF;
            }
            let sum = row
                .iter()
                .zip(col.iter())
                .map(|data| data.0 * data.1)
                .fold(0, |x, accel| x + accel);
            m3.data[i * nn + j] = (m3.data[i * nn + j] & 0xFFFF) + (sum & 0xFFFF);
        }
    }
    Ok(())
}
