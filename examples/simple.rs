use std::io;

fn main() {
    let stdin = io::stdin();
    code_minimap::print(stdin.lock(), 1.0, 1.0, None).unwrap();
}
