mod mlp;

use ndarray::{Array, Array1, Array2, array};


fn main() {
    {
        let diag = array![1, 2, 3];
        let matrix = Array2::from_diag(&diag);
        let vec = Array1::from_iter([3, 1, 0].into_iter());
        let vec = &matrix * vec;
    }

    let a = array![1, 2, 3];
    let b = array![[1, 0, 0], [0, 2, 0], [0, 0, 1]];

    let c = a.dot(&b);
    // let d = &b * &a;

    println!("{:?}", c);
    // println!("{:?}", d);
}
