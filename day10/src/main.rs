use aoclib::*;
use std::collections::HashMap;

fn main() {
    let input_data = read_input_data();
    let mut adapters: Vec<i32> = input_data.split('\n').map(|x| x.parse().unwrap()).collect();
    adapters.sort();
    println!("input = {:?}", adapters);

    let mut diffs = HashMap::new();
    let mut voltage = 0;
    for adapter in adapters.iter() {
        let diff = adapter - voltage;

        if diff >= 1 && diff <= 3 {
            let entry = diffs.entry(diff).or_insert(0);
            *entry += 1;
            println!("voltage = {}, diff = {}, adapter = {}", voltage, diff, adapter);
            voltage = *adapter;
        }
    }

    let entry = diffs.entry(3).or_insert(0);
    *entry += 1;
    voltage += 3;

    println!("Final voltage = {}", voltage);
    println!("Distributions = {:?}", diffs);
    let answer = diffs.get(&1).unwrap() * diffs.get(&3).unwrap();
    println!("Stage 1: answer = {}", answer);
}
