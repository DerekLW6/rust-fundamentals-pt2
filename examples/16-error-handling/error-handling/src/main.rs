use std::fs::File;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // The first argument is the path that was used to call the program.
    println!("My path is {}.", args[0]);

    // Check if a file name was provided
    if args.len() < 2 {
        panic!("Please provide a file name as argument");
    }

    let _file = File::open(&args[1]).unwrap();
}