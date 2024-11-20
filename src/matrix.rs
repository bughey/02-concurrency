use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Mul},
};

use anyhow::Result;

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
            let mut sum = T::default();
            for k in 0..a.col {
                sum += a.data[i * a.col + k] * b.data[k * b.col + j];
            }
            result.data[i * result.col + j] = sum;
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
        let result = multiply(&m1, &m2).unwrap();
        println!("{}", result);
    }
}
