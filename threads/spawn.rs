use std::thread;

static NTHREADS: u32 = 10000;

fn main() {
    let handle = thread::spawn(|| {
        print!("I am a spawned thread!");
    });
    let output = handle.join().unwrap();
    println!("{:?}", output); // ':?' to avoid =>   error: the trait `core::fmt::Display` is not implemented for the type `()`

    for n in 0..NTHREADS {
    	let _ = thread::spawn(move || { println!("thread {}", n); }).join();
    }
}
