use common::*;

fn combination_strength(cards: &str) -> i64 {
    let mut chars: Vec<_> = cards.chars().collect();
    chars.sort();

    let mut current_counter = None;

    let mut card_counters = Vec::new();

    for c in chars.iter() {
        if let Some((x, ref mut cnt)) = current_counter {
            if x == c {
                *cnt += 1;
                continue;
            }
            card_counters.push((x, *cnt));
        }
        current_counter = Some((c, 1));
    }

    card_counters.push(current_counter.unwrap());

    card_counters.sort_by_key(|x| -x.1);

    let strength = match card_counters[..] {
        [(_, 5)] => 7,
        [(_, 4), (_, 1)] => 6,
        [(_, 3), (_, 2)] => 5,
        [(_, 3), ..] => 4,
        [(_, 2), (_, 2), ..] => 3,
        [(_, 2), ..] => 2,
        _ => 1,
    };

    return strength;
}

fn main() {
    let card_ordering: Vec<_> = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ]
    .iter()
    .rev()
    .enumerate()
    .collect();

    let card_order = |c: char| card_ordering.iter().find(|(_, &o)| o == c).unwrap().0;

    let mut input_buffer = get_input_buffer();

    let mut hands = Vec::new();

    while let Some((cards, bid_str)) = read_kv_line_whitespace_separated(&mut input_buffer) {
        hands.push((cards, bid_str.parse::<i64>().unwrap()));
    }

    hands.sort_by_cached_key(|(cards, _)| {
        (
            combination_strength(cards),
            cards.chars().map(card_order).collect::<Vec<_>>(),
        )
    });

    let mut result = 0i64;

    for (idx, (_, bid)) in hands.iter().enumerate() {
        result += (idx as i64 + 1i64) * bid;
    }

    println!("result is {}", result);
}
