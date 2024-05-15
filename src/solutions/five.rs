use std::collections::HashMap;

const SEED_TO_SOIL: &str = "seed-to-soil";
const SOIL_TO_FERT: &str = "soil-to-fertilizer";
const FERT_TO_WATER: &str = "fertilizer-to-water";
const WATER_TO_LIGHT: &str = "water-to-light";
const LIGHT_TO_TEMP: &str = "light-to-temperature";
const TEMP_TO_HUMID: &str = "temperature-to-humidity";
const HUMID_TO_LOC: &str = "humidity-to-location";

const MAP_ARR: [&str; 7] = [SEED_TO_SOIL, SOIL_TO_FERT, FERT_TO_WATER, WATER_TO_LIGHT, LIGHT_TO_TEMP, TEMP_TO_HUMID, HUMID_TO_LOC];

struct SeedRange {
    dest: u32,
    src: u32,
    range: u32
}

impl SeedRange {
    fn build(dest: u32, src: u32, range: u32) -> Self {
        SeedRange {
            dest,
            src,
            range
        }
    }
}

fn dest(src: u32, map_idx: usize, maps: HashMap<&str, Vec<SeedRange>>) -> u32 {
    if map_idx >= MAP_ARR.len() {
        return src;
    }

    let map_key: &str = MAP_ARR[map_idx];
    let map_range: &Vec<SeedRange> = maps.get(map_key).unwrap();

    let mut dest_val = src;
    for range in map_range {
        if src >= range.src && src < range.src + range.dest {
            dest_val = range.dest + (src - range.src);
            break;
        }
    }

    dest(dest_val, map_idx + 1, maps)
}

fn build_maps(data: &str) -> HashMap<&str, Vec<SeedRange>> {
    let mut maps: HashMap<&str, Vec<SeedRange>> = HashMap::new();
    let mut map_key: Option<&str> = None;
    for line in data.lines() {
        let split_line: Vec<_> = line.split(' ').collect();
        if split_line.is_empty() {
            continue;
        }

        if MAP_ARR.contains(&split_line[0]) {
            map_key = Some(split_line[0]);
            continue;
        }

        let dest: u32 = str::parse::<u32>(split_line[0]).unwrap();
        let src: u32 = str::parse::<u32>(split_line[1]).unwrap();
        let range: u32 = str::parse::<u32>(split_line[2]).unwrap();

        let seed_range = SeedRange { dest, src, range };

        maps.entry(map_key.unwrap()).or_default().push(seed_range);
    }

    maps
} 

pub fn main() {
    extract seeds and build maps
}
