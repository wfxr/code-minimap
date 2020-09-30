use std::io;
use std::io::BufReader;

fn main() {
    let reader = Box::new(BufReader::new(io::stdin()));
    code_minimap::print(reader, 1.0, 1.0, None).unwrap();
}
