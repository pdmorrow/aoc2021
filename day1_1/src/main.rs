use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut prev_depth: Option<u16> = None;
    let mut count = 0;

    for (_, line) in reader.lines().enumerate() {
        let cur_depth = line.unwrap().parse::<u16>().unwrap();
        if prev_depth.is_some() {
            if cur_depth > prev_depth.unwrap() {
                count += 1;
            }
        }

        prev_depth = Some(cur_depth);
    }

    println!("{}", count);

}
