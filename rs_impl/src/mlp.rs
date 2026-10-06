use ndarray::{Array2, Array1};

pub struct Mlp<T> {
    layer_weights: Vec<Array2<T>>,
    layer_biases: Vec<Array1<T>>
}

impl<T> Mlp<T> {
    pub fn feed_through(&self, x: &Array1<T>) -> Array1<T> {
        let iter = self
            .layer_weights
            .iter()
            .zip(self.layer_biases.iter());

        for (w, b) in iter {
            
        }

        todo!()
    }
}