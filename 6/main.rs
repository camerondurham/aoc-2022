use std::collections::HashMap;

// Part 1: 1198
// Part 2: 3120

enum States {
    Loading,
    Checking,
    Found,
}

const LENGTH: usize = 14;
fn main() {
    let input = include_str!("input.txt");
    let mut i = 0;
    let mut m = HashMap::<char, i32>::new();
    let mut state = States::Loading;
    let char_vec = input.chars().collect::<Vec<char>>();
    loop {
        if i == char_vec.len() {
            break;
        }
        match &state {
            States::Loading => {
                if i == LENGTH - 1 {
                    state = States::Checking;
                    continue;
                }
                let c = char_vec.get(i).unwrap();
                if let Some(v) = m.get(c) {
                    m.insert(c.clone(), v + 1);
                } else {
                    m.insert(c.clone(), 1);
                }
                i += 1;
            }
            States::Checking => {
                let c = char_vec.get(i).unwrap();
                println!("checking: {} {}", i, c);
                if let Some(v) = m.get(c) {
                    m.insert(c.clone(), v + 1);
                } else {
                    m.insert(c.clone(), 1);
                }

                let count = m.iter().filter(|(_, v)| **v == 1 as i32).count();
                if count == LENGTH {
                    println!("found a match, map is: {:?}", &m);
                    state = States::Found;
                } else {
                    i += 1;
                }
                let prev_char = char_vec.get(i - LENGTH).unwrap();
                let prev_val = m.get(prev_char).unwrap();
                m.insert(prev_char.clone(), prev_val - 1);
            }
            States::Found => {
                println!("found solution: count={}", i + 1);
                break;
            }
        }
    }
}
