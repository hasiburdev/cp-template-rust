use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    let mut stdin = stdin.lock(); // create buffered reader
    stdin.read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    for i in 0..n {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let input_vec: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        println!("{}", result);
    }
}
