// TODO: Macro to ensure that size is big enough before doing it

use std::default::Default;

pub struct ElasticVector<T> {
    vec: Vec<T>,
}

impl<T> ElasticVector<T>
    where T: Default {
    pub fn new() -> Self {
        Self {
            vec: vec![]
        }
    }

    fn ensure_size(&mut self, required_size: usize) {
        if required_size >= self.vec.len() {
            self.vec.resize_with(required_size + 1, Default::default);
        }
    }

    // Sets the value at the specified index, ensuring the vector is large enough
    pub fn set(&mut self, index: usize, value: T) {
        self.ensure_size(index);
        self.vec[index] = value;
    }

    // Gets a reference to the value at the specified index, if it exists
    pub fn get(&self, index: usize) -> &T {
        self.vec.get(index).unwrap()
    }

    // Gets a reference to the value at the specified index, if it exists
    pub fn get_mut(&mut self, index: usize) -> &mut T {
        self.vec.get_mut(index).expect("Size ensured")
    }
}

// impl<T> Index<usize> for ElasticVector<T> where T: Default {
//     type Output = T;
//
//     fn index(&self, index: usize) -> &Self::Output {
//         self.ensure_size(index);
//         &self.vec.borrow()[index]
//     }
// }
//
// impl<T> IndexMut<usize> for ElasticVector<T>
//     where T: Default {
//     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//         self.ensure_size(index);
//         &mut self.vec.borrow_mut()[index]
//     }
// }
