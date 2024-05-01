use std::{collections::HashSet, env, fs, process::exit};

fn get_result(left: Vec<&str>, right: Vec<&str>) -> i32 {
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

    let base: i32 = 2;
    base.pow(exp - 1)
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

fn run(data: &str) -> Result<i32, &'static str> {
    let mut sum = 0;
    for line in data.lines() {
        if let Ok(line_idxs) = get_idx_of_col_and_bar(line) {
            let col_idx = line_idxs.0.unwrap();
            let bar_idx = line_idxs.1.unwrap();

            sum += get_result(line[col_idx+1..bar_idx].split(' ').collect(), line[bar_idx+1..].split(' ').collect());
        } else {
            return Err("failure in extracting colon and bar from data line");
        }
    }

    Ok(sum)
}

pub fn main() {
    let data = extract_contents(env::args()).unwrap_or_else(|err| {
        eprintln!("Issue extracting data from file! {err}");
        exit(1);
    });

    if let Ok(res) = run(&data) {
        println!("Got result: {res}");
    } else {
        exit(1);
    }
}
