use matrixmultiply::dgemm;
use rand::prelude::*;

fn randomize(array: &mut [f64]) -> *mut f64 {
    let mut rng = rand::thread_rng();
    for v in array.iter_mut() {
        *v = rng.gen();
    }
    &mut array[0] as *mut f64
}

fn main() {
    // C ← α A B + β C
    // https://docs.rs/matrixmultiply/0.2.3/matrixmultiply/fn.dgemm.html
    const m: usize = 4;
    const k: usize = 4;
    const n: usize = 4;
    const alpha: f64 = 1.0;
    let a = randomize(&mut [0.0; m * k]);
    const rsa: isize = 1;
    const csa: isize = 1;
    let b = randomize(&mut [0.0; k * n]);
    const rsb: isize = 1;
    const csb: isize = 1;
    const beta: f64 = 0.0;
    let c = randomize(&mut [0.0; m * n]);
    const rsc: isize = 1;
    const csc: isize = 1;
    unsafe {
        dgemm(m, k, n, alpha, a, rsa, csa, b, rsb, csb, beta, c, rsc, csc);
        let a = std::slice::from_raw_parts_mut(a, m * k);
        let b = std::slice::from_raw_parts_mut(b, k * n);
        let c = std::slice::from_raw_parts_mut(c, m * n);
        println!("A: {:?}", a);
        println!("B: {:?}", b);
        println!("C: {:?}", c);
    }
}
