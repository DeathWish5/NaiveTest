use super::*;

type MyResult = Result<(), &'static str>;

#[inline(never)]
pub async fn cdot(m1: &Matrix, m2: &Matrix, m3: &mut Matrix) {
    size_check(m1, m2, m3).await.unwrap();
    let NN = m1.n;
    l1(NN, m1, m2, m3).await;
}

#[inline(never)]
pub async fn l1(NN: usize, m1: &Matrix, m2: &Matrix, m3: &mut Matrix) {
    for i in 0..NN {
        l2(NN, i, m1, m2, m3).await;
    }
}

#[inline(never)]
pub async fn l2(NN: usize, i: usize, m1: &Matrix, m2: &Matrix, m3: &mut Matrix) {
    for j in 0..NN {
        l3(NN, i, j, m1, m2, m3).await;
    }
}

#[inline(never)]
pub async fn l3(NN: usize, i: usize, j: usize, m1: &Matrix, m2: &Matrix, m3: &mut Matrix) {
    let mut sum: usize = 0;
    for k in 0..NN {
        sum = sum + (m1.data[i * NN + k] & 0xFFFF) * (m2.data[k * NN + j] & 0xFFFF);
    }
    m3.data[i * NN + j] = (m3.data[i * NN + j] & 0xFFFF) + (sum & 0xFFFF);
}

#[inline(never)]
pub async fn l3_stack(nn: usize, i: usize, j: usize, m1: &Matrix, m2: &Matrix, m3: &mut Matrix) {
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

#[inline(never)]
pub async fn l1_stack(NN: usize, m1: &Matrix, m2: &Matrix, m3: &mut Matrix) {
    for i in 0..NN {
        l2_stack(NN, i, m1, m2, m3).await;
    }
}

#[inline(never)]
pub async fn l2_stack(NN: usize, i: usize, m1: &Matrix, m2: &Matrix, m3: &mut Matrix) {
    for j in 0..NN {
        l3_stack(NN, i, j, m1, m2, m3).await;
    }
}

#[inline(never)]
pub async fn size_check(m1: &Matrix, m2: &Matrix, m3: &Matrix) -> MyResult {
    if !equal(m1.n, m2.n).await || !equal(m1.n, m3.n).await {
        return Err("matrix not aligned");
    }
    Ok(())
}

#[inline(never)]
pub async fn cdot_slow(m1: &Matrix, m2: &Matrix, m3: &mut Matrix) {
    size_check(m1, m2, m3).await.unwrap();
    let NN = m1.n;
    l1(NN, m1, m2, m3).await;
}

#[inline(never)]
pub async fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

#[inline(never)]
pub async fn equal<T: std::cmp::PartialEq>(a: T, b: T) -> bool {
    a == b
}

#[inline(never)]
pub async fn times<T: std::ops::Mul<Output = T>>(a: T, b: T) -> T {
    a * b
}

#[inline(never)]
pub async fn overflow(n: usize) -> usize {
    n & 0xFFFF
}

#[inline(never)]
pub async fn l3_slow(NN: usize, i: usize, j: usize, m1: &Matrix, m2: &Matrix, m3: &mut Matrix) {
    let mut sum: usize = 0;
    for k in 0..NN {
        sum = add(
            sum,
            times(
                overflow(m1.data[i * NN + k]).await,
                overflow(m2.data[k * NN + j]).await,
            )
            .await,
        )
        .await;
    }
    m3.data[i * NN + j] = add(overflow(m3.data[i * NN + j]).await, overflow(sum).await).await;
}

#[inline(never)]
pub async fn l2_slow(NN: usize, i: usize, m1: &Matrix, m2: &Matrix, m3: &mut Matrix) {
    for j in 0..NN {
        l3_slow(NN, i, j, m1, m2, m3).await;
    }
}

#[inline(never)]
pub async fn l1_slow(NN: usize, m1: &Matrix, m2: &Matrix, m3: &mut Matrix) {
    for i in 0..NN {
        l2_slow(NN, i, m1, m2, m3).await;
    }
}
