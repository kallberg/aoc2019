use std::io::stdin;

mod d2;

fn main() {
    let stdin = stdin();
    let mut handle = stdin.lock();

    d2::run(&mut handle);
}
