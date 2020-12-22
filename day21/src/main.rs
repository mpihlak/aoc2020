use aoclib::*;
use std::collections::{HashMap};

#[derive(Debug)]
struct Food<'a> {
    ingredients: Vec<&'a str>,
    allergens: Vec<&'a str>,
}

fn map_allergens_to_ingredients(
    allergen_ingredients: &mut HashMap<&str, Vec<&str>>
) -> HashMap<String,String> {
    let mut known_allergens = HashMap::new();

    while known_allergens.len() < allergen_ingredients.keys().len() {
        for (allergen, ingredients) in allergen_ingredients.iter() {
            let mut reduced_ingredients = Vec::new();

            for ingredient in ingredients {
                if !known_allergens.contains_key(*ingredient) {
                    reduced_ingredients.push(ingredient.clone());
                }
            }

            if reduced_ingredients.len() == 1 {
                known_allergens.insert(reduced_ingredients[0].to_string(), allergen.to_string());
            }
        }
    }
    
    known_allergens
}

fn solve(foods: &[Food]) {
    let mut allergen_ingredients: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut ingredient_counts = HashMap::new();

    for food in foods.iter() {
        for allergen in food.allergens.iter() {
            if let Some(entry) = allergen_ingredients.get(allergen.clone()) {
                // Take a union to of the ingredients
                let v: Vec<&str> = food.ingredients.iter()
                    .filter(|x| entry.contains(x))
                    .map(|x| *x)
                    .collect();
                allergen_ingredients.insert(allergen, v);
            } else {
                allergen_ingredients.insert(
                    allergen,
                    food.ingredients.iter().map(|x| *x).collect());
            }
        }

        for ingredient in food.ingredients.iter() {
            let entry = ingredient_counts.entry(ingredient.clone()).or_insert(0);
            *entry += 1;
        }
    }

    let ingredient_map = map_allergens_to_ingredients(&mut allergen_ingredients);

    let sum: u32 = ingredient_counts.iter()
        .filter(|(ingredient, _)| !ingredient_map.contains_key(&ingredient.to_string()))
        .map(|(_, count)| *count)
        .sum();
    println!("Sum of safe ingredients = {}", sum);

    let mut dangerous_ingredients: Vec<(String, String)> = ingredient_map.iter()
        .map(|x| (x.0.to_string(), x.1.to_string()))
        .collect();
    dangerous_ingredients.sort_by(|a,b| a.1.cmp(&b.1.to_string()));
    let canonical_list: Vec<String> = dangerous_ingredients.iter()
        .map(|x| x.0.clone())
        .collect();

    let answer = canonical_list.join(",");
    println!("Canonical list of dangerous ingredients: {}", answer);
}

fn main() {
    let input_data = read_input_data();
    let mut foods = Vec::new();

    for line in input_data.split('\n') {
        let mut split = line.split(" (contains ");
        let ingredients: Vec<_> = split.next().unwrap().split(' ').collect();
        let allergens: Vec<_> = split.next().unwrap().trim_end_matches(')').split(", ").collect();
        foods.push(Food{ ingredients, allergens });
    }

    solve(&foods);
}
