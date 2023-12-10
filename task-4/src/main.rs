use std::{collections::HashMap, io::BufRead};

fn main() {
    let input_filename = std::env::args().nth(1).expect("No input filename given!");

    let input_file = std::fs::File::open(&input_filename)
        .expect(&std::format!("Cannot read file at {}!", &input_filename));

    let mut input_buffer = std::io::BufReader::new(input_file);

    let mut result: i64 = 0;

    loop {
        let mut record = String::new();
        let bytes_read = input_buffer
            .read_line(&mut record)
            .expect("failed to read from file");
        if bytes_read == 0 {
            break;
        }

        let mut record_split = record.split(":");
        let game_str = record_split.nth(1).unwrap();

        let mut color_count = HashMap::new();

        for round in game_str.split(';') {
            for color_info in round.split(',') {
                let mut color_info_split = color_info.split_whitespace();
                let amount: i32 = color_info_split.next().unwrap().parse().unwrap();
                let color = color_info_split.next().unwrap();

                if !color_count.contains_key(color) {
                  color_count.insert(color, amount);
                } else {
                  *color_count.get_mut(color).unwrap() = std::cmp::max(amount, color_count[color]);
                }
            }
        }

        let mut power: i64 = 1;

        for (_, amount) in color_count.iter() {
          power *= *amount as i64;
        }

        result += power;
    }

    println!("result: {}", result);
}
