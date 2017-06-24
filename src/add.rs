use std::io::*;

fn main() {
    let mut line = String::new();

    stdin().read_to_string(&mut line).ok();

    let numbers: Vec<_> = line.trim().split(' ')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    println!("{}", &numbers[0] + &numbers[1]);
}
