use aoclib::*;
use std::collections::HashMap;

fn main() {
    let answer_data = read_input_data();

    let mut groups = Vec::new();
    for group_data in answer_data.trim_end().split("\n\n") {
        let mut answer_count = HashMap::new();
        let mut member_count = 0;
        for group_answer in group_data.split("\n") {
            for c in group_answer.chars() {
                let entry = answer_count.entry(c).or_insert(0);
                *entry += 1;
            }
            member_count += 1;
        }
        groups.push((member_count, answer_count));
    }

    let answer_count_sum = groups
        .iter()
        .fold(0, |acc, x| acc + x.1.keys().len());
    println!("Stage 1: Sum of answers = {}", answer_count_sum);

    let mut answer_count_sum = 0;
    for (member_count, answer_count) in groups {
        answer_count_sum += answer_count.iter()
            .fold(0, |acc, (_k, v)| acc + if *v == member_count { 1 } else { 0 });
    }
    println!("Stage 2: Sum of answers = {}", answer_count_sum);
}
