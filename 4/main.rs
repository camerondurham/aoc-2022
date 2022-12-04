/*
 * Some of the pairs have noticed that one of their assignments fully contains the other. For
 * example, 2-8 fully contains 3-7, and 6-6 is fully contained by 4-6. In pairs where one
 * assignment fully contains the other, one Elf in the pair would be exclusively cleaning sections
 * their partner will already be cleaning, so these seem like the most in need of reconsideration.
 * In this example, there are 2 such pairs.

 * In how many assignment pairs does one range fully contain the other?

Example:
.234.....  2-4
.....678.  6-8

.23......  2-3
...45....  4-5

....567..  5-7
......789  7-9

.2345678.  2-8
..34567..  3-7

.....6...  6-6
...456...  4-6

.23456...  2-6
...45678.  4-8


Input Format:

2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8


 */
fn main() {
    let input = include_str!("input.txt");
    let mut count = 1;
    let mut full_contains = 0;
    for line in input.split("\n") {
        if line.len() <= 1 {
            continue;
        }
        let pair = line.split(",").collect::<Vec<&str>>();
        let (ll, lr) = get_range(pair.get(0).unwrap());
        let (rl, rr) = get_range(pair.get(1).unwrap());
        let contains = (ll <= rl && lr >= rr) || (rl <= ll && rr >= lr);
        println!(
            "{}: ({},{}) - ({},{}) - contains? {}",
            count, ll, lr, rl, rr, contains
        );
        count += 1;
        if contains {
            full_contains += 1;
        }
    }
    println!("total: {}", full_contains); // 498
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
