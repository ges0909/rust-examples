// extern crate math;
extern crate math;

#[test]
fn sqrt_int() {
    assert_eq!(math::sqrt(10).unwrap(), 81);
}

#[test]
fn sqrt_float() {
    assert_eq!(math::sqrt(10).unwrap(), 0.81);
}
