use regex::Regex;
use lazy_static::lazy_static;

type RES = u32;

fn count_subset(subset : &String) -> (u32, u32, u32) {
    lazy_static!{
        static ref RE : Regex = Regex::new(r"(?<count>\d+) (?<color>\w+)").unwrap();
    }
    RE.captures_iter(&subset).fold((0, 0, 0), |(r, g, b), caps| {
        let count : u32 = caps.name("count").unwrap().as_str().parse().unwrap();
        match caps.name("color").unwrap().as_str() {
            "red" => (r + count, g, b),
            "green" => (r, g + count, b),
            "blue" => (r, g, b + count),
            _ => panic!(),
        }
    })
}

fn is_subset_possible(subset : &String) -> bool {
    let (r, g, b) = count_subset(subset);
    r <= 12 && g <= 13 && b <= 14
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
