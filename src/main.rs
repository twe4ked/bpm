use bpm;
use std::process;

fn main() {
    bpm::run().unwrap_or_else(|err| {
        println!("An error occured: {}", err);
        process::exit(1);
    });
}
