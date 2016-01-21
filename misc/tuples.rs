fn main() {

    // tuples:
    // store values of different types
    let tup1 = ("Gerrit", "Schrader", 9, 9, 1961);
    println!("{} {} born at {:02}.{:02}.{}",
             tup1.0,
             tup1.1,
             tup1.2,
             tup1.3,
             tup1.4);

    // tuple structs:
    // named tuples are called tuple structs
    struct Person(&'static str, &'static str, usize, usize, usize);
    let tup2 = Person("Gerrit", "Schrader", 9, 9, 1961);
    // destructering
    let Person(first, last, day_of_birth, month_of_birth, year_of_birth) = tup2;
    println!("Birtday = {:02}.{:02}.{}",
             day_of_birth,
             month_of_birth,
             year_of_birth);

    // tuple structs:
    // names start with a capital letter and follows CamelCase
    // field names follow snake_case
    struct Person2 {
        first: &'static str,
        last: &'static str,
        day_of_birth: u8,
        month_of_birth: u8,
        year_of_birth: u16,
    }
    // struct requires 'name: value' syntax
    let mut ich = Person2 {
        first: "Gerrit",
        last: "Schrader",
        day_of_birth: 9,
        month_of_birth: 9,
        year_of_birth: 1991,
    };
    ich.year_of_birth = 1961;

    // Destructuring
    let Person2{last: my_last_name, ..} = ich;
    println!("my last name is {}.", my_last_name);

}
