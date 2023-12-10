use std::{fmt::Debug, io::BufRead, str::FromStr};

pub type InputBuffer = std::io::BufReader<std::fs::File>;

pub fn get_input_buffer() -> InputBuffer {
    let input_filename = std::env::args().nth(1).expect("No input filename given!");

    let input_file = std::fs::File::open(&input_filename)
        .expect(&std::format!("Cannot read file at {}!", &input_filename));

    return std::io::BufReader::new(input_file);
}

pub fn read_line(input_buffer: &mut std::io::BufReader<std::fs::File>) -> Option<String> {
    let mut record = String::new();

    let bytes_read = input_buffer
        .read_line(&mut record)
        .expect("failed to read from file");
    if bytes_read == 0 {
        return None;
    }

    return Some(record.trim().to_string());
}

pub fn read_kv_line(
    input_buffer: &mut std::io::BufReader<std::fs::File>,
    kv_sep: &str,
) -> Option<(String, String)> {
    if let Some(line) = read_line(input_buffer) {
        let mut iter = line.split(kv_sep);
        let key = iter.next().unwrap();
        let value = iter.next().unwrap();
        return Some((String::from(key), String::from(value)));
    } else {
        return None;
    }
}

pub fn read_kv_line_whitespace_separated(
    input_buffer: &mut std::io::BufReader<std::fs::File>,
) -> Option<(String, String)> {
    if let Some(line) = read_line(input_buffer) {
        let mut iter = line.trim().split_whitespace();
        let key = iter.next().unwrap();
        let value = iter.next().unwrap();
        return Some((String::from(key), String::from(value)));
    } else {
        return None;
    }
}

pub trait ParseCollection<ItemType> {
    fn parse_collection<B: FromIterator<ItemType>>(&self, sep: &str) -> B;

    fn parse_collection_whitespace_separated<B: FromIterator<ItemType>>(&self) -> B;
}

impl<Item: FromStr> ParseCollection<Item> for str
where
    Item::Err: Debug,
{
    fn parse_collection<B: FromIterator<Item>>(&self, sep: &str) -> B {
        return self.trim().split(sep).map(|x| x.parse().unwrap()).collect();
    }

    fn parse_collection_whitespace_separated<B: FromIterator<Item>>(&self) -> B {
        return self
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
    }
}
