#![allow(unused_imports)]
use rust_decimal::{prelude::*, Decimal};

fn main() {
    let d = Decimal::new(1337, 2);
    assert_eq!(d.trunc(), Decimal::new(1300, 2));
    assert_eq!(d.fract(), Decimal::new(0037, 2));
    let a = Decimal::from_scientific("9.7e-7").unwrap();
    let b = Decimal::from_scientific("3.33e-5").unwrap();
    println!("{} + {} = {}", a, b, a + b);
}
