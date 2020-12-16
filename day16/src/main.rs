use aoclib::*;

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
}

fn is_valid_field(rules_list: &Vec<ValidationRule>, field: i32) -> bool {
    for rule in rules_list.iter() {
        for r in rule.ranges.iter() {
            if field >= r.0 && field <= r.1 {
                return true;
            }
        }
    }
    false
}

fn main() {
    let input_data = read_input_data();
    let mut s = input_data.split("\n\n");
    let rules_str = s.next().unwrap();
    let _my_ticket_str = s.next().unwrap();
    let nearby_tickets_str = s.next().unwrap();

    let nearby_tickets: Vec<Vec<i32>> = nearby_tickets_str
        .split('\n')
        .skip(1)
        .map(|x| x.split(',').map(|x| x.parse().unwrap()).collect())
        .collect();

    let mut rules_list = vec![];
    for rule in rules_str.split('\n') {
        rules_list.push(ValidationRule::from_str(rule));
    }

    let mut sum_invalid_values = 0;
    for ticket in nearby_tickets {
        for field in ticket.iter() {
            if !is_valid_field(&rules_list, *field) {
                sum_invalid_values += field;
            }
        }
    }

    println!("Stage 1: Answer = {}", sum_invalid_values);
}
