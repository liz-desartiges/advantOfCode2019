use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let file = File::open("input").unwrap();
    let file = BufReader::new(file);
    let fuel: u64 = file.lines().fold(0u64, |sum, x| {
        sum + calc_fuel(x.unwrap().parse().unwrap())
    });
    println!("{}",  fuel);
}

fn calc_fuel(weight: u64) -> u64 {
    weight / 3 - 2
}
