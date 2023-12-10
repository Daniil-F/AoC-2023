use std::{collections::BTreeSet, io::BufRead};

fn read_line(input_buffer: &mut std::io::BufReader<std::fs::File>) -> Option<String> {
    let mut record = String::new();

    let bytes_read = input_buffer
        .read_line(&mut record)
        .expect("failed to read from file");
    if bytes_read == 0 {
        return None;
    }

    return Some(record.trim().to_string());
}

fn main() {
    let input_filename = std::env::args().nth(1).expect("No input filename given!");

    let input_file = std::fs::File::open(&input_filename)
        .expect(&std::format!("Cannot read file at {}!", &input_filename));

    let mut input_buffer = std::io::BufReader::new(input_file);

    let mut items: BTreeSet<i64> = read_line(&mut input_buffer)
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    assert_eq!(read_line(&mut input_buffer), Some(String::from("")));

    for _ in 0..7 {
        let mut new_items = BTreeSet::new();

        read_line(&mut input_buffer);

        while let Some(record) = read_line(&mut input_buffer) {
            if record.is_empty() {
                break;
            }

            let [to_start, from_start, amount] = record
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()[..] else {
                panic!("Not three numbers in array");
            };
            let from_end = from_start + amount;
            let diff = to_start - from_start;
            let matching_items: Vec<i64> = items.range(from_start..from_end).map(|x| x.clone()).collect();
            
            new_items.extend(matching_items.iter().map(|x| *x + diff));

            for item in matching_items {
              items.remove(&item);
            }
        }

        items.append(&mut new_items);
    }

    println!("result: {}", items.first().unwrap());
}
