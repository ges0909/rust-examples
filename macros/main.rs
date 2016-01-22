macro_rules! welcome {
	() => (
		println!("Welcome Hugo!")
	);
	// single arg
	( $single:expr ) => (
		println!("Welcome {}!", $single)
	);
	// multiple args
	// repetitions are enclosed in $(...)*
	( $format:expr, $( $multiple:expr ),* ) => (
		println!($format, $( $multiple ),* );
	);
}

macro_rules! create_fn {
    ($fname:ident) => (
    	fn $fname() { println!("Benvenuto! Sono il funzione '{}'.", stringify!($fname)) }
    )
}

create_fn!(benvenuto);

fn main() {
    welcome!();
    welcome!("Hugo");
    welcome!("Welcome {}, {} and {}!", "Hugo", "Go", "Brightone");
    benvenuto();
}
