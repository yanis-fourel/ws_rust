use std::fs::read_to_string;

fn main() {
    let mut sum = 0;

    for line in read_to_string("./input.txt").unwrap().lines() {
        println!("{}", line);
    }

    println!("Sum is {}", sum);
}
