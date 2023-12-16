use std::{env, fs, iter::Iterator};

fn match_first_digit<'a, I>(line : I) -> u32
    where I : Iterator<Item = char>,
    {
    for c in line {
        if let Some(x) = c.to_digit(10) {
            return x;
        }
    }
    panic!();
}

fn main() {
    let args : Vec<String> = env::args().collect(); 
    let fpath = &args[1];

    let res = fs::read_to_string(fpath).unwrap().lines()
        .fold(0, |res, line|
            res + match_first_digit(line.chars()) * 10 + match_first_digit(line.chars().rev())
        );
    dbg!(res);
}
