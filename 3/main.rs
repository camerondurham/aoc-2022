use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
fn main() {
    // TODO: try to make this cleaner, more idiomatic
    let mut score = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            let chars: Vec<char> = line.as_ref().unwrap().chars().collect();
            let midpoint = line.as_ref().unwrap().len() / 2;
            let mut lcharset: HashSet<char> = HashSet::new();
            for i in 0..midpoint {
                lcharset.insert(chars.get(i).unwrap().clone());
            }
            let mut rcharset: HashSet<char> = HashSet::new();
            for i in midpoint..line.as_ref().unwrap().len() {
                rcharset.insert(chars.get(i).unwrap().clone());
            }
            let intersection: Vec<&char> = lcharset.intersection(&rcharset).collect();
            if intersection.len() != 0 {
                score += get_value(intersection.get(0).unwrap());
            }
        }
    }
    println!("the score is: {}", score);
}

// From: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn get_value(c: &char) -> i32 {
    let val = c.clone() as i32;
    if val >= 'a' as i32 && val <= 'z' as i32 {
        return val - 'a' as i32 + 1;
    } else if val >= 'A' as i32 && val <= 'Z' as i32 {
        return val - 'A' as i32 + 1 + 26;
    } else {
        return -1;
    }
}
