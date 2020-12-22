use aoclib::*;
use std::collections::{HashMap};

#[derive(Debug)]
struct Food<'a> {
    ingredients: Vec<&'a str>,
    allergens: Vec<&'a str>,
}

fn solve(foods: &[Food]) -> Option<u32> {
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

    // Now we should just reduce the allergen map, starting from the entry that
    // has just one ingredient.
    // Instead we do this complicated thing ...

    for (allergen, ingredients) in allergen_ingredients.iter_mut() {
        ingredients.sort();
        ingredients.dedup();
        println!("{}: {:?}", allergen, ingredients);
    }

    let mut allergen_map   = HashMap::new();
    let mut ingredient_map = HashMap::new();

    solve_aux(&foods, &allergen_ingredients, &mut allergen_map, &mut ingredient_map);

    //println!("Allergy free ingredients:");
    let mut sum = 0;
    for (ingredient, count) in ingredient_counts.iter() {
        if !ingredient_map.contains_key(&ingredient.to_string()) {
            //println!("{}: {}", ingredient, count);
            sum += count;
        }
    }

    println!("Allergen count = {}", allergen_ingredients.len());
    println!("Allergens: {:?}", allergen_map);
    println!("Ingredients: {:?}", ingredient_map);

    let mut dangerous_ingredients: Vec<(String, String)> = ingredient_map.iter()
        .map(|x| (x.0.to_string(), x.1.to_string()))
        .collect();
    println!("a={:?}", dangerous_ingredients);
    dangerous_ingredients.sort_by(|a,b| a.1.cmp(&b.1.to_string()));
    println!("b={:?}", dangerous_ingredients);
    // NOT the right answer!
    // rcqb,cltx,nrl,qjvvcvz,zqzmzl,tsqpn,tfqsb,xhnk
    // rcqb,nrl,qjvvcvz,cltx,tsqpn,tfqsb,xhnk,zqzmzl
    let canonical_list: Vec<String> = dangerous_ingredients.iter()
        .map(|x| x.0.clone())
        .collect();

    let answer = canonical_list.join(",");
    println!("Canonical list of dangerous ingredients: {}", answer);

    Some(sum)
}

fn solve_aux(
    foods: &[Food],
    allergen_ingredients: &HashMap<&str, Vec<&str>>,
    mut allergen_map: &mut HashMap<String, String>,
    mut ingredient_map: &mut HashMap<String, String>,
) -> bool {
    if allergen_map.len() == 8 { // allergen_ingredients.len() {
        // All allergens mapped
        return true;
    }

    for (allergen, ingredients) in allergen_ingredients.iter() {
        if allergen_map.contains_key(*allergen) {
            continue;
        }

        for ingredient in ingredients.iter() {
            if ingredient_map.contains_key(*ingredient) {
                continue;
            }

            allergen_map.insert(allergen.to_string(), ingredient.to_string());
            ingredient_map.insert(ingredient.to_string(), allergen.to_string());

            if !validate_solution(&foods, &allergen_map, &ingredient_map) {
                allergen_map.remove(&allergen.to_string());
                ingredient_map.remove(&ingredient.to_string());
            } else {
                if solve_aux(&foods,
                    &allergen_ingredients,
                    &mut allergen_map,
                    &mut ingredient_map,)
                {
                    return true;
                }
            }
        }
    }
    false
}

fn validate_solution(
    foods: &[Food],
    allergen_map:   &HashMap<String, String>,
    ingredient_map: &HashMap<String, String>
) -> bool {

    for food in foods.iter() {
        for allergen in food.allergens.iter() {
            let allergen = allergen.to_string();

            if let Some(ingredient) = allergen_map.get(&allergen) {
                // This allergen is already mapped to ingredient. At least one of ingredients for
                // this food must contain it.
                if !food.ingredients.contains(&ingredient.as_str()) {
                    //println!("Expected to find {} in food containing {}", ingredient, allergen);
                    return false;
                }
            } else {
                // This is an unmapped allergen, pick some allergen-free ingreedient
                // and map to this allergen. Fail if we don't find any.

                let mut found_it = false;
                for ingredient in food.ingredients.iter() {
                    let ingredient = ingredient.to_string();

                    if let Some(mapped_allergen) = ingredient_map.get(&ingredient) {
                        // Already mapped to an allergen. See if they match.
                        if *mapped_allergen == allergen {
                            //println!("Ingredient {} already mapped to allergen {}", ingredient, allergen);
                            found_it = true;
                            break;
                        }
                    } else {
                        //println!("Mapping ingredient {} to allergen {}", ingredient, allergen);
                        found_it = true;
                        break;
                    }
                }

                if !found_it {
                    //println!("Did not find an ingredient that could be mapped to {}", allergen);
                    return false;
                }
            }
        }
    }

    // Exhausted the food list. Must be good.
    true
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

    let answer = solve(&foods);
    println!("Stage 1: answer = {:?}", answer);
}
