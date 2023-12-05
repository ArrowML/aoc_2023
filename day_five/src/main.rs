
use std::fs::File;
use std::io::{BufReader, BufRead};

use std::time::Instant;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Map {
    map_type: MapType,
    ranges: Vec<IntMap>
}
#[derive(Debug, Clone)]
struct IntMap {
    source: u64,
    destination: u64,
    range: u64
}

#[derive(Debug, Clone)]
enum MapType {
    Unknown,
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

impl MapType {
    fn from_string(s: &str) -> MapType {
        match s {
            "seed-to-soil map" => MapType::SeedToSoil,
            "soil-to-fertilizer map" => MapType::SoilToFertilizer,
            "fertilizer-to-water map" => MapType::FertilizerToWater,
            "water-to-light map" => MapType::WaterToLight,
            "light-to-temperature map" => MapType::LightToTemperature,
            "temperature-to-humidity map" => MapType::TemperatureToHumidity,
            "humidity-to-location map" => MapType::HumidityToLocation,
            &_ => MapType::Unknown
        }
    }
}

fn main() {
    //part1();
    part2();
}

fn part1() {
    let data = parse_data();
    let mut results: Vec<u64> = Vec::new();
    for s in data.0 {
        results.push(get_map_path(s, &data.1))
    }
    let total = results.iter().min().unwrap();
    println!("{}", total);
}

fn part2() {

    let data = parse_data();
    let mut results: HashMap<u64, u64> = HashMap::new();
    let now = Instant::now();

    let mut ranges: Vec<(u64, u64)> = Vec::new();
    for (i, num) in data.0.iter().enumerate() {
        if i%2 == 0 {
            ranges.push((*num, num + data.0[i + 1]));
        }
    }

    ranges.sort();

    println!("{:?}", ranges);

    let mut overlaps: Vec<u64> = Vec::new();
    for i in 1..ranges.len() - 1 {
        let min = ranges[i - 1].0;
        let max = ranges[i - 1].1;

        if max > ranges[i].0 {

        }

    }
    /*

    1000 - 2000  1300 - 2100  3000 - 4000

    for (i, num) in data.0.iter().enumerate() {
        if i%2 == 0 {
            let range = data.0[i+1];
            for n in 0..range {
                let it: u64 = num + n;
                if results.get(&it).is_none() {
                    let res = get_map_path(it, &data.1);
                    results.insert(it, res);
                }
                if n%100000 == 0 {
                    let elapsed = now.elapsed();
                    println!("Elapsed: {:.2?}", elapsed);   
                }
            }
        }
    }
    */

    let total: u64 = *results.values().min().unwrap();
    println!("{}", total);

} 

fn parse_data() -> (Vec<u64>, Vec<Map>) {
    let file: File = File::open("./src/input.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(file);

    let mut seeds: Vec<u64> = Vec::new();
    let mut int_maps: Vec<Map> = Vec::new();

    let mut section = false;
    let mut map_type: Option<MapType> = Some(MapType::Unknown);
    let mut ranges: Vec<IntMap> = Vec::new();

    for line in reader.lines() {

        let ch = line.as_ref().unwrap().clone().chars().nth(0).unwrap_or(' ');

        if !section {
            ranges.clear();
            map_type = Some(MapType::Unknown)
        }

        if ch.is_alphabetic() {
            let parts: Vec<&str> = line.as_ref().unwrap().split(":").collect();

            if parts[0] == "seeds" {
                seeds = parts[1].trim().split_whitespace().map(|n| n.parse::<u64>().unwrap()).collect();
                continue;
            } else {
                map_type = Some(MapType::from_string(parts[0]));
                section = true;
                continue;
            }  
        }

        if ch.is_digit(10) {
            let range_parts: Vec<u64> = line.as_ref().unwrap().trim().split_whitespace().map(|n: &str| n.parse::<u64>().unwrap()).collect();
            ranges.push(IntMap { source: range_parts[1], destination: range_parts[0], range: range_parts[2] });
            continue;
        }   

        if ch == ' ' {
            if ranges.len() > 0 {
                int_maps.push(Map { map_type: Some(map_type.clone()).unwrap_or(Some(MapType::Unknown)).unwrap(), ranges: ranges.clone()});
            }
            section = false;
            continue;
        }
    }
    return (seeds, int_maps);
}


fn get_map_path(id: u64, maps: &Vec<Map>) -> u64 {
    let mut res: u64 = id;
    for m in maps {
        res = get_mapping(res, m);
    }
    return res;
}

fn get_mapping(id: u64, map: &Map) -> u64 {
    for m in &map.ranges {
        let min = m.source;
        let max = (m.source + m.range) - 1;
        if id >= min && id <= max {
            let diff = m.source.abs_diff(m.destination);
            if m.source > m.destination {
                return id - diff;
            }
            if m.source < m.destination {
                return id + diff;
            }
        }
    }
    return id;
}
