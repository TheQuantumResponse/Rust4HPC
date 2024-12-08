use std::error::Error;

use matrix::Matrix;

pub fn matrix_multiplication<T>(c: &mut Matrix<T>, a: &Matrix<T>, b: &Matrix<T>) -> Result<(), Box<dyn Error>>
    where
        T: Copy + std::ops::AddAssign<<T as std::ops::Mul>::Output> + std::ops::Mul + Default,
{
    for i in 0..c.nrows {
        for j in 0..c.ncols {
            let mut sum = T::default();
            for k in 0..a.ncols {
                let a_ik = a.get(i, k).ok_or("Invalid index in matrix A")?;
                let b_kj = b.get(k, j).ok_or("Invalid index in matrix B")?;

                sum += (*a_ik)*(*b_kj);
            }
            c.set(i, j, sum)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_multiplication() {
        let mut c = Matrix::new(2,2,0.0f64);
        let a = Matrix::new(2,2,1.0f64);
        let b = Matrix::new(2,2,1.0f64);

        let mut actual = Matrix::new(2,2,0.0f64);
        actual.set(0,0,2.0).unwrap();
        actual.set(0,1,2.0).unwrap();
        actual.set(1,0,2.0).unwrap();
        actual.set(1,1,2.0).unwrap();

        _ = matrix_multiplication(&mut c, &a, &b);
        assert_eq!(actual, c);
    }
}
