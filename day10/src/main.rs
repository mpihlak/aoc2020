use aoclib::*;
use std::collections::HashMap;

fn count_arrangements(
    adapters: &[i32],
    voltage: i32,
    device_voltage: i32,
    mut cache: &mut HashMap<i32,i64>
) -> i64 {
    if voltage > device_voltage {
        return 0;
    }
    if voltage == device_voltage {
        return 1;
    }

    if let Some(arrangements) = cache.get(&adapters[0]) {
        return *arrangements;
    }

    let mut arrangements = 0;

    for i in 0..adapters.len() {
        let diff = adapters[i] - voltage;

        if diff >= 1 && diff <= 3 {
            arrangements += count_arrangements(&adapters[(i+1)..], adapters[i], device_voltage, &mut cache);
        } else {
            break;
        }
    }

    cache.insert(adapters[0], arrangements);
    return arrangements;
}

fn main() {
    let input_data = read_input_data();
    let mut adapters: Vec<i32> = input_data.split('\n').map(|x| x.parse().unwrap()).collect();
    adapters.sort();
    println!("Input = {:?}", adapters);

    let device_voltage = *adapters.last().unwrap();

    let mut diffs = HashMap::new();
    let mut voltage = 0;
    for adapter in adapters.iter() {
        let diff = adapter - voltage;

        if diff >= 1 && diff <= 3 {
            let entry = diffs.entry(diff).or_insert(0);
            *entry += 1;
            voltage = *adapter;
        }
    }

    let entry = diffs.entry(3).or_insert(0);
    *entry += 1;
    assert_eq!(device_voltage, voltage);

    println!("Final voltage = {}", voltage);
    println!("Distributions = {:?}", diffs);
    let answer = diffs.get(&1).unwrap() * diffs.get(&3).unwrap();
    println!("Stage 1: answer = {}", answer);

    let mut cache = HashMap::new();
    let answer = count_arrangements(&adapters[..], 0, device_voltage, &mut cache);
    println!("Stage 2: answer = {}", answer);
}
