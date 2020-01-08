use ndarray::Array;

fn main() {
    let mut a = Array::zeros((2, 3));
    let mut b = Array::zeros((3, 2));
    a[[1, 2]] = 7;
    b[[0, 0]] = 7;
    let c = a.dot(&b);
}
