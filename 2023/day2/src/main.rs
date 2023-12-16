type RES = u32;

fn solve_part_one(file : &String) -> RES {
    todo!()
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
            .zip([8, 0, 0, 0])
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
