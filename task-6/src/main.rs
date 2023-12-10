use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

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

    let mut gear_ratios: HashMap<(usize, usize), Vec<i64>> = HashMap::new();

    for (x, row) in layout.iter().enumerate() {
        let mut current_number = String::new();
        let mut gears = HashSet::new();

        let mut do_flush = |current_number: &mut String, gears: &mut HashSet<(usize, usize)>| {
            if current_number.is_empty() {
                return;
            }
            let number: i64 = current_number.parse().unwrap();
            for &gear in gears.iter() {
                if let Some(parts) = gear_ratios.get_mut(&gear) {
                    parts.push(number);
                } else {
                    gear_ratios.insert(gear, Vec::from([number]));
                }
            }
            current_number.clear();
            gears.clear();
        };

        for (y, ch) in row.iter().enumerate() {
            if !ch.is_numeric() {
                do_flush(&mut current_number, &mut gears);
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
                    if *neigbor == '*' {
                        gears.insert((nx as usize, ny as usize));
                    }
                }
            }
        }

        do_flush(&mut current_number, &mut gears);
    }

    let result: i64 = gear_ratios
        .iter()
        .map(|x| {
            if x.1.len() > 1 {
                x.1.iter().product()
            } else {
                0
            }
        })
        .sum();

    println!("result: {}", result);
}
