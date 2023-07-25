use std::ops::Add;

#[derive(Clone)]
pub struct Matrix<T> {
    data: Vec<Vec<T>>,
}

impl<T> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix {
            data: vec![vec![]],
        }
    }

    pub fn of(data: Vec<Vec<T>>) -> Matrix<T> {
        Self::integrity_check(&data);
        Matrix {
            data
        }
    }

    pub fn get_element(&self, x: usize, y: usize) -> &T {
        return &self.data[x][y];
    }

    pub fn size(&self) -> [usize; 2] {
        let data = &self.data;
        [data.len(), data[0].len()]
    }

    fn integrity_check(data: &Vec<Vec<T>>) {
        let valid_vector_size = data[0].len();
        for vector in data {
            if vector.len() != valid_vector_size {
                panic!("Matrix contains invalid vector size")
            }
        }
    }
}

impl Matrix<i8> {}

impl Add<Matrix<i8>> for Matrix<i8> {
    type Output = Self;

    fn add(self, addend: Matrix<i8>) -> Self {
        let result = self.clone();
        // todo
        Matrix::new()
    }
}

impl Matrix<i16> {}

impl Matrix<i32> {}

impl Matrix<i64> {}

impl Matrix<i128> {}

impl Matrix<isize> {}

impl Matrix<f32> {}

impl Matrix<f64> {}

#[cfg(test)]
mod matrix_tests {
    #[test]
    fn test() {}
}