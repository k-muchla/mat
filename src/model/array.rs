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
        if data.len() != Self::get_dimensions_size(&dimensions) {
            panic!("Data size does not match dimensions size.")
        }
        Array {
            dimensions,
            data,
        }
    }

    pub fn fill(dimensions: Vec<usize>, value: T) -> Array<T> {
        Array {
            data: vec![value; Self::get_dimensions_size(&dimensions)],
            dimensions,
        }
    }

    fn get_dimensions_size(dimensions: &Vec<usize>) -> usize {
        if dimensions.is_empty() {
            return 0;
        }
        let mut dimensions_size: usize = 1;
        for dimension in dimensions {
            dimensions_size *= dimension;
        }
        dimensions_size
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
        let dimensions: Vec<usize> = vec![2, 3, 2];
        let data: Vec<isize> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

        let array: Array<isize> = Array::of(dimensions, data);

        assert_eq!(array.dimensions, vec![2, 3, 2]);
        assert_eq!(array.data, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    }

    #[test]
    #[should_panic]
    fn should_not_create_array_of_larger_dimensions_than_data() {
        let dimensions: Vec<usize> = vec![1, 2, 3];
        let data: Vec<isize> = vec![1, 1];

        Array::of(dimensions, data);
    }

    #[test]
    #[should_panic]
    fn should_not_create_array_of_larger_data_than_dimensions() {
        let dimensions: Vec<usize> = vec![1, 1];
        let data: Vec<isize> = vec![1, 2];

        Array::of(dimensions, data);
    }

    #[test]
    fn should_fill_array() {
        let dimensions: Vec<usize> = vec![1, 2, 3];
        let value: isize = 11;

        let array: Array<isize> = Array::fill(dimensions, value);

        assert_eq!(array.dimensions, vec![1, 2, 3]);
        assert_eq!(array.data, vec![11, 11, 11, 11, 11, 11]);
    }
}