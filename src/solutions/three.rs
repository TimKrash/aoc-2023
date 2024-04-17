use std::env;
use std::fs;
use regex::Regex;
use std::process;

#[derive(Debug, Eq)]
struct EnginePart<'a> {
    value: &'a str,
    pos_x: i32,
    pos_y: i32
}

impl PartialEq for EnginePart<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value &&
        self.pos_x == other.pos_x &&
        self.pos_y == other.pos_y
    }
}

fn build_engine_parts(data: &str) -> Result<Vec<EnginePart>, &'static str> {
    if data.len() == 0 {
        return Err("No data found while building engine parts!");
    }
    let mut res: Vec<EnginePart> = Vec::new();
    let re = Regex::new(r"\d+").map_err(|_| "Issue with regex pattern").unwrap();

    for (line_idx, line) in data.lines().enumerate() {
        for mat in re.find_iter(line) {
            res.push(EnginePart {
                value: mat.as_str(),
                pos_y: line_idx as i32,
                pos_x: mat.start() as i32
            })
        }
    }

    Ok(res)
}

struct Graph {
    elements: Vec<Vec<char>>,
    size: (usize, usize)
}

impl Graph {
    fn build(data: &str) -> Result<Self, &'static str> {
        if data.len() == 0 {
            return Err("No data found in input list!");
        }

        let elements: Vec<_> = data
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect();

        let size = (elements.len(), elements[0].len());

        Ok( Graph { elements, size } )
    }

    fn get(&self, coord: (i32, i32)) -> Option<char> {
        let convert_coord = (coord.0 as usize, coord.1 as usize);

        if 
            convert_coord.0 >= self.size.0 ||
            convert_coord.1 >= self.size.1
         {
            return None;
        }

        Some(self.elements[convert_coord.0][convert_coord.1])
    }

    fn neighbors(&self, coord: (i32, i32), size: i32) -> Vec<Option<char>> {
        let mut res = Vec::new();

        for (dr, dc) in (-1..=1).flat_map(|dr| (-1..=size).map(move |dc| (dr, dc))) {
            if dr == 0 && (dc != -1 && dc != size) { continue; }
            res.push(self.get((coord.0 + dr, coord.1 + dc)));
        }

        res
    }
}

fn is_target_part(graph: &Graph, engine_part: &EnginePart) -> bool {
    graph.neighbors((engine_part.pos_y, engine_part.pos_x), engine_part.value.len() as i32)
        .iter()
        .filter_map(|&x| x)
        .any(|z| z != '.' && !z.is_digit(10))
}

fn extract_contents<T: Iterator<Item = String>>(mut args: T) -> Result<String, &'static str> {
    args.next();
    
    let file_path = match args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get a file path for the data!")
    };

    fs::read_to_string(file_path).map_err(|_| "Failed to read file! Does it exist?")
}

pub fn main() {
    let data = extract_contents(env::args()).unwrap_or_else(|err| {
        eprintln!("Failed to extract data from input file: {err}");
        process::exit(1);
    });

    let graph = Graph::build(&data).unwrap_or_else(|err| {
        eprintln!("Failed to generate graph: {err}");
        process::exit(1);
    });

    let engine_parts = build_engine_parts(&data).unwrap_or_else(|err| {
        eprintln!("Failed to extract engine parts: {err}");
        process::exit(1);
    });

    let mut res = 0;
    for part in engine_parts {
        if is_target_part(&graph, &part) {
            res += part.value.parse::<i32>().unwrap();
        }
    }

    println!("Part 1 result: {res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_part_boundary() {
        let elements = vec![
            vec!['4', '4', '0', '1', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '*', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        let size = (elements.len(), elements[0].len());

        let graph = Graph { elements, size };

        let engine_part = EnginePart {
            value: "4401",
            pos_x: 0,
            pos_y: 0
        };

        assert!(is_target_part(&graph, &engine_part));
    }

    #[test]
    fn test_target_part_true() {
        let elements = vec![
            vec!['.', '.', '.', '.', '.', '@', '.', '.', '.'],
            vec!['.', '.', '.', '4', '4', '0', '1', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        let size = (elements.len(), elements[0].len());

        let graph = Graph { elements, size };

        let engine_part = EnginePart {
            value: "4401",
            pos_x: 3,
            pos_y: 1
        };

        assert!(is_target_part(&graph, &engine_part));
    }

    #[test]
    fn test_target_part_false() {
        let elements = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '4', '4', '0', '1', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        let size = (elements.len(), elements[0].len());

        let graph = Graph { elements, size };

        let engine_part = EnginePart {
            value: "4401",
            pos_x: 3,
            pos_y: 1
        };

        assert!(!is_target_part(&graph, &engine_part));
    }

    #[test]
    fn test_neighbors() {
        let elements = vec![
            vec!['.', '.', '.', '.', '.', '.', '@', '.', '.'],
            vec!['.', '.', '.', '4', '4', '0', '1', '.', '.'],
            vec!['.', '.', '.', '.', '.', '*', '.', '.', '.'],
        ];
        let size = (elements.len(), elements[0].len());

        let graph = Graph { elements, size };
        let engine_part = EnginePart {
            value: "4401",
            pos_x: 4,
            pos_y: 2
        };

        let mut exp_vec = vec!['.'; 12];
        exp_vec.push('@');
        exp_vec.push('*');
        assert_eq!(exp_vec.sort(), graph.neighbors((engine_part.pos_y, engine_part.pos_x), engine_part.value.len() as i32).sort())
    }

    #[test]
    fn test_engine_part_build() {
        let data = "...4401...12..\n351..$..#...3.";

        let expect_res: Vec<EnginePart> = vec![
            EnginePart {
                value: "4401",
                pos_x: 3,
                pos_y: 0
            },
            EnginePart {
                value: "12",
                pos_x: 10,
                pos_y: 0
            },
            EnginePart {
                value: "351",
                pos_x: 0,
                pos_y: 1
            },
            EnginePart {
                value: "3",
                pos_x: 12,
                pos_y: 1
            }
        ];

        let res = build_engine_parts(&data).unwrap();
        assert_eq!(expect_res, res);
    }
}
