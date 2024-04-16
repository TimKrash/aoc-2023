use std::fs;
use std::process;
use std::env;
use regex::Regex;

enum Color {
    Red,
    Blue,
    Green
}

#[derive(Debug)]
struct CubeSet {
    red: i32,
    blue: i32,
    green: i32
}

impl CubeSet {
    fn build(input: &str) -> Self {
        let red = extract_max_for_color(Color::Red, &input).unwrap();
        let blue = extract_max_for_color(Color::Blue, &input).unwrap();
        let green = extract_max_for_color(Color::Green, &input).unwrap();

        CubeSet {
            red,
            blue,
            green
        }
    }

    fn larger_than(&self, other: &CubeSet) -> bool {
        self.red >= other.red &&
        self.green >= other.green &&
        self.blue >= other.blue
    }
}

fn extract_max_for_color(color: Color, input: &str) -> Result<i32, &'static str> {
    let color_str = match color {
        Color::Red => "red",
        Color::Blue => "blue",
        Color::Green => "green"
    };

    let max_val: i32 = Regex::new(format!(r"(\d+)\s{}", color_str).as_str())
        .unwrap()
        .find_iter(input)
        .map(|m| m.as_str()) 
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.split_whitespace().next().unwrap().parse().unwrap())
        .max()
        .expect("Failed to extract max digit for color!");

    Ok(max_val)
}

fn calculate_val<T>(mut args: T) -> Result<(usize, usize), &'static str>
where
    T: Iterator<Item = String>
{
    args.next();

    // Get file path
    let file_path: String = match args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get a file path for data!")
    };

    let content = fs::read_to_string(file_path).map_err(|_| "Failed to read file! Does it exist?")?;

    let global_cap = CubeSet { red: 12, green: 13, blue: 14 };
    let mut part_one_sum = 0;
    let mut part_two_sum = 0;
    for (idx, line) in content.lines().enumerate() {
        let curr_set = CubeSet::build(&line);
        part_two_sum += curr_set.red * curr_set.blue * curr_set.green;
        if global_cap.larger_than(&curr_set) {
            part_one_sum += idx + 1;
        }
    }

    Ok((part_one_sum, part_two_sum as usize))
}

pub fn main() {
    let (part_one_res, part_two_res) = calculate_val(env::args()).unwrap_or_else(|err| {
        eprintln!("Failure in parsing arguments: {err}");
        process::exit(1);
    });

    println!("Part 1 result: {part_one_res}");
    println!("Part 2 result: {part_two_res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_cubeset() {
        let input = "Game 5: 9 green, 1 red; 18 green, 2 red, 7 blue; 1 blue, 9 green, 3 red; 3 red, 15 blue, 18 green";
        let cs = CubeSet::build(&input);

        assert_eq!(3, cs.red);
        assert_eq!(18, cs.green);
        assert_eq!(15, cs.blue);
    }

    #[test]
    fn larger_than_cap() {
        let cs_cap = CubeSet {
            red: 12,
            blue: 13,
            green: 14
        };

        let cs_test = CubeSet {
            red: 1,
            blue: 9,
            green: 17
        };

        assert!(!cs_cap.larger_than(&cs_test))
    }

    #[test]
    fn smaller_than_cap() {
        let cs_cap = CubeSet {
            red: 12,
            blue: 13,
            green: 14
        };

        let cs_test = CubeSet {
            red: 10,
            blue: 9,
            green: 11
        };

        assert!(cs_cap.larger_than(&cs_test))
    }

    #[test]
    fn equal_to_cap() {
        let cs_cap = CubeSet {
            red: 12,
            blue: 13,
            green: 14
        };

        let cs_test = CubeSet {
            red: 12,
            blue: 9,
            green: 14
        };

        assert!(cs_cap.larger_than(&cs_test))
    }
}
