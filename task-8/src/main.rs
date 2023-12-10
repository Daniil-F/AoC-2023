use std::{collections::{HashSet}, io::BufRead};

fn main() {
    let input_filename = std::env::args().nth(1).expect("No input filename given!");

    let input_file = std::fs::File::open(&input_filename)
        .expect(&std::format!("Cannot read file at {}!", &input_filename));

    let mut input_buffer = std::io::BufReader::new(input_file);

    let mut result: i64 = 0;
    let mut amounts = Vec::new();

    let mut idx = 0;

    loop {
        let mut record = String::new();
        let bytes_read = input_buffer
            .read_line(&mut record)
            .expect("failed to read from file");
        if bytes_read == 0 {
            break;
        }

        if amounts.len() <= idx {
          amounts.push(0 as i64);
        }

        amounts[idx] += 1;

        let amount = amounts[idx];
        result += amount;

        let mut card_str_parts = record.split(":").nth(1).unwrap().split("|");

        let card_numbers: Vec<i64> = card_str_parts.next().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let winning_numbers: HashSet<i64> = card_str_parts.next().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

        let cards_won = card_numbers.iter().filter(|x| winning_numbers.contains(x)).count();
        for won_idx in idx+1..idx+cards_won+1 {
          if amounts.len() <= won_idx {
            amounts.push(0);
          }
          amounts[won_idx] += amount;
        }

        idx += 1;
    }

    println!("result: {}", result);
}
