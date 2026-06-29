use bc_utils::other::roll_slice1;

pub struct Buffer(pub Vec<Vec<f64>>);

impl Buffer {
    pub fn new(src: Vec<Vec<f64>>,) -> Self {
        Self(src)
    }
}

impl Buffer {
    pub fn update(&mut self, src: Vec<f64>) {
        roll_slice1(&mut self.0, -1);
        let l = self.0.len() - 1;
        self.0[l] = src;
    }
    pub fn update_extend(&mut self, src: &[Vec<f64>]) {
        roll_slice1(&mut self.0, -(src.len() as i32));
        for _ in 0..src.len() {
            self.0.pop();
        }
        self.0.extend_from_slice(src);
    }
}

impl Extend<Vec<f64>> for Buffer {
    fn extend<T: IntoIterator<Item = Vec<f64>>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}