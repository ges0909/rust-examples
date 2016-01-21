extern crate math;

#[test]
fn sqrt_int() {
    assert_eq!(math::sqrt(9).unwrap(), 81);
}

#[test]
fn sqrt_float() {
    assert_eq!(math::sqrt(0.9).unwrap(), 0.81);
}
