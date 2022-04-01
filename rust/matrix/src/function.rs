use super::*;

type MyResult = Result<(), &'static str>;

// 1000 * 1000 func calls

#[inline(never)]
pub fn l3(NN: usize, i: usize, j: usize, m1: &Matrix, m2: &Matrix, m3: &mut Matrix) {
    let mut sum: usize = 0;
    for k in 0..NN {
        sum = sum + (m1.data[i * NN + k] & 0xFFFF) * (m2.data[k * NN + j] & 0xFFFF);
    }
    m3.data[i * NN + j] = (m3.data[i * NN + j] & 0xFFFF) + (sum & 0xFFFF);
}

#[inline(never)]
pub fn l2(NN: usize, i: usize, m1: &Matrix, m2: &Matrix, m3: &mut Matrix) {
    for j in 0..NN {
        l3(NN, i, j, m1, m2, m3);
    }
}

#[inline(never)]
pub fn l1(NN: usize, m1: &Matrix, m2: &Matrix, m3: &mut Matrix) {
    for i in 0..NN {
        l2(NN, i, m1, m2, m3);
    }
}

#[inline(never)]
pub fn fdot(m1: &Matrix, m2: &Matrix, m3: &mut Matrix) -> MyResult {
    size_check(m1, m2, m3)?;
    let NN = m1.n;
    l1(NN, m1, m2, m3);
    Ok(())
}

#[inline(never)]
pub fn size_check(m1: &Matrix, m2: &Matrix, m3: &Matrix) -> MyResult {
    if !equal(m1.n, m2.n) || !equal(m1.n, m3.n) {
        return Err("matrix not aligned");
    }
    Ok(())
}

// 1000 * 1000 * 7: func calls

#[inline(never)]
pub fn fdot_slow(m1: &Matrix, m2: &Matrix, m3: &mut Matrix) -> MyResult {
    size_check(m1, m2, m3)?;
    let NN = m1.n;
    l1_slow(NN, m1, m2, m3);
    Ok(())
}

#[inline(never)]
pub fn l3_slow(NN: usize, i: usize, j: usize, m1: &Matrix, m2: &Matrix, m3: &mut Matrix) {
    let mut sum: usize = 0;
    for k in 0..NN {
        sum = add(
            sum,
            times(overflow(m1.data[i * NN + k]), overflow(m2.data[k * NN + j])),
        );
    }
    m3.data[i * NN + j] = add(overflow(m3.data[i * NN + j]), overflow(sum));
}

#[inline(never)]
pub fn l2_slow(NN: usize, i: usize, m1: &Matrix, m2: &Matrix, m3: &mut Matrix) {
    for j in 0..NN {
        l3_slow(NN, i, j, m1, m2, m3);
    }
}

#[inline(never)]
pub fn l1_slow(NN: usize, m1: &Matrix, m2: &Matrix, m3: &mut Matrix) {
    for i in 0..NN {
        l2_slow(NN, i, m1, m2, m3);
    }
}

#[inline(never)]
pub fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

#[inline(never)]
pub fn equal<T: std::cmp::PartialEq>(a: T, b: T) -> bool {
    a == b
}

#[inline(never)]
pub fn times<T: std::ops::Mul<Output = T>>(a: T, b: T) -> T {
    a * b
}

#[inline(never)]
pub fn overflow(n: usize) -> usize {
    n & 0xFFFF
}