use std::{
    cmp::{max, min},
    collections::BTreeMap,
    io::BufRead,
};

struct SegmentUnion {
    segment_ends: BTreeMap<i64, i64>,
}

impl SegmentUnion {
    fn new() -> SegmentUnion {
        return SegmentUnion {
            segment_ends: BTreeMap::new(),
        };
    }

    fn add_segment(&mut self, l: i64, r: i64) {
        let mut current_l = l;
        let mut current_r = r;

        let mut ends_to_remove = Vec::new();

        if let Some((pred_l, pred_r)) = self.segment_ends.range(..l).last() {
            if *pred_r >= l {
                current_l = *pred_l;
                current_r = max(current_r, *pred_r);
                ends_to_remove.push(*pred_l);
            }
        }

        for (inter_l, inter_r) in self.segment_ends.range(l..r + 1) {
            current_r = max(current_r, *inter_r);
            ends_to_remove.push(*inter_l);
        }

        for end_to_remove in ends_to_remove {
            self.segment_ends.remove(&end_to_remove);
        }

        self.segment_ends.insert(current_l, current_r);
    }

    fn cut_segment(&mut self, l: i64, r: i64) -> Vec<(i64, i64)> {
        let mut result = Vec::new();
        let mut rem_r = None;

        if let Some((_, pred_r)) = self.segment_ends.range_mut(..l).last() {
            if *pred_r > l {
                result.push((l, min(r, *pred_r)));
                if *pred_r > r {
                    rem_r = Some(*pred_r)
                }
                *pred_r = l;
            }
        }

        for (inter_l, inter_r) in self.segment_ends.range(l..r) {
            result.push((*inter_l, min(*inter_r, r)));
            if *inter_r > r {
                rem_r = Some(*inter_r);
            }
        }

        for (res_l, _) in result.iter() {
            self.segment_ends.remove(&res_l);
        }

        if let Some(rem_r) = rem_r {
            self.segment_ends.insert(r, rem_r);
        }

        return result;
    }

    fn from<I: Iterator<Item = (i64, i64)>>(iter: I) -> SegmentUnion {
        let mut result = SegmentUnion::new();

        for (l, r) in iter {
            result.add_segment(l, r);
        }

        return result;
    }
}

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

    let mut segments = SegmentUnion::from(
        read_line(&mut input_buffer)
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .scan(None, |st, x: i64| {
                if st.is_some() {
                    let l = st.unwrap();
                    *st = None;
                    return Some(Some((l, x)));
                } else {
                    *st = Some(x);
                    return Some(None);
                }
            })
            .filter(|x| x.is_some())
            .map(|x| {
                let Some((l, len)) = x else {
                  panic!("Shouldn't be here")
                };
                return (l, l + len);
            }),
    );

    assert_eq!(read_line(&mut input_buffer), Some(String::from("")));

    for _ in 0..7 {
        read_line(&mut input_buffer);

        let mut new_segments = Vec::new();

        while let Some(record) = read_line(&mut input_buffer) {
            if record.is_empty() {
                break;
            }

            let [to_l, from_l, len] = record
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()[..]
            else {
                panic!("Not three numbers in array");
            };

            let from_r = from_l + len;
            let diff = to_l - from_l;
            new_segments.extend(
                segments
                    .cut_segment(from_l, from_r)
                    .iter()
                    .map(|(x, y)| (x + diff, y + diff)),
            );
        }

        for (l, r) in new_segments {
          segments.add_segment(l, r);
        }
    }

    println!("result: {}", segments.segment_ends.first_entry().unwrap().key());
}
