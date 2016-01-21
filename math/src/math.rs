extern crate num;
use num::traits::Num;

pub fn sqrt<T: Num + Copy>(n: T) -> Result<T, String> {
    Result::Ok(n * n)
}

fn main() {
    println!("{}", sqrt(19).unwrap());
    println!("{}", sqrt(1.9).unwrap());
}

#[cfg(test)]
mod tests {
    use super::sqrt;

    #[test]
    fn sqrt_int() {
        assert!(sqrt(9).unwrap() == 81);
    }

    #[test]
    fn sqrt_float() {
        assert_eq!(sqrt(0.9).unwrap(), 0.81);
    }
}
