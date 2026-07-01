use std::{
    ops::Index,
    slice::SliceIndex,
};

use bc_utils::other::roll_slice1;

pub struct Buffer(pub Vec<Vec<f64>>);

impl Buffer {
    pub fn new(src: Vec<Vec<f64>>) -> Self {
        Self(src)
    }
}

impl Buffer {
    pub fn update(
        &mut self,
        src: Vec<f64>,
    ) {
        roll_slice1(&mut self.0, -1);
        let l = self.0.len() - 1;
        self.0[l] = src;
    }
    pub fn update_extend(
        &mut self,
        src: &[Vec<f64>],
    ) {
        roll_slice1(&mut self.0, -(src.len() as i32));
        for _ in 0..src.len() {
            self.0.pop();
        }
        self.0.extend_from_slice(src);
    }
}

impl Buffer {
    pub fn iter(&self) -> impl Iterator<Item = &Vec<f64>> {
        self.0.iter()
    }
    pub fn first(&self) -> Option<&Vec<f64>> {
        self.0.first()
    }
    pub fn last(&self) -> Option<&Vec<f64>> {
        self.0.last()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn as_slice(&self) -> &[Vec<f64>] {
        self.0.as_slice()
    }
    pub fn transpose(mut self) -> Self {
        Self((0..self.0[0].len()).map(|_| self.0.iter_mut().map(|v| v.remove(0)).collect::<Vec<f64>>()).collect::<Vec<Vec<f64>>>())
    }
}

impl<T: SliceIndex<[Vec<f64>]>> Index<T> for Buffer {
    type Output = T::Output;
    fn index(
        &self,
        index: T,
    ) -> &Self::Output {
        &self.0[index]
    }
}

impl Extend<Vec<f64>> for Buffer {
    fn extend<T: IntoIterator<Item = Vec<f64>>>(
        &mut self,
        iter: T,
    ) {
        self.0.extend(iter);
    }
}