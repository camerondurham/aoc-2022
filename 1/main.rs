use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    if args.len() > 2 {
        let filename = args.get(2).unwrap();
        println!("Opening file: {:?}", filename);
        let file = cat(Path::new(filename)).unwrap();
        println!("File: {}", &file);
    }
}

fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
