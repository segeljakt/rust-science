#![allow(unused_imports)]
use quickcheck::QuickCheck;

fn associative<T: Eq + Copy>(f: fn(T, T) -> T) -> impl Fn(T, T, T) -> bool {
    move |a, b, c| f(a, f(b, c)) == f(f(a, b), c)
}

fn f(a: i32, b: i32) -> i32 {
    a + b
}

fn g(a: i32, b: i32) -> i32 {
    a - b
}
fn commutativity(a: i32, b: i32) -> bool {
    f(a, b) == f(b, a)
}
fn associativity(a: i32, b: i32, c: i32) -> bool {
    f(a, f(b, c)) == f(f(a, b), c)
}

fn semi_associativity(a: i32, b: i32, c: i32) -> bool {
    g(a, f(b, c)) == g(g(a, b), c)
}

fn main() {
//     let incr = (|x: i32| x + 1) as fn(i32) -> i32;
//     QuickCheck::new().quicktest(associative(f) as fn(i32,i32,i32)->bool);
    match QuickCheck::new().quicktest(associativity as fn(i32, i32, i32) -> bool) {
        Ok(n) => println!("Associativity passed for {} tests", n),
        Err(e) => println!("Not associative {:?}", e),
    }
    match QuickCheck::new().quicktest(commutativity as fn(i32, i32) -> bool) {
        Ok(n) => println!("Commutativity passed for {} tests", n),
        Err(e) => println!("Not commutative {:?}", e),
    }
}
