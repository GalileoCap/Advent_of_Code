use regex::Regex;
use lazy_static::lazy_static;

type RES = u32;

fn is_subset_possible(subset : &String) -> bool {
    if subset.starts_with("Game") { // Hack for the first part
        return true
    }

    lazy_static!{
        static ref RED : Regex = Regex::new(r"(\d+) red").unwrap();
        static ref GREEN : Regex = Regex::new(r"(\d+) green").unwrap();
        static ref BLUE : Regex = Regex::new(r"(\d+) blue").unwrap();
    }

    RED.captures_iter(&subset).fold(0, |ret, cap| ret + cap[1].parse::<u32>().unwrap()) <= 12 &&
    GREEN.captures_iter(&subset).fold(0, |ret, cap| ret + cap[1].parse::<u32>().unwrap()) <= 13 &&
    BLUE.captures_iter(&subset).fold(0, |ret, cap| ret + cap[1].parse::<u32>().unwrap()) <= 14
}

fn is_game_possible(game : &String) -> bool {
    game[game.find(":").unwrap()..].split(";").all(|subset| is_subset_possible(&subset.to_string()))
}

fn solve_part_one(file : &String) -> RES {
    lazy_static!{
        static ref ID : Regex = Regex::new(r"Game (\d+)").unwrap();
    }

    file.lines().fold(0,
        |res, line|
            res + (if is_game_possible(&line.to_string()) {
                ID.captures(&line.to_string()).unwrap()[1].parse().unwrap()
            } else { 0 })
    )
}

fn solve_part_two(file : &String) -> RES {
    todo!()
}

fn solve(file : &String, part_two : bool) -> RES {
    if part_two {
        solve_part_two(file)
    } else {
        solve_part_one(file)
    }
}

fn main() {
    use std::fs::read_to_string;
    let file_1 = &read_to_string("inout/1.example.in").unwrap();
    let file_2 = &read_to_string("inout/2.example.in").unwrap();
    let file = &read_to_string("inout/input").unwrap();

    for (i, (file, expected)) in
        [file_1, file, file_2, file].iter()
            .zip([8, 2164, 0, 0])
            .enumerate()
        {
        use std::time::Instant;
        let part_two = i >= 2;

        let now = Instant::now();
        let res = solve(file, part_two);
        let elapsed = now.elapsed();
        assert_eq!(expected, res);
        println!("{i}: {:?} {} ", elapsed, res);
    }
}
