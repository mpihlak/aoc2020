use aoclib::*;
use std::collections::HashMap;

fn main() {
    let input_data = read_input_data();

    let mut mem = HashMap::new();
    let mut and_mask = !0u64;
    let mut or_mask  =  0u64;
    for line in input_data.split('\n') {
        if let Some(mask) = line.strip_prefix("mask = ") {
            and_mask = u64::from_str_radix(&mask.replace("X", "1"), 2).unwrap();
            or_mask  = u64::from_str_radix(&mask.replace("X", "0"), 2).unwrap();
        } else {
            let mut split_line = line.split(" = ");
            let mem_str = &split_line.next().unwrap()[4..].trim_end_matches(']');
            let val_str = &split_line.next().unwrap();
            let val: u64 = val_str.parse::<u64>().unwrap() & and_mask | or_mask;

            mem.insert(mem_str.to_string(), val);
        }
    }

    let sum: u64 = mem.values().sum();
    println!("Stage 1: Answer = {}", sum);

    let mut mem = HashMap::new();
    let mut and_mask = !0u64;
    let mut or_mask  =  0u64;
    let mut floating_bits = vec![0u32; 0];

    for line in input_data.split('\n') {
        if let Some(mask_str) = line.strip_prefix("mask = ") {
            and_mask = u64::from_str_radix(&mask_str.replace("0", "1").replace("X", "0"), 2).unwrap();
            or_mask  = u64::from_str_radix(&mask_str.replace("X", "0"), 2).unwrap();
            floating_bits = mask_str.chars()
                .enumerate()
                .filter(|x| x.1 == 'X')
                .map(|x| 35 - x.0 as u32)
                .collect();
        } else {
            let mut split_line = line.split(" = ");

            let addr: u64 = (&split_line.next().unwrap()[4..].trim_end_matches(']')).parse().unwrap();
            let val:  u64 = (&split_line.next().unwrap()).parse().unwrap();

            for i in 0..2_u32.pow(floating_bits.len() as u32) {
                let mut floating_bits_mask = 0_u64;
                for (bit_pos, j) in floating_bits.iter().rev().enumerate() {
                    let b = 1 << bit_pos;
                    if i & b == b {
                        floating_bits_mask |= 1 << j;
                    }
                }

                let masked_addr = addr & and_mask | or_mask | floating_bits_mask;
                mem.insert(masked_addr, val);
            }
        }
    }

    let sum: u64 = mem.values().sum();
    println!("Stage 2: Answer = {}", sum);
}
