// The tests in the tests directory are an entirely separate crate
// and so we need to import the library. The integration-style tests
// use the library like any other consumer would.
extern crate math;
// use math::square;

// annotate function with attribute 'test'
#[test]
fn integration_test_square() {
    assert!(math::square(9).unwrap() == 81);
    assert_eq!(math::square(0.9).unwrap(), 0.81);
}

#[test]
#[should_panic]
fn integration_test_square_should_panic() {
    assert_eq!(math::square(0.8).unwrap(), 0.81);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn integration_test_square_should_panic_expected() {
    assert_eq!(math::square(0.8).unwrap(), 0.81);
}

#[test]
#[ignore]
fn integration_test_ignore() {
    assert_eq!(math::square(0.9).unwrap(), 0.81);
}
