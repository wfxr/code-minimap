use std::io;
use std::io::BufReader;

fn main() {
    let reader = Box::new(BufReader::new(io::stdin()));
    let s = code_minimap::write_to_string(reader, 1.0, 1.0, None).unwrap();
    print!("{}", s);
}
