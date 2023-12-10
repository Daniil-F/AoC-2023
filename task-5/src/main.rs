use std::io::BufRead;

type Layout = Vec<Vec<char>>;

const DELTAS: [(i32, i32); 8] = [
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
];

fn get_layout(mut input_buffer: std::io::BufReader<std::fs::File>) -> Layout {
    let mut layout: Layout = Vec::new();

    loop {
        let mut record = String::new();
        let bytes_read = input_buffer
            .read_line(&mut record)
            .expect("failed to read from file");
        if bytes_read == 0 {
            break;
        }

        layout.push(record.trim_end().chars().collect());
        assert_eq!(layout.first().unwrap().len(), layout.last().unwrap().len());
    }

    return layout;
}

fn main() {
    let input_filename = std::env::args().nth(1).expect("No input filename given!");

    let input_file = std::fs::File::open(&input_filename)
        .expect(&std::format!("Cannot read file at {}!", &input_filename));

    let layout = get_layout(std::io::BufReader::new(input_file));

    let mut result: i64 = 0;

    for (x, row) in layout.iter().enumerate() {
        let mut current_number = String::new();
        let mut is_part_number = false;
        let mut part_symbol = ' ';

        let mut do_flush =
            |current_number: &mut String, is_part_number: &mut bool, part_symbol: &char| {
                if current_number.is_empty() {
                    return;
                }
                if *is_part_number {
                    println!(
                        "Got {} as part number with {}",
                        current_number, &part_symbol
                    );
                    result += current_number.parse::<i64>().unwrap();
                } else {
                    println!("Got {} as not part number", current_number);
                }
                current_number.clear();
                *is_part_number = false;
            };

        for (y, ch) in row.iter().enumerate() {
            if !ch.is_numeric() {
                do_flush(&mut current_number, &mut is_part_number, &part_symbol);
                continue;
            }
            current_number += &ch.to_string();

            for (dx, dy) in DELTAS {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx < 0 || ny < 0 {
                    continue;
                }

                if let Some(neigbor) = layout.get(nx as usize).and_then(|row| row.get(ny as usize))
                {
                    if !neigbor.is_numeric() && *neigbor != '.' {
                        is_part_number = true;
                        part_symbol = *neigbor;
                        break;
                    }
                }
            }
        }

        do_flush(&mut current_number, &mut is_part_number, &part_symbol);
    }

    println!("result: {}", result);
}
