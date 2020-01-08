#![allow(unused_imports)]
use quickcheck_macros::quickcheck;

fn reverse<T: Clone>(xs: &[T]) -> Vec<T> {
    let mut rev = vec![];
    for x in xs {
        rev.insert(0, x.clone())
    }
    rev
}

#[quickcheck]
fn double_reversal_is_identity(xs: Vec<isize>) -> bool {
    xs == reverse(&reverse(&xs))
}

fn f(a: i32, b: i32) -> i32 {
    a + b
}

fn g(a: i32, b: i32) -> i32 {
    a - b
}

#[quickcheck]
fn commutativivity(a: i32, b: i32) -> bool {
    f(a, b) == f(b, a)
}

#[quickcheck]
fn associativity(a: i32, b: i32, c: i32) -> bool {
    f(a, f(b, c)) == f(f(a, b), c)
}

#[quickcheck]
fn semi_associativity(a: i32, b: i32, c: i32) -> bool {
    g(a, f(b, c)) == g(g(a, b), c)
}
