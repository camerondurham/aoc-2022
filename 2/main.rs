use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
fn main() {
    let score_map: HashMap<&str, i32> = [
        ("A X", 4),
        ("B X", 1),
        ("C X", 7),
        ("A Y", 8),
        ("B Y", 5),
        ("C Y", 2),
        ("A Z", 3),
        ("B Z", 9),
        ("C Z", 6),
    ]
    .iter()
    .cloned()
    .collect();
    let mut score = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            let round = score_map.get(&line.unwrap().as_str()).unwrap().clone();
            score += round;
        }
    }
    println!("final score: {}", score);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
