use std::fs;
use std::env;

fn main() {
    let mut data: Vec<String> = fs::read_to_string("calibrationDocument.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    
    for d in data {
        let mut numbers = Vec::new();

        for c in d.chars() {
            let n:i32 = c as i32 - 0x30;
            numbers.push(n);
        }

        let sum = numbers.first
        
        println!("{}", d);

    }
}
