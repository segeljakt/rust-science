#![allow(unused_imports)]
use alga::general::AbstractMonoid;
use quickcheck::quickcheck;
quickcheck! {
    fn prop_mul_is_associative(args: (i32, i32, i32)) -> bool {
        AbstractMonoid::<f32>::prop_mul_is_associative(args)
    }
}
fn main() {}
