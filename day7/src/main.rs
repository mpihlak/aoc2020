use std::collections::{HashMap, HashSet};

use aoclib::*;

type BagOfBags = HashMap<String, Vec<String>>;

fn collect_outer_bag_colors(bag_color: &str, bags: &BagOfBags, mut result: &mut HashSet<String>) {
    if let Some(container_bags) = bags.get(bag_color) {
        for container in container_bags.iter() {
            result.insert(container.to_string());
            collect_outer_bag_colors(container, &bags, &mut result);
        }
    }
}

fn count_contained_bags(bag_color: &str, bags: &BagOfBags) -> i32 {
    if let Some(contained_bags) = bags.get(bag_color) {
        let mut res = 0;
        for bag in contained_bags {
            res += 1 + count_contained_bags(bag, &bags);
        }
        res
    } else {
        0
    }
}

fn main() {
    let input_data = read_input_data();

    let mut bag_containers = HashMap::new();

    for line in input_data.split("\n") {
        let mut split_at_contain = line.split(" contain ");
        let container = split_at_contain.next().unwrap().replace(" bags", "");
        let content_data  = split_at_contain.next().unwrap();
        let content = content_data
            .trim_end_matches('.')
            .split(", ")
            .map(|x| x.replace(" bags", ""))
            .map(|x| x.replace(" bag", ""))
            .collect::<Vec<String>>();

        for bag in content.iter() {
            let mut it = bag.splitn(2, " ");
            let _count = it.next().unwrap();
            let color = it.next().unwrap().to_string();

            let entry = bag_containers.entry(color).or_insert(Vec::new());
            entry.push(container.clone());
        }
    }

    let mut outer_colors = HashSet::new();
    collect_outer_bag_colors("shiny gold", &bag_containers, &mut outer_colors);
    println!("Stage 1: Outer bag colors = {:?}", outer_colors.len());

    let mut bag_contents = HashMap::new();

    for line in input_data.split("\n") {
        let mut split_at_contain = line.split(" contain ");
        let container = split_at_contain.next().unwrap().replace(" bags", "");
        let content_data  = split_at_contain.next().unwrap();
        let content = content_data
            .trim_end_matches('.')
            .split(", ")
            .map(|x| x.replace(" bags", ""))
            .map(|x| x.replace(" bag", ""))
            .collect::<Vec<String>>();

        let mut contained_bags = Vec::new();
        for bag in content.iter() {
            let mut it = bag.splitn(2, " ");
            let count = it.next().unwrap();
            let color = it.next().unwrap().to_string();
            let count = if count == "no" { 0 } else { count.parse::<usize>().unwrap() };

            for _ in 0..count {
                contained_bags.push(color.clone());
            }
        }
        println!("{} contains {:?}", container, contained_bags);
        bag_contents.insert(container.clone(), contained_bags);
    }

    let contained_bags = count_contained_bags("shiny gold", &bag_contents);
    println!("Stage 2: Inner bags count = {:?}", contained_bags);
}
