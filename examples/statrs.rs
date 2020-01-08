use rand::distributions::Distribution;
use statrs::distribution::Normal;

fn main() {
    let mut r = rand::thread_rng();
    let n = Normal::new(0.0, 1.0).unwrap();
    for _ in 0..10 {
        print!("{}", n.sample(&mut r));
    }
}
