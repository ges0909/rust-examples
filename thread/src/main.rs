extern crate num_cpus;
extern crate threadpool;

use std::thread;
use threadpool::ThreadPool; 

static NTHREADS: u32 = 10000;

fn main() {
	// 1. spawn
    let handle = thread::spawn(|| {
        print!("I am a spawned thread!");
    });
    let output = handle.join().unwrap();
    println!("{:?}", output); // ':?' to avoid =>   error: the trait `core::fmt::Display` is not implemented for the type `()`

    for n in 0..NTHREADS {
    	let _ = thread::spawn(move || { print!("{}; ", n); }).join();
    }
	println!("");

	// 2. number of cpus
	let ncpus = num_cpus::get();
	println!("#cpus = {}", ncpus);

	// 3.thread pool
	let pool = ThreadPool::new(ncpus);
	for n in 0..ncpus {
		pool.execute(move || {
			println!("this is thread # {}", n);
		});
	}
	thread::sleep_ms(50);

	// 4. panicking thread
	let result = thread::spawn(move || {
		panic!("panic: file={}, line={}", file!(), line!());
	}).join();

	if result.is_err() {
		println!("The child has panicked!");
	}
}
