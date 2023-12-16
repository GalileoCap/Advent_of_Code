use std::{env, fs, iter::Iterator};
use lazy_static::lazy_static;
use aho_corasick::{AhoCorasick, PatternID};

fn match_first_digit_part1(line : String) -> u32 {
    line.chars().find_map(|c| c.to_digit(10)).unwrap()
}

fn match_first_digit_part2(line : String, rev : bool) -> u32 {
    lazy_static!{
        static ref AC : AhoCorasick = AhoCorasick::new(
            &["1", "one", "2", "two", "3", "three", "4", "four", "5", "five", "6", "six", "7", "seven", "8", "eight", "9", "nine"]
        ).unwrap();
        static ref AC_rev : AhoCorasick = AhoCorasick::new(
            &["1", "eno", "2", "owt", "3", "eerht", "4", "ruof", "5", "evif", "6", "xis", "7", "neves", "8", "thgie", "9", "enin"]
        ).unwrap();
    }

    if rev {
        AC_rev.find(&line).unwrap().pattern().as_u32() / 2 + 1
    } else {
        AC.find(&line).unwrap().pattern().as_u32() / 2 + 1
    }
}

fn match_first_digit(line : String, part_two : bool, rev : bool) -> u32 {
    if part_two {
        match_first_digit_part2(line, rev)
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
            + match_first_digit(line.chars().collect(), part_two, false) * 10 
            + match_first_digit(line.chars().rev().collect(), part_two, true)
        );
    dbg!(res);
}
