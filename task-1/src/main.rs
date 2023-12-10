use std::io::BufRead;

fn main() {
    let input_filename = std::env::args().nth(1).expect("No input filename given!");

    let input_file = std::fs::File::open(&input_filename)
        .expect(&std::format!("Cannot read file at {}!", &input_filename));

    let mut input_buffer = std::io::BufReader::new(input_file);

    let mut result: u64 = 0;

    loop {
        let mut record = String::new();
        let bytes_read = input_buffer.read_line(&mut record).expect("failed to read from file");
        if bytes_read == 0 {
          break;
        }

        let first_digit = record.as_str().chars().find(|&x| x.is_numeric()).expect("No digits in line!");
        let last_digit = record.as_str().chars().rfind(|&x| x.is_numeric()).expect("No digits in line!");

        let original_record = first_digit.to_string() + &last_digit.to_string();
        let original_record_number: u64 = original_record.parse().unwrap();
        result += original_record_number;
    }

    println!("result: {}", result);
}
