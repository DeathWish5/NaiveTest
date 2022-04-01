

pub mod function;
pub mod coroutine;

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
    let NN = m.n;
    for i in 0..NN {
        for j in 0..NN {
            m.data[i * NN + j] = random();
        }
    }
    Ok(())
}

pub fn dot(m1: &Matrix, m2: &Matrix, m3: &mut Matrix) -> Result<(), &'static str> {
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
