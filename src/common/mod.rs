use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T> {
    pub data: Vec<T>,
    pub height: usize,
    pub width: usize,
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        &self.data[y * self.width + x]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (x, y) = index;
        &mut self.data[y * self.width + x]
    }
}

impl<T> IntoIterator for Matrix<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<T> Matrix<T>
where
    T: Clone,
{
    pub fn new(width: usize, height: usize, default: T) -> Self {
        Self {
            data: vec![default; width * height],
            height,
            width,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        match self.in_bounds(x, y) {
            false => None,
            true => Some(&self[(x, y)]),
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        match self.in_bounds(x, y) {
            false => None,
            true => Some(&mut self[(x, y)]),
        }
    }

    pub fn set(&mut self, x: usize, y: usize, new_val: T) -> bool {
        match self.in_bounds(x, y) {
            false => false,
            true => {
                self[(x, y)] = new_val;
                true
            }
        }
    }

    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }
}
