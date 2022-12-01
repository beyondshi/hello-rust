use ferris_says::say;
use std::io::{ stdout, BufWriter };

fn main() {
     println!("Hello, world!");
    let stdout = stdout();
    let out = "Hello fellow Rustaceans!";
    let width = 24;

    let writer = BufWriter::new(stdout.lock());
    say(out, width, writer).unwrap();
    test();
}

fn test() {
    let _a = vec![1, 2, 3];
    let _b = vec![4, 5, 6];
}