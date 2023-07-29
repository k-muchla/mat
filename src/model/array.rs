use std::ops::{Add, Index};

#[derive(Clone)]
pub struct Array<T> {
    dimensions: Vec<usize>,
    data: Vec<T>,
}

impl<T: Clone> Array<T> {
    pub fn new() -> Array<T> {
        Array {
            dimensions: vec![0],
            data: vec![],
        }
    }

    pub fn of(dimensions: Vec<usize>, data: Vec<T>) -> Array<T> {
        if data.len() != Self::get_data_len(&dimensions) {
            panic!("Incorrect dimensions for provided data.")
        }
        Array { dimensions, data }
    }

    pub fn fill(dimensions: Vec<usize>, value: T) -> Array<T> {
        Array {
            data: vec![value; Self::get_data_len(&dimensions)],
            dimensions,
        }
    }

    fn get_data_len(dimensions: &Vec<usize>) -> usize {
        if dimensions.is_empty() {
            return 0;
        }
        let mut data_len: usize = 1;
        for dimension in dimensions {
            data_len *= dimension;
        }
        data_len
    }
}

impl<T> Index<Vec<usize>> for Array<T> {
    type Output = T;

    fn index(&self, array_index: Vec<usize>) -> &T {
        if array_index.is_empty() {
            panic!("Array index cannot be empty.")
        }
        let dimensions = &self.dimensions;
        if array_index.len() != dimensions.len() {
            panic!(
                "Array index length {} does not match dimensions length {}.",
                array_index.len(),
                dimensions.len()
            )
        }
        let mut data_index: usize = 0;
        let mut offset: usize = 1;
        for dimensions_index in 0..dimensions.len() {
            let dimension = dimensions[dimensions_index];
            let dimension_index = array_index[dimensions_index];
            if dimension_index >= dimension {
                panic!(
                    "Provided dimension index {} is out of bounds for dimension of length {}.",
                    dimension_index, dimension
                )
            }
            data_index += dimension_index * offset;
            offset *= dimension;
        }
        return &self.data[data_index];
    }
}

impl<T: Add<Output = T> + Copy> Add<Array<T>> for Array<T> {
    type Output = Self;

    fn add(self, addend: Array<T>) -> Self::Output {
        if self.dimensions != addend.dimensions {
            panic!("Augend dimensions and addend dimensions mismatch.")
        }
        let data: Vec<T> = (0..self.data.len())
            .map(|data_index| self.data[data_index] + addend.data[data_index])
            .collect();
        Array {
            data,
            dimensions: self.dimensions,
        }
    }
}

#[cfg(test)]
mod tests {
    use model::array::Array;

    #[test]
    fn should_create_new_array() {
        let array: Array<isize> = Array::new();

        assert_eq!(array.dimensions, vec![0]);
        assert_eq!(array.data, vec![]);
    }

    #[test]
    fn should_create_array_of() {
        let dimensions = vec![2, 3, 2];
        let data: Vec<isize> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

        let array: Array<isize> = Array::of(dimensions, data);

        assert_eq!(array.dimensions, vec![2, 3, 2]);
        assert_eq!(array.data, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    }

    #[test]
    #[should_panic]
    fn should_not_create_array_of_larger_dimensions_than_data() {
        let dimensions = vec![1, 2, 3];
        let data: Vec<isize> = vec![1, 1];

        Array::of(dimensions, data);
    }

    #[test]
    #[should_panic]
    fn should_not_create_array_of_larger_data_than_dimensions() {
        let dimensions = vec![1, 1];
        let data: Vec<isize> = vec![1, 2];

        Array::of(dimensions, data);
    }

    #[test]
    fn should_fill_array() {
        let dimensions = vec![1, 2, 3];
        let value: isize = 11;

        let array: Array<isize> = Array::fill(dimensions, value);

        assert_eq!(array.dimensions, vec![1, 2, 3]);
        assert_eq!(array.data, vec![11, 11, 11, 11, 11, 11]);
    }

    #[test]
    fn should_get_array_element_by_index() {
        let array_index = vec![0, 2, 1];
        let dimensions = vec![2, 3, 2];
        let data: Vec<isize> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let array: Array<isize> = Array::of(dimensions, data);

        let array_element = array[array_index];

        assert_eq!(array_element, 11);
    }

    #[test]
    #[should_panic]
    fn should_not_get_array_element_by_index_when_empty() {
        let array_index = vec![];
        let array: Array<isize> = Array::new();

        array[array_index];
    }

    #[test]
    #[should_panic]
    fn should_not_get_array_element_by_index_with_different_dimensions_length() {
        let array_index = vec![0, 0, 0];
        let dimensions = vec![2, 2];
        let data: Vec<isize> = vec![1, 2, 3, 4];
        let array: Array<isize> = Array::of(dimensions, data);

        array[array_index];
    }

    #[test]
    #[should_panic]
    fn should_not_get_array_element_by_index_with_invalid_dimension_index() {
        let array_index = vec![0, 999];
        let dimensions = vec![2, 2];
        let data: Vec<isize> = vec![1, 2, 3, 4];
        let array: Array<isize> = Array::of(dimensions, data);

        array[array_index];
    }

    #[test]
    fn should_add_arrays() {
        let first_array: Array<isize> = Array::of(vec![2, 2], vec![4, 3, 2, 1]);
        let second_array: Array<isize> = Array::of(vec![2, 2], vec![1, 2, 3, 4]);

        let result = first_array + second_array;

        assert_eq!(result.dimensions, vec![2, 2]);
        assert_eq!(result.data, vec![5, 5, 5, 5]);
    }

    #[test]
    #[should_panic]
    fn should_not_add_arrays_with_different_dimensions() {
        let first_array: Array<isize> = Array::of(vec![2, 2], vec![4, 3, 2, 1]);
        let second_array: Array<isize> = Array::of(vec![2, 2, 1], vec![1, 2, 3, 4]);

        let _ = first_array + second_array;
    }
}
