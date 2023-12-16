use std::{env, fs, iter::Iterator};

fn match_first_digit_part1(line : String) -> u32 {
    line.chars().find_map(|c| c.to_digit(10)).unwrap()
}

fn match_first_digit_part2(line : String) -> u32 {
    todo!()
}

fn match_first_digit(line : String, part_two : bool) -> u32 {
    if part_two {
        match_first_digit_part2(line)
    } else {
        match_first_digit_part1(line)
    }
}

fn main() {
    let args : Vec<String> = env::args().collect(); 
    let part_two = args[1].parse::<bool>().unwrap();
    let fpath = &args[2];

    let res = fs::read_to_string(fpath).unwrap().lines()
        .fold(0, |res, line|
            res
            + match_first_digit(line.chars().collect(), part_two) * 10 
            + match_first_digit(line.chars().rev().collect(), part_two)
        );
    dbg!(res);
}
