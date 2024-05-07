use std::{collections::HashSet, env, fs, process::exit};
use regex::Regex;

fn update_scratchcards(card_num: usize, num_copies: usize, scratch_cards: &mut Vec<usize>) {
    for copy_idx in card_num+1..card_num+num_copies+1 {
        if copy_idx > scratch_cards.len() {
            scratch_cards.push(1);
        } else {
            scratch_cards[copy_idx-1] += scratch_cards[card_num-1];
        }
    }
}

fn get_num_matches(left: Vec<&str>, right: Vec<&str>) -> u32 {
    let mut set: HashSet<i32> = HashSet::new();
    let mut exp = 0;
    for s in left {
        let parsed_s = s.parse::<i32>();
        if let Ok(val) = parsed_s {
            set.insert(val);
        }
    }

    for s in right {
        let parsed_s = s.parse::<i32>();
        if parsed_s.is_ok() && set.contains(&parsed_s.unwrap()) {
            exp += 1;
        }
    }

    if exp == 0 { return 0; }

    exp
}

fn get_card_number(data: &str) -> usize {
    let re = Regex::new(r"Card\s+(\d+):").unwrap();
    re.captures(data)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<usize>()
        .expect("Expected an integer, got err")
}

fn get_idx_of_col_and_bar(data: &str) -> Result<(Option<usize>, Option<usize>), &'static str> {
    let bytes = data.as_bytes();
    let mut col_idx: Option<usize> = None;
    let mut bar_idx: Option<usize> = None;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b':' {
            col_idx = Some(i);
        } else if item == b'|' {
            bar_idx = Some(i);
        }
    }

    if col_idx.is_some() && bar_idx.is_some() {
        return Ok((col_idx, bar_idx));
    }

    Err("Could not find : or | in the data!")
}

fn extract_contents<T: Iterator<Item = String>>(mut args: T) -> Result<String, &'static str> {
    args.next();

    if let Some(path) = args.next() {
        fs::read_to_string(path).map_err(|_| "Failed to read file path. Is it there?")
    } else {
        Err("could not extract file path")
    }
}

fn run(data: &str) -> Result<(usize, usize), &'static str> {
    let mut part_one: usize = 0;

    let mut scratch_cards = vec![1; data.lines().count()];
    for line in data.lines() {
        if let Ok(line_idxs) = get_idx_of_col_and_bar(line) {
            let col_idx = line_idxs.0.unwrap();
            let bar_idx = line_idxs.1.unwrap();

            let base: i32 = 2;
            let num_matches = get_num_matches(line[col_idx+1..bar_idx].split(' ').collect(), line[bar_idx+1..].split(' ').collect());
            if num_matches > 0 {
                part_one += base.pow(num_matches - 1) as usize;
            }

            let card_num = get_card_number(line);
            update_scratchcards(card_num, num_matches as usize, &mut scratch_cards);
        } else {
            return Err("failure in extracting colon and bar from data line");
        }
    }

    let part_two = scratch_cards
        .iter()
        .sum();

    Ok((part_one, part_two))
}

pub fn main() {
    let data = extract_contents(env::args()).unwrap_or_else(|err| {
        eprintln!("Issue extracting data from file! {err}");
        exit(1);
    });

    if let Ok(res) = run(&data) {
        println!("Part 1 -- got result: {}", res.0);
        println!("Part 2 -- got result: {}", res.1);
    } else {
        exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scratch_pad_update() {
        let mut scratch_pad = vec![1, 1, 1, 1, 1];
        update_scratchcards(2, 5, &mut scratch_pad);

        assert_eq!(scratch_pad, vec![1, 1, 2, 2, 2, 1, 1]);
    }
}
