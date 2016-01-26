// integration tests -- cargo test

// import crate to be tested
extern crate grep;

#[cfg(test)]
mod test {

    use super::grep::*;

    #[test]
    #[should_panic]
    fn test_outer_scope() {
        assert!(grep());
    }

    #[test]
    fn test_2() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_3() {
        assert_eq!("gerrit", "schrader");
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_4() {
        assert_eq!("gerrit", "schrader");
    }

}
