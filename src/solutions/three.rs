use std::collections::HashMap;
use std::env;
use std::fs;
use regex::Regex;
use std::process;

#[derive(Debug, Eq)]
struct EnginePart<'a> {
    value: &'a str,
    pos_x: i32,
    pos_y: i32,
    has_symbol: bool
}

impl PartialEq for EnginePart<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value &&
        self.pos_x == other.pos_x &&
        self.pos_y == other.pos_y
    }
}

fn build_gear_map(coord: (i32, i32), val: i32, gear_map: &mut HashMap<(i32, i32), [Option<i32>; 2]>) {
    if !gear_map.contains_key(&coord) {
        gear_map.insert(coord, [Some(val), None]);
        return;
    }

    if gear_map
        .get(&coord)
        .unwrap()
        .iter()
        .all(|&option| option.is_none()) {
        return;
    }

    if gear_map
        .get(&coord)
        .unwrap()
        .iter()
        .all(|&option| option.is_some()) {
        if let Some(arr) = gear_map.get_mut(&coord) {
            arr[0] = None;
            arr[1] = None;
        }
        return;
    }

    if let Some(arr) = gear_map.get_mut(&coord) {
        arr[1] = Some(val);
    }
}

fn build_engine_parts<'a>(data: &'a str, graph: &Graph, gear_map: &mut HashMap<(i32, i32), [Option<i32>; 2]>) -> Result<Vec<EnginePart<'a>>, &'static str> {
    if data.len() == 0 {
        return Err("No data found while building engine parts!");
    }
    let mut res: Vec<EnginePart> = Vec::new();
    let re = Regex::new(r"\d+").map_err(|_| "Issue with regex pattern").unwrap();

    for (line_idx, line) in data.lines().enumerate() {
        for mat in re.find_iter(line) {
            let pos_x = mat.start() as i32;
            let pos_y = line_idx as i32;
            let has_symbol = graph.neighbors((pos_y, pos_x), mat.as_str().len() as i32)
                .iter()
                .filter_map(|&coord| graph.get(coord))
                .any(|z| z != '.' && !z.is_digit(10));

            let mut asterisk_coord: Option<(i32, i32)> = None;
            if graph.neighbors((pos_y, pos_x), mat.as_str().len() as i32)
                .iter()
                .filter_map(|&coord| {
                    asterisk_coord = Some(coord);
                    graph.get(coord)
                })
                .any(|z| z == '*') {
                build_gear_map(asterisk_coord.unwrap(), mat.as_str().parse::<i32>().unwrap(), gear_map)
            }

            res.push(EnginePart {
                value: mat.as_str(),
                pos_y: line_idx as i32,
                pos_x: mat.start() as i32,
                has_symbol
            });
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
        if coord.0 < 0 || coord.1 < 0 { return None; }
        let convert_coord = (coord.0 as usize, coord.1 as usize);

        if 
            convert_coord.0 >= self.size.0 ||
            convert_coord.1 >= self.size.1
         {
            return None;
        }

        Some(self.elements[convert_coord.0][convert_coord.1])
    }

    fn neighbors(&self, coord: (i32, i32), size: i32) -> Vec<(i32, i32)> {
        let mut res = Vec::new();

        for (dr, dc) in (-1..=1).flat_map(|dr| (-1..=size).map(move |dc| (dr, dc))) {
            if dr == 0 && (dc != -1 && dc != size) { continue; }
            res.push((coord.0 + dr, coord.1 + dc));
        }

        res
    }
}

fn is_target_part(engine_part: &EnginePart) -> bool {
    engine_part.has_symbol
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

    let mut gear_map: HashMap<(i32, i32), [Option<i32>; 2]> = HashMap::new();

    let engine_parts = build_engine_parts(&data, &graph, &mut gear_map).unwrap_or_else(|err| {
        eprintln!("Failed to extract engine parts: {err}");
        process::exit(1);
    });

    let mut res = 0;
    for part in engine_parts {
        if is_target_part(&part) {
            res += part.value.parse::<i32>().unwrap();
        }
    }

    println!("Part 1 result: {res}");

    let mut res_part_two = 0;
    for (_, v) in &gear_map {
        if v
            .iter()
            .all(|&option| option.is_some()) {
            res_part_two += v[0].unwrap() * v[1].unwrap();
        }
    }

    println!("Part 2 result: {res_part_two}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_part_boundary() {
        let engine_part = EnginePart {
            value: "4401",
            pos_x: 0,
            pos_y: 0,
            has_symbol: true
        };

        assert!(is_target_part(&engine_part));
    }

    #[test]
    fn test_target_part_true() {
        let engine_part = EnginePart {
            value: "4401",
            pos_x: 3,
            pos_y: 1,
            has_symbol: true
        };

        assert!(is_target_part(&engine_part));
    }

    #[test]
    fn test_target_part_false() {
        let engine_part = EnginePart {
            value: "4401",
            pos_x: 3,
            pos_y: 1,
            has_symbol: false
        };

        assert!(!is_target_part(&engine_part));
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
            pos_x: 3,
            pos_y: 1,
            has_symbol: true
        };

        let mut exp_vec = vec![(0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7), (1, 7), (2, 7), (2, 6), (2, 5), (2, 4), (2, 3), (2, 2), (1, 2)];
        let mut res = graph.neighbors((engine_part.pos_y, engine_part.pos_x), engine_part.value.len() as i32);
        exp_vec.sort();
        res.sort();
        assert_eq!(exp_vec, res);
    }

    #[test]
    fn test_engine_part_and_gear_build() {
        let data = "...331..3\n...&401..\n.2%..*...";

        let elements = vec![
            vec!['.', '.', '.', '3', '3', '1', '.', '.', '3'],
            vec!['.', '.', '.', '.', '4', '0', '1', '.', '.'],
            vec!['.', '2', '%', '.', '.', '*', '.', '.', '.'],
        ];
        let size = (elements.len(), elements[0].len());

        let graph = Graph { elements, size };

        let mut gear_map = HashMap::new();
        let expect_res: Vec<EnginePart> = vec![
            EnginePart {
                value: "331",
                pos_x: 3,
                pos_y: 0,
                has_symbol: false
            },
            EnginePart {
                value: "3",
                pos_x: 8,
                pos_y: 0,
                has_symbol: false
            },
            EnginePart {
                value: "401",
                pos_x: 4,
                pos_y: 1,
                has_symbol: true
            },
            EnginePart {
                value: "2",
                pos_x: 1,
                pos_y: 2,
                has_symbol: true
            },
        ];

        let res = build_engine_parts(&data, &graph, &mut gear_map).unwrap();
        assert_eq!(expect_res, res);

        let exp_gear_map: HashMap<(i32, i32), [Option<i32>; 2]> = HashMap::from([
            ((2, 5), [Some(401), None])
        ]);
        assert_eq!(exp_gear_map, gear_map);
    }
}
