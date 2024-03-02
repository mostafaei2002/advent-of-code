use std::fs;

fn main() {
    let inp = fs::read_to_string("../question").unwrap();

    println!("{inp}")
}
