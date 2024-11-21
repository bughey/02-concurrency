use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Mul},
    sync::mpsc,
};

use anyhow::Result;
use oneshot::Sender;

use crate::vector::{dot_product, Vector};

const NUM_PRODUCERS: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Matrix<T>
where
    T: Debug + Default + Copy,
{
    data: Vec<T>,
    row: usize,
    col: usize,
}

#[allow(dead_code)]
impl<T> Matrix<T>
where
    T: Debug + Default + Copy,
{
    pub fn new(row: usize, col: usize) -> Self {
        Self {
            data: vec![T::default(); row * col],
            row,
            col,
        }
    }

    pub fn from_data(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        Self {
            data: data.into(),
            row,
            col,
        }
    }

    // 获取第n行数据，返回Vector
    pub fn row(&self, n: usize) -> Vector<T> {
        Vector::new(&self.data[n * self.col..n * self.col + self.col])
    }

    // 获取第n列数据，返回Vector， 使用iterator实现
    pub fn col(&self, n: usize) -> Vector<T> {
        Vector::new(
            /* (0..self.row)
            .map(|i| self.data[i * self.col + n])
            .collect::<Vec<T>>(), */
            self.data[n..]
                .iter()
                .step_by(self.col)
                .copied()
                .collect::<Vec<T>>(),
        )
    }
}

impl<T> Display for Matrix<T>
where
    T: Debug + Default + Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.row {
            for j in 0..self.col {
                write!(f, "{:?} ", self.data[i * self.col + j])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> Mul for Matrix<T>
where
    T: Debug + Default + Copy + Add<Output = T> + AddAssign + Mul<Output = T> + Send + 'static,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        multiply(&self, &rhs).expect("Matrix multiply error")
    }
}

struct MsgInput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}

struct MsgOutput<T> {
    idx: usize,
    value: T,
}

struct Msg<T> {
    sender: Sender<MsgOutput<T>>,
    input: MsgInput<T>,
}

#[allow(dead_code)]
pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Debug + Default + Copy + Add<Output = T> + AddAssign + Mul<Output = T> + Send + 'static,
{
    if a.col != b.row {
        return Err(anyhow::anyhow!("Matrix dimensions mismatch"));
    }

    // 根据NUM_PRODUCERS开启任务线程，返回mpsc::Sender，保存到Vec<Sencer>中
    let mut senders: Vec<mpsc::Sender<Msg<T>>> = Vec::new();
    for _ in 0..NUM_PRODUCERS {
        let (tx, rx) = mpsc::channel();
        senders.push(tx);
        std::thread::spawn(move || {
            for msg in rx {
                let MsgInput { idx, row, col } = msg.input;
                let value = dot_product(row, col).unwrap();
                msg.sender.send(MsgOutput { idx, value }).unwrap();
            }
        });
    }

    let mut result = Matrix::new(a.row, b.col);

    let mut receivers: Vec<oneshot::Receiver<MsgOutput<T>>> = Vec::new();
    for i in 0..a.row {
        for j in 0..b.col {
            let row = a.row(i);
            let col = b.col(j);

            // 从集合中选择Sender发送数据
            // let sender = senders[i % NUM_PRODUCERS].clone();
            let (tx, rx) = oneshot::channel();
            let idx = i * result.col + j;
            senders[i % NUM_PRODUCERS]
                .send(Msg {
                    sender: tx,
                    input: MsgInput { idx, row, col },
                })
                .expect("Send msg input error");
            receivers.push(rx);

            // result.data[i * result.col + j] = dot_product(row, col)?;
        }
    }

    // 从Receiver中接收数据，填充到result中
    for rx in receivers {
        let MsgOutput { idx, value } = rx.recv()?;
        result.data[idx] = value;
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        let m1 = Matrix::from_data(vec![1; 4], 2, 2);
        let m2 = Matrix::from_data(vec![2; 4], 2, 2);
        let result = m1 * m2;
        println!("{}", result);
    }
}
