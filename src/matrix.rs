use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Mul},
};

use anyhow::Result;

use crate::vector::{dot_product, Vector};

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
    T: Debug + Default + Copy + Add<Output = T> + AddAssign + Mul<Output = T>,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        multiply(&self, &rhs).expect("Matrix multiply error")
    }
}

#[allow(dead_code)]
pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Debug + Default + Copy + Add<Output = T> + AddAssign + Mul<Output = T>,
{
    if a.col != b.row {
        return Err(anyhow::anyhow!("Matrix dimensions mismatch"));
    }

    let mut result = Matrix::new(a.row, b.col);

    for i in 0..a.row {
        for j in 0..b.col {
            /* let mut sum = T::default();
            for k in 0..a.col {
                sum += a.data[i * a.col + k] * b.data[k * b.col + j];
            }
            result.data[i * result.col + j] = sum; */

            // use dot_product
            let row = a.row(i);
            let col = b.col(j);
            result.data[i * result.col + j] = dot_product(row, col)?;
        }
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
