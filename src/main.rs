// use std::io;
// use std::io::BufRead;
mod leetcode;

fn main() {
    // let stdin = io::stdin();
    // let mut input = String::new();
    // let mut stdin = stdin.lock(); // create buffered reader
    // stdin.read_line(&mut input).unwrap();
    // let n: i32 = input.trim().parse().unwrap();

    // for i in 0..n {
    //     input.clear();
    //     stdin.read_line(&mut input).unwrap();
    //     let input_vec: Vec<i32> = input
    //         .trim()
    //         .split_whitespace()
    //         .map(|x| x.parse().unwrap())
    //         .collect();

    //     println!("{}", result);
    // }
    println!(
        "{}",
        leetcode::Solution::contains_duplicate(vec![1, 2, 3, 4])
    );
    println!(
        "{}",
        leetcode::Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2])
    );
    println!(
        "{}",
        leetcode::Solution::contains_duplicate(vec![1, 2, 3, 4])
    );
}
