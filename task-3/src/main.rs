use std::{io::BufRead, collections::HashMap};

fn main() {
    let input_filename = std::env::args().nth(1).expect("No input filename given!");

    let input_file = std::fs::File::open(&input_filename)
        .expect(&std::format!("Cannot read file at {}!", &input_filename));

    let mut input_buffer = std::io::BufReader::new(input_file);

    let mut result: u64 = 0;

    let color_limitations = HashMap::from(
      [
        ("red", 12),
        ("green", 13),
        ("blue", 14),
      ]
    );

    loop {
        let mut record = String::new();
        let bytes_read = input_buffer
            .read_line(&mut record)
            .expect("failed to read from file");
        if bytes_read == 0 {
            break;
        }

        let mut record_split = record.split(":");
        let game_id_str = record_split.next().unwrap();
        let game_str = record_split.next().unwrap();

        let game_id: u64 = game_id_str.split_whitespace().nth(1).unwrap().parse().unwrap();

        let mut is_valid = true;
        for round in game_str.split(';') {
          for color_info in round.split(',') {
            let mut color_info_split = color_info.split_whitespace();
            let amount: i32 = color_info_split.next().unwrap().parse().unwrap();
            let color = color_info_split.next().unwrap();

            if !color_limitations.contains_key(color) || color_limitations[color] < amount {
              is_valid = false;
            }
          }
        }

        if is_valid {
          result += game_id;
        }
    }

    println!("result: {}", result);
}
