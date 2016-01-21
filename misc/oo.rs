use std::fmt::Debug;

// - nested structs are not possible in Rust
// - method overloading is not possible in Rust
// - bit methods can b defined on tuples and enums

#[derive(Debug)]
struct Birth {
    day: u8,
    month: u8,
    year: u16,
}

impl Birth {
    fn new(_day: u8, _month: u8, _year: u16) -> Birth {
        assert!(_day >= 1 && _day <= 31, "day of birth out of range");
        assert!(_month >= 1 && _month <= 12, "month of birth out of range");
        // assert!(_year >= 0, "negative year of birth"); // warning: comparison is useless due to type limits
        Birth {
            day: _day,
            month: _month,
            year: _year,
        }
    }
}

#[derive(Debug)]
struct Person {
    first: &'static str,
    last: &'static str,
    birth: Birth,
}

impl Person {
    fn new(_first: &'static str, _last: &'static str, _birth: Birth) -> Person {
        Person {
            first: _first,
            last: _last,
            birth: _birth,
        }
    }

    // instance method has 'self' param. referring to the object itself
    fn to_string(&mut self) -> String {
        self.first = "Heike";
        let mut buf = String::new();
        buf.push_str("first: ");
        buf.push_str(self.first);
        buf.push_str(", last: ");
        buf.push_str(self.last);
        buf
    }
}

fn main() {
    // new() -> static method
    let mut ich = Person::new("Gerrit", "Schrader", Birth::new(9, 9, 1961));
    println!("{:?}", ich); // requires impl. of trait 'Debug'
    // to_string() -> instance method
    println!("to_string() = {}", ich.to_string()); // requires impl. of trait 'Debug'
}
