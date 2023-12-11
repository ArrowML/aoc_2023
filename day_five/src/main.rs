
use std::fs::File;
use std::io::{BufReader, BufRead};

use std::time::Instant;

#[derive(Debug, Clone)]
struct Map {
    map_type: MapType,
    ranges: Vec<IntMap>
}
#[derive(Debug, Clone)]
struct IntMap {
    source: i64,
    destination: i64,
    range: i64
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
    // part1();
    part2();
}

fn part1() {
    let data = parse_data();
    let mut results: Vec<i64> = Vec::new();
    for s in data.0 {
        results.push(get_map_path(s, &data.1))
    }
    let total = results.iter().min().unwrap();
    println!("{}", total);
}

fn part2() {

    let data = parse_data();
    let now = Instant::now();

    let mut seeds: Vec<(i64, i64)> = Vec::new();
    for (i, num) in data.0.iter().enumerate() {
        if i%2 == 0 {
            seeds.push((*num, num + data.0[i + 1] - 1));
        }
    }

    for block in data.1 {
        let mut new: Vec<(i64,i64)> = Vec::new();
        while seeds.len() > 0 {
            let (s, e) = seeds.pop().unwrap();
            let mut found = false;
            for m in &block.ranges {
                let os = s.max(m.source);
                let oe = e.min(m.source + m.range);

                if os < oe {
                    found = true;
                    new.push((os - m.source + m.destination, oe - m.source + m.destination));
                    if os > s {
                        seeds.push((s, os))
                    }
                    if e > oe {
                        seeds.push((oe, e))
                    }
                    break; 
                }
            }
            if !found {
                new.push((s, e));
            }
        }
        seeds = new;
    }

    println!("{:?}", seeds.iter().min().unwrap());
    //println!("{:?}", seeds);

} 

fn parse_data() -> (Vec<i64>, Vec<Map>) {
    let file: File = File::open("./src/input.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(file);

    let mut seeds: Vec<i64> = Vec::new();
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
                seeds = parts[1].trim().split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect();
                continue;
            } else {
                map_type = Some(MapType::from_string(parts[0]));
                section = true;
                continue;
            }  
        }

        if ch.is_digit(10) {
            let range_parts: Vec<i64> = line.as_ref().unwrap().trim().split_whitespace().map(|n: &str| n.parse::<i64>().unwrap()).collect();
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


fn get_map_path(id: i64, maps: &Vec<Map>) -> i64 {
    let mut res: i64 = id;
    for m in maps {
        res = get_mapping(res, m);
    }
    return res;
}

fn get_mapping(id: i64, map: &Map) -> i64 {
    for m in &map.ranges {
        let min = m.source;
        let max = (m.source + m.range) - 1;
        if id >= min && id <= max {
            return id - m.source + m.destination;
        }
    }
    return id;
}

