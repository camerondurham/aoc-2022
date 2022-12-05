use std::collections::HashMap;
use std::sync::{Arc, Mutex};
/*

            [G] [W]         [Q]
[Z]         [Q] [M]     [J] [F]
[V]         [V] [S] [F] [N] [R]
[T]         [F] [C] [H] [F] [W] [P]
[B] [L]     [L] [J] [C] [V] [D] [V]
[J] [V] [F] [N] [T] [T] [C] [Z] [W]
[G] [R] [Q] [H] [Q] [W] [Z] [G] [B]
[R] [J] [S] [Z] [R] [S] [D] [L] [J]
 1   2   3   4   5   6   7   8   9
01234567890123456789012345678901234

*/

enum States {
    ParseBoxes,
    ParseMoves,
}

#[derive(Debug)]
struct Move {
    count: i32,
    from: i32,
    to: i32,
}

fn main() {
    let input = include_str!("input.txt");
    let mut rows: Vec<Vec<char>> = Vec::new();
    let mut state = States::ParseBoxes;
    let mut map: Option<HashMap<i32, Vec<char>>> = None;

    let mut moves: Vec<Move> = Vec::new();
    for line in input.split("\n") {
        match &state {
            States::ParseBoxes => {
                if line.len() == 0 {
                    // need to pivot the table
                    map = Some(pivot(rows.clone()));
                    state = States::ParseMoves;
                    continue;
                }

                println!("{}", line);
                let cols = parse_columns(line, 1, 4);
                println!("parsed: {:?}", cols);
                if cols.get(0).unwrap().is_alphabetic() || cols.get(0).unwrap().is_whitespace() {
                    rows.push(cols);
                }
            }
            States::ParseMoves => {
                if line.len() == 0 {
                    continue;
                }
                let parsed = parse_move(line);
                println!("parsed move: {:?}", parsed);
                moves.push(parsed);
            }
        }
    }
    let arc: Arc<Mutex<HashMap<i32, Vec<char>>>> = Arc::new(Mutex::new(map.unwrap()));
    for mv in moves.iter() {
        // TODO: make this less memory intensive
        let mut f = arc.lock().unwrap().get(&(mv.from - 1)).unwrap().clone();
        let mut t = arc.lock().unwrap().get(&(mv.to - 1)).unwrap().clone();
        // Part 1:
        // for _ in 0..mv.count {
        //     let item = f.pop().unwrap();
        //     t.push(item);
        // }
        // Part 2:
        let sliced = f[f.len() - mv.count as usize..f.len()].to_vec();
        for c in sliced {
            f.pop();
            t.push(c);
        }
        arc.lock().unwrap().insert(mv.from - 1, f);
        arc.lock().unwrap().insert(mv.to - 1, t);
    }
    println!("Final state: {:?}", arc.lock().unwrap());
    println!("");
    let mut final_state = String::new();
    for i in 0..9 {
        final_state.push(arc.lock().unwrap().get(&i).unwrap().last().unwrap().clone());
    }
    println!("final state: {}", final_state);
}

fn parse_columns(line: &str, start: i32, offset: i32) -> Vec<char> {
    // [G] [R] [Q] [H] [Q] [W] [Z] [G] [B]
    // 01234567890123456789012345678901234
    let chars = line.chars().collect::<Vec<char>>().to_vec();
    let mut ret = Vec::<char>::new();
    for i in (start as usize)..(line.len()) {
        if (i - start as usize) % offset as usize == 0 {
            ret.push(chars.get(i).unwrap().clone());
        }
    }
    ret
}

fn parse_move(line: &str) -> Move {
    let split_line = line.split(" ").collect::<Vec<&str>>();
    if split_line.len() == 0 {
        panic!("unexpected line");
    }
    let count = split_line.get(1).unwrap().parse::<i32>().unwrap();
    let from = split_line.get(3).unwrap().parse::<i32>().unwrap();
    let to = split_line.get(5).unwrap().parse::<i32>().unwrap();

    Move { count, from, to }
}

fn pivot(input: Vec<Vec<char>>) -> HashMap<i32, Vec<char>> {
    println!("called with input: {:?}", input);
    let mut pivoted_map: HashMap<i32, Vec<char>> = HashMap::new();
    for i in 0..input.get(0).unwrap().len() {
        pivoted_map.insert(i as i32, vec![]);
    }
    println!("pivoted map len: {}", pivoted_map.len());
    for vec in input {
        for i in 0..vec.len() {
            let val = vec.get(i).unwrap().clone();
            if val.is_alphabetic() {
                pivoted_map.get_mut(&(i as i32)).unwrap().push(val);
            }
        }
    }
    println!("pivoted map before: {:?}", &pivoted_map);
    let mut pivoted_reversed: HashMap<i32, Vec<char>> = HashMap::new();
    for (key, mut val) in pivoted_map {
        val.reverse();
        pivoted_reversed.insert(key, val.clone());
    }
    println!("pivoted map after: {:?}", &pivoted_reversed);
    pivoted_reversed
}
