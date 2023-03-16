use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() {
    let path = "lines.txt";

    // WRITE FILE
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("problem loading with file {:?}", error);
        }
    };

    write!(output, "some random\nText file\nExtra line\n========").expect("could not write file");

    // READ FILE
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }
}
