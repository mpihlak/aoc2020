use aoclib::*;
use std::collections::HashMap;

fn main() {
    let input_data = read_input_data();
    let mut lines = input_data.split('\n');

    let mut mem = HashMap::new();
    let mut and_mask = !0u64;
    let mut or_mask  =  0u64;
    while let Some(line) = lines.next() {
        if line.starts_with("mask = ") {
            let mask = &line[7..];
            and_mask = u64::from_str_radix(&mask.replace("X", "1"), 2).unwrap();
            or_mask  = u64::from_str_radix(&mask.replace("X", "0"), 2).unwrap();
        } else {
            let mut split_line = line.split(" = ");
            let mem_str = &split_line.next().unwrap()[4..].trim_end_matches(']');
            let val_str = &split_line.next().unwrap();
            let val: u64 = val_str.parse::<u64>().unwrap() & and_mask | or_mask;

            let entry = mem.entry(mem_str.clone()).or_insert(0);
            *entry = val;
        }
    }

    let sum: u64 = mem.values().sum();
    println!("Stage 1: Answer = {}", sum);
}
