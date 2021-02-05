use std::io;

fn main() {
    let stdin = io::stdin();
    let minimap = code_minimap::write_to_string(stdin.lock(), 1.0, 1.0, None).unwrap();
    print!("{}", minimap);
}
