struct EnginePart<'a> {
    value: &'a str,
    pos_x: i32,
    pos_y: i32
}

fn get_neighbors(coord: (i32, i32), size: i32) -> Vec<(i32, i32)> {
    let mut res = Vec::new();

    for (dr, dc) in (-1..=1).flat_map(|dr| (-1..=size).map(move |dc| (dr, dc))) {
        if dr == 0 && (dc != -1 && dc != size) { continue; }
        res.push((coord.0 + dr, coord.1 + dc));
    }

    res
}

fn is_target_part(graph: Vec<Vec<char>>, engine_part: EnginePart) -> bool {
    let m = graph.len() as i32;
    let n = graph[0].len() as i32;

    get_neighbors((engine_part.pos_y, engine_part.pos_x), engine_part.value.len() as i32)
        .iter()
        .filter(|x| x.0 >= 0 && x.0 < m && x.1 >= 0 && x.1 < n)
        .map(|m| graph[m.0 as usize][m.1 as usize])
        .any(|z| z != '.' && !z.is_digit(10))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_part_boundary() {
        let graph = vec![
            vec!['4', '4', '0', '1', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '*', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];

        let engine_part = EnginePart {
            value: "4401",
            pos_x: 0,
            pos_y: 0
        };

        assert!(is_target_part(graph, engine_part));
    }

    #[test]
    fn test_target_part_true() {
        let graph = vec![
            vec!['.', '.', '.', '.', '.', '@', '.', '.', '.'],
            vec!['.', '.', '.', '4', '4', '0', '1', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];

        let engine_part = EnginePart {
            value: "4401",
            pos_x: 3,
            pos_y: 1
        };

        assert!(is_target_part(graph, engine_part));
    }

    #[test]
    fn test_target_part_false() {
        let graph = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '4', '4', '0', '1', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];

        let engine_part = EnginePart {
            value: "4401",
            pos_x: 3,
            pos_y: 1
        };

        assert!(!is_target_part(graph, engine_part));
    }

    #[test]
    fn test_neighbors() {
        let engine_part = EnginePart {
            value: "4401",
            pos_x: 4,
            pos_y: 2
        };

        let mut exp_vec = vec![(2,3), (1, 3), (1, 4), (1, 5), (1, 6), (1, 7), (1, 8), (2, 8), (3, 8), (3, 7), (3, 6), (3, 5), (3, 4), (3, 3)];
        assert_eq!(exp_vec.sort(), get_neighbors((engine_part.pos_y, engine_part.pos_x), engine_part.value.len() as i32).sort())
    }
}
