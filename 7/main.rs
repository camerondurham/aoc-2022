use std::collections::HashMap;
enum FType {
    File,
    Dir,
}
struct File {
    filename: String,
    ftype: FType,
    size: i32,
    subdirs: Option<HashMap<String, File>>,
}
fn main() {
    let mut root = File {
        filename: String::from("/"),
        ftype: FType::Dir,
        size: 0,
        subdirs: None,
    };
    let input = include_str!("input.txt");
    for line in input.lines() {
        if line.chars().nth(0).unwrap() == '$' {}
    }
}

#[cfg(test)]
mod test {
    // use crate::FUNCTION_NAME;
    #[test]
    fn test() {
        assert_eq!(1 + 1, 2);
    }
}
