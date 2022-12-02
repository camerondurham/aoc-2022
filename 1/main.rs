use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        println!("Insufficient arguments provided");
        std::process::exit(1);
    }
    let filename = args.get(2).unwrap();
    let mut tracker = Vec::<i32>::new();
    if let Ok(lines) = read_lines(filename) {
        let mut inner = Vec::<i32>::new();
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(calories) = line {
                if calories.len() > 0 {
                    inner.push(calories.parse::<i32>().unwrap());
                } else {
                    if inner.len() > 0 {
                        tracker.push(inner.iter().sum());
                    }
                    inner = Vec::<i32>::new();
                }
            }
        }
    }

    println!("max: {:?}", tracker.iter().max().unwrap());
}

// From: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
