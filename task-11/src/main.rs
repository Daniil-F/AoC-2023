use num::BigInt;

use std::iter::zip;

use common::*;

fn main() {
    let mut input_buffer = get_input_buffer();

    let times: Vec<i64> = read_kv_line(&mut input_buffer, ":")
        .unwrap()
        .1
        .parse_collection_whitespace_separated();

    let distances: Vec<i64> = read_kv_line(&mut input_buffer, ":")
        .unwrap()
        .1
        .parse_collection_whitespace_separated();

    let mut result = BigInt::from(1);

    for (time, distance) in zip(times, distances) {
        let discriminant = time * time - distance * 4i64;

        if discriminant <= 0 {
            continue;
        }

        let discr_root = (discriminant as f64).sqrt();

        let mut left_bound = (((time as f64) - discr_root) / 2f64).ceil() as i64;
        let mut right_bound = (((time as f64) + discr_root) / 2f64).floor() as i64;

        let does_win = |windup: &i64| {
          return windup * (&time - windup) > distance;
        };

        let left_bound_clone = left_bound;

        for (value, bound) in [
            (&mut left_bound, right_bound.clone()),
            (&mut right_bound, left_bound_clone),
        ] {
            let initial_behavior = does_win(&value);
            'variate: for step_size in 1i64.. {
                for multiplier in [-1i64, 1i64] {
                  let step = step_size * multiplier;
                  let new_value = *value + step;

                  if new_value == bound && !initial_behavior {
                    break 'variate;
                  }

                  if does_win(&new_value) != initial_behavior {
                    if initial_behavior {
                      *value = new_value - multiplier;
                    } else {
                      *value = new_value;
                    }

                    break 'variate;
                  }
                }
            }
        }

        result *= right_bound - left_bound + 1;
    }

    println!("result is {}", result);
}
