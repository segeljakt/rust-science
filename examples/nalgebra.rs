use approx::assert_relative_eq;
use nalgebra::{ArrayStorage, Matrix, SliceStorage, U1, U10, U20, U3, U4, U5, U6, U8};
fn main() {
    type M<R, C> = Matrix<f64, R, C, ArrayStorage<f64, R, C>>;
    type S<'a, R, C, RS, CS> = Matrix<f64, R, C, SliceStorage<'a, f64, R, C, RS, CS>>;
    //     let a: M<U10, U20> = M::<U10, U20>::new_random();
    //     let b: M<U20, U10> = M::<U20, U10>::new_random();
    //     let c: M<U10, U10> = a * b;
    //     let d: S<U6, U6, U1, U10> = c.fixed_slice::<U6, U6>(1, 1);
    //     let e: M<U3, U6> = d.remove_fixed_rows::<U3>(1);
    //     let f: M<U5, U8> = e.fixed_resize::<U5, U8>(0.0);

    let a: M<U10, U20> = M::<U10, U20>::new_random();
    let b: M<U20, U10> = M::<U20, U10>::new_random();
    let c: M<U10, U1> = M::<U10, U1>::new_random();
    let f = (a * b)
        .fixed_slice::<U6, U6>(1, 1)
        .remove_fixed_rows::<U3>(1)
        .fixed_resize::<U5, U8>(0.0);

    let l = M::<U4, U4>::new_random();
    let p = M::<U4, U1>::new_random();
    let x = l.lu().solve(&p).expect("Linear resolution failed.");
    assert_relative_eq!(l * x, p);
}
