use std::{env, fs};

fn main() {
    let args : Vec<String> = env::args().collect(); 
    let fpath = &args[1];

    let mut res = 0;
    for line in fs::read_to_string(fpath).unwrap().lines() {
        for c in line.chars() {
            if let Some(x) = c.to_digit(10) {
                res += x * 10;
                break;
            }
        }

        for c in line.chars().rev() {
            if let Some(x) = c.to_digit(10) {
                res += x;
                break;
            }
        }
    }
    dbg!(res);
}
