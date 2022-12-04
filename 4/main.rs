// https://adventofcode.com/2022/day/4
fn main() {
    let input = include_str!("input.txt");
    let mut full_contains = 0;
    let mut partially_contains = 0;
    for line in input.split("\n") {
        if line.len() <= 1 {
            continue;
        }
        let pair = line.split(",").collect::<Vec<&str>>();
        let (ll, lr) = get_range(pair.get(0).unwrap());
        let (rl, rr) = get_range(pair.get(1).unwrap());

        // fully overlaps
        if (ll <= rl && lr >= rr) || (rl <= ll && rr >= lr) {
            full_contains += 1;
        }
        // partial overlap
        if !((lr < rl) || (rr < ll)) {
            partially_contains += 1;
        }
    }
    println!(
        "fully_overlaps: {}, partially_overlaps: {}",
        full_contains, partially_contains
    );
}

fn get_range(s: &str) -> (i32, i32) {
    let v = s.split("-").collect::<Vec<&str>>();
    (v[0].parse::<i32>().unwrap(), v[1].parse::<i32>().unwrap())
}

#[cfg(test)]
mod test {
    use crate::get_range;

    #[test]
    fn test1() {
        let (a, b) = get_range("9-20");
        assert_eq!(a, 9);
        assert_eq!(b, 20);
    }
}
