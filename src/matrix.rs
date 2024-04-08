use crate::error::{
    self, CustomErrors, EmptyVectorError, MismatchError, NonUniformError, NotImplementedError,
};
use crate::operations::add_vec;
use crate::util;

#[derive(Debug, Clone)]
pub struct Matrix<T: Copy> {
    pub rows: Vec<Vec<T>>,
    pub m: usize,
    pub n: usize,
}

impl<T: Copy> Matrix<T> {
    pub fn new(rows: Vec<Vec<T>>) -> Result<Matrix<T>, CustomErrors> {
        let m = rows.len();

        if m == 0 {
            return Err(CustomErrors::EmptyVector(EmptyVectorError));
        }

        let n = rows[0].len();

        if n == 0 {
            return Err(CustomErrors::EmptyVector(EmptyVectorError));
        }

        for i in 1..m {
            if rows[i].len() != n {
                return Err(CustomErrors::NonUniform(NonUniformError));
            }
        }
        Ok(Matrix { rows, m, n })
    }

    pub fn transpose(&self) -> Matrix<T> {
        let mut new_matrix: Vec<Vec<T>> = vec![];
        let loop1: usize = self.m;
        let loop2: usize = self.n;

        for j in 0..loop2 {
            let mut transposed_row = vec![];
            for i in 0..loop1 {
                transposed_row.push(self.rows[i][j])
            }
            new_matrix.push(transposed_row)
        }
        return Matrix {
            rows: new_matrix,
            m: self.n,
            n: self.m,
        };
    }
}

pub fn create_identity_matrix<T: Copy + From<u8>>(dim: usize) -> Result<Matrix<T>, CustomErrors> {
    if dim == 0 {
        return Err(CustomErrors::EmptyVector(EmptyVectorError));
    }
    let cast_zero: T = 0.into();
    let cast_one: T = 1.into();

    let mut rows: Vec<Vec<T>> = vec![];
    for i in 0..dim {
        let mut matrix_row: Vec<T> = vec![];

        for j in 0..dim {
            if j == i {
                matrix_row.push(cast_one)
            } else {
                matrix_row.push(cast_zero)
            }
        }
        rows.push(matrix_row);
    }
    Ok(Matrix {
        rows,
        m: dim,
        n: dim,
    })
}

pub fn add_matrices<T: std::marker::Copy + std::ops::Add<Output = T>>(
    matrix_1: Matrix<T>,
    matrix_2: Matrix<T>,
) -> Result<Matrix<T>, error::CustomErrors> {
    if !can_add(&matrix_1, &matrix_2) {
        return Err(error::CustomErrors::Mismatch(MismatchError));
    }

    let mut new_matrix_rows: Vec<Vec<T>> = vec![];
    for i in 0..matrix_1.m {
        let new_row = match add_vec(&matrix_1.rows[i], &matrix_2.rows[i]) {
            Ok(vec) => vec,
            Err(e) => return Err(e),
        };
        new_matrix_rows.push(new_row)
    }
    Ok(Matrix {
        rows: new_matrix_rows,
        m: matrix_1.m,
        n: matrix_1.n,
    })
}

pub fn multiply_matrices<
    T: Copy + From<u8> + std::ops::Mul + std::ops::AddAssign<<T as std::ops::Mul>::Output>,
>(
    matrix_1: Matrix<T>,
    matrix_2: Matrix<T>,
) -> Result<Matrix<T>, error::CustomErrors> {
    if matrix_1.m == 0 || matrix_1.n == 0 || matrix_2.m == 0 || matrix_2.n == 0 {
        return Err(CustomErrors::EmptyVector(EmptyVectorError));
    }

    if !can_multiply(&matrix_1, &matrix_2) {
        return Err(CustomErrors::Mismatch(MismatchError));
    }

    let mut new_matrix_rows: Vec<Vec<T>> = vec![];

    for i in 0..matrix_1.m {
        let mut new_row: Vec<T> = vec![];
        for j in 0..matrix_2.n {
            let mut element_sum: T = 0.into();
            for k in 0..matrix_1.n {
                element_sum += matrix_1.rows[i][k] * matrix_2.rows[k][j]
            }
            new_row.push(element_sum)
        }
        new_matrix_rows.push(new_row)
    }

    Ok(Matrix {
        rows: new_matrix_rows,
        m: matrix_1.m,
        n: matrix_2.n,
    })
}

pub fn is_square<T: Copy>(matrix: &Matrix<T>) -> bool {
    if matrix.m != matrix.n {
        return false;
    }
    true
}

pub fn can_add<T: Copy>(matrix_1: &Matrix<T>, matrix_2: &Matrix<T>) -> bool {
    if matrix_1.m != matrix_2.n {
        return false;
    }

    if matrix_1.n != matrix_2.n {
        return false;
    }

    true
}

pub fn can_multiply<T: Copy>(matrix_1: &Matrix<T>, matrix_2: &Matrix<T>) -> bool {
    if matrix_1.n != matrix_2.m {
        return false;
    }
    true
}

pub fn get_determinant<
    T: From<u8>
        + From<i32>
        + Copy
        + std::ops::Mul<Output = T>
        + std::ops::AddAssign
        + std::fmt::Debug
        + std::ops::MulAssign,
>(
    matrix: &Matrix<T>,
) -> Result<T, CustomErrors> {
    if !is_square(matrix) {
        return Err(CustomErrors::NotImplemented(NotImplementedError));
    }

    let size = matrix.rows.len();
    let rows = &matrix.rows;
    let perms = util::get_perms(size);
    let zero_cast: T = 0.into();
    let one_cast: T = 1.into();
    let mut determinant: T = zero_cast;
    for perm in perms {
        let sign = util::get_permutation_sign(perm.clone());
        let sign: T = sign.into();
        let mut term: T = one_cast;
        for i in 0..size {
            term *= rows[i][perm[i]]
        }
        determinant += sign * term;
    }


    Ok(determinant)
}
