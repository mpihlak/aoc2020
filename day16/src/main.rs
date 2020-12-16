use aoclib::*;

use std::collections::HashMap;

#[derive(Debug)]
struct ValidationRule {
    name: String,
    ranges: Vec<(i32,i32)>,
}

fn parse_int_range(s: &str) -> (i32, i32) {
    let mut split = s.split('-');
    let i1 = split.next().unwrap().parse().unwrap();
    let i2 = split.next().unwrap().parse().unwrap();
    (i1, i2)
}

impl ValidationRule {
    fn from_str(rule_str: &str) -> Self {
        let mut split = rule_str.split(": "); 
        let name = split.next().unwrap().to_string();
        let the_rest = split.next().unwrap();
        let mut split = the_rest.split(' ');
        let first_set = split.next().unwrap();
        let _or = split.next().unwrap();
        let second_set = split.next().unwrap();

        let mut ranges = vec![];
        ranges.push(parse_int_range(first_set));
        ranges.push(parse_int_range(second_set));

        ValidationRule {
            name,
            ranges,
        }
    }

    fn is_valid_field(&self, field: i32) -> bool {
        self.ranges.iter().any(|(start, end)| field >= *start && field <= *end)
    }
}

fn is_within_rules(rules_list: &[ValidationRule], field: i32) -> bool {
    for rule in rules_list.iter() {
        if rule.is_valid_field(field) {
            return true;
        }
    }
    false
}

// Update the possible validators for the field
fn update_validators_for(
    rules_list: &[ValidationRule],
    validation_rules: &mut HashMap<String, bool>,
    field: i32)
{
    for rule in rules_list.iter() {
        if rule.is_valid_field(field) {
            // If it's already there we leave it as is. If it's not, add as valid.
            let _e = validation_rules.entry(rule.name.clone()).or_insert(true);
        } else {
            // Otherwise set to false
            validation_rules.insert(rule.name.clone(), false);
        }
    }
}

fn main() {
    let input_data = read_input_data();
    let mut s = input_data.split("\n\n");
    let rules_str = s.next().unwrap();
    let my_ticket_str = s.next().unwrap();
    let tickets_str = s.next().unwrap();

    let my_ticket: Vec<i32> = my_ticket_str
        .split('\n')
        .nth(1)
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let tickets: Vec<Vec<i32>> = tickets_str
        .split('\n')
        .skip(1)
        .map(|x| x.split(',').map(|x| x.parse().unwrap()).collect())
        .collect();

    let mut rules_list = vec![];
    for rule in rules_str.split('\n') {
        rules_list.push(ValidationRule::from_str(rule));
    }

    let mut sum_invalid_values = 0;
    let mut invalid_tickets = vec![false; tickets.len()];

    for (pos, ticket) in tickets.iter().enumerate() {
        for field in ticket.iter() {
            if !is_within_rules(&rules_list, *field) {
                sum_invalid_values += field;
                invalid_tickets[pos] = true;
            }
        }
    }

    println!("Stage 1: Answer = {}", sum_invalid_values);

    let ticket_field_count = my_ticket.len();
    let mut field_validators = vec![HashMap::new(); ticket_field_count];

    for (ticket_pos, ticket) in tickets.iter().enumerate() {
        if invalid_tickets[ticket_pos] {
            continue;
        }
        for (field_pos, field_val) in ticket.iter().enumerate() {
            update_validators_for(&rules_list, &mut field_validators[field_pos], *field_val);
        }
    }

    // Now build the field by reducing the validators so that we choose the entry with only 1
    // validator and then remove that validator from all the other lists.
    // Brute force will do just fine here.

    let mut field_map = Vec::new();

    while field_map.len() < field_validators.len() {
        let mut mapped_field = None;

        // Find the field that has just 1 validator
        for (pos, validators) in field_validators.iter_mut().enumerate() {
            let v: Vec<String> = validators.iter()
                .filter(|(_k, v)| **v)
                .map(|(k, _v)| k.to_owned())
                .collect();

            if v.len() == 1 {
                mapped_field = Some((v[0].clone(), pos));
                break;
            }
        }

        // If we found a validator, then that must be the only match and
        // we must remove it from the rest of the fields.
        if let Some((field_name, pos)) = mapped_field {
            for validator in field_validators.iter_mut() {
                validator.remove(&field_name);
            }
            field_map.push((field_name, pos));
        }
    }

    let mut answer: u64 = 1;
    for (field_name, field_pos) in field_map.iter() {
        if field_name.starts_with("departure") {
            answer *= my_ticket[*field_pos] as u64;
        }
    }

    println!("Stage 2: answer = {}", answer);
}
