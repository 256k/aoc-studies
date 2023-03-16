use std::fs::File;
use std::io::{BufRead, BufReader};

fn group_values() -> Vec<i32> {
    // create a function that returns a vector array of type i32
    let mut totals = Vec::new();
    let mut temp_sum = 0;
    let mut interator = 0;

    let path = "assets/data.txt";
    let input = File::open(path).unwrap();
    let data_buffer = BufReader::new(input);

    for line in data_buffer.lines() {
        match line {
            Ok(line) => {
                if !line.is_empty() {
                    let line_val = line.trim().parse::<i32>();
                    match line_val {
                        Ok(val) => {
                            temp_sum += val;
                        },
                        Err(e) =>{println!("{}", e)},
                    }
                } else {
                    totals.push(temp_sum);
                    interator += 1;
                    println!("{}: temp_sum => {}", interator, temp_sum);
                    temp_sum = 0;
                }
            }
            Err(err) => {
                panic!("Error => {}", err);
            }
        };
    }
    totals
}

fn get_biggest(totals: &Vec<i32>) -> i32 {
    let mut biggest = 0;

    let mut interator = 0;
    for total in totals {
        interator += interator;
        if *total > biggest {
            biggest = *total
        };
    }
    biggest
}
fn main() {
    let totals = group_values();
    println!("Result: {}", get_biggest(&totals));
}
