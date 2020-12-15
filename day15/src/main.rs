use aoclib::*;
use std::collections::HashMap;

fn count_to(starting_numbers: &Vec<i32>, count: usize) -> i32 {
    let mut occurrences: HashMap<i32, usize> = HashMap::new();
    for (pos, num) in starting_numbers.iter().take(starting_numbers.len()-1).enumerate() {
        occurrences.insert(*num, pos);
    }

    let mut last = *(starting_numbers.last().unwrap());
    for count in starting_numbers.len()-1 .. count-1 {
        let output = if let Some(pos) = occurrences.get(&last) {
            count - pos
        } else {
            0
        };

        occurrences.insert(last, count);
        last = output as i32;
    }

    last
}

fn main() {
    let input_data = read_input_data();
    let starting_numbers: Vec<i32> = input_data
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    println!("Starting numbers = {:?}", starting_numbers);

    assert_eq!(1, count_to(&vec![1,3,2], 2020));
    assert_eq!(10, count_to(&vec![2,1,3], 2020));

    let answer = count_to(&starting_numbers, 2020);
    println!("Stage 1: answer = {}", answer);

    let answer = count_to(&starting_numbers, 30000000);
    println!("Stage 2: answer = {}", answer);
}
