use std::io::BufRead;

struct DigitSpelling {
    spelling: String,
    digit: char,
}

fn main() {
    let input_filename = std::env::args().nth(1).expect("No input filename given!");

    let input_file = std::fs::File::open(&input_filename)
        .expect(&std::format!("Cannot read file at {}!", &input_filename));

    let mut input_buffer = std::io::BufReader::new(input_file);

    let mut result: u64 = 0;

    let digit_spellings = Vec::from([
        DigitSpelling {
            spelling: "one".to_string(),
            digit: '1',
        },
        DigitSpelling {
            spelling: "two".to_string(),
            digit: '2',
        },
        DigitSpelling {
            spelling: "three".to_string(),
            digit: '3',
        },
        DigitSpelling {
            spelling: "four".to_string(),
            digit: '4',
        },
        DigitSpelling {
            spelling: "five".to_string(),
            digit: '5',
        },
        DigitSpelling {
            spelling: "six".to_string(),
            digit: '6',
        },
        DigitSpelling {
            spelling: "seven".to_string(),
            digit: '7',
        },
        DigitSpelling {
            spelling: "eight".to_string(),
            digit: '8',
        },
        DigitSpelling {
            spelling: "nine".to_string(),
            digit: '9',
        },
    ]);

    loop {
        let mut record = String::new();
        let bytes_read = input_buffer
            .read_line(&mut record)
            .expect("failed to read from file");
        if bytes_read == 0 {
            break;
        }

        let get_spelled_digit = |idx: usize| {
            for digit_spelling in &digit_spellings {
                if idx + digit_spelling.spelling.len() > record.len() {
                    continue;
                }
                if record[idx..idx + digit_spelling.spelling.len()] == digit_spelling.spelling {
                    return Some(digit_spelling.digit);
                }
            }
            return None;
        };

        let extract_digit = |x: &(usize, char)| {
          if x.1.is_numeric() {
            return x.1;
          }
          return get_spelled_digit(x.0).unwrap();
        };

        let first_digit = record
            .char_indices()
            .find(|&x| x.1.is_numeric() || get_spelled_digit(x.0).is_some())
            .expect("No digits in line!");
        let last_digit = record
            .char_indices()
            .rfind(|&x| x.1.is_numeric() || get_spelled_digit(x.0).is_some())
            .expect("No digits in line!");

        let original_record = extract_digit(&first_digit).to_string() + &extract_digit(&last_digit).to_string();
        let original_record_number: u64 = original_record.parse().unwrap();
        result += original_record_number;
    }

    println!("result: {}", result);
}
