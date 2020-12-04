use std::{fs, env};
use std::collections::{HashSet, HashMap};


fn main() {
    let mut args = env::args();
    let _ = args.next();
    let filename = match args.next() {
        Some(name) => name,
        _ => "input.txt".to_string(),
    };

    let required_fields: HashSet<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .map(|x| *x)
        .collect();

    let input = fs::read_to_string(filename).unwrap();

    let mut num_valid_passports = 0;
    for passport_data in input.trim_end().split("\n\n") {
        let passport_data = passport_data.replace("\n", " ");

        let mut _num_fields = 0;
        let mut num_required_fields = 0;
        for field_data in passport_data.split(" ") {
            let mut fs = field_data.split(":");
            let name = fs.next().unwrap();
            let _value = fs.next().unwrap();
            _num_fields += 1;
            if required_fields.contains(name) {
                num_required_fields += 1;
            }
        }

        if num_required_fields == required_fields.len() {
            num_valid_passports += 1;
        }
    }

    println!("Stage 1: valid passports = {}", num_valid_passports);

    let mut num_valid_passports = 0;
    for passport_data in input.trim_end().split("\n\n") {
        let passport_data = passport_data.replace("\n", " ");

        let mut passport_fields = HashMap::new();
        for field_data in passport_data.split(" ") {
            let mut fs = field_data.split(":");
            let name = fs.next().unwrap();
            let value = fs.next().unwrap();

            passport_fields.insert(name, value);
        }

        let mut valid_passport = true;
        for field in required_fields.iter() {
            if let Some(val) = passport_fields.get(field) {
                let valid_field_value = match field {
                    &"byr" => {
                        let v = val.parse::<i32>().unwrap();
                        v >= 1920 && v <= 2002
                    },
                    &"iyr" => {
                        let v = val.parse::<i32>().unwrap();
                        v >= 2010 && v <= 2020
                    },
                    &"eyr" => {
                        let v = val.parse::<i32>().unwrap();
                        v >= 2020 && v <= 2030
                    },
                    &"hgt" => {
                        if val.ends_with("cm") {
                            let v = val[..val.len()-2].parse::<i32>().unwrap();
                            v >= 150 && v <= 193
                        } else if val.ends_with("in") {
                            let v = val[..val.len()-2].parse::<i32>().unwrap();
                            v >= 59 && v <= 76
                        } else {
                            false
                        }
                    },
                    &"hcl" => {
                        //println!("field = {}, value = {}", field, val);
                        let mut valid_chars = true;
                        for c in val[1..].chars() {
                            if ! ((c >= 'a' && c <= 'z') || (c >= '0' && c <= '9')) {
                                valid_chars = false;
                                break;
                            }
                        }
                        valid_chars && val.starts_with("#") && val.len() == 7
                    },
                    &"ecl" => {
                        vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                            .iter()
                            .find(|x| *x == val)
                            .is_some()
                    },
                    &"pid" => {
                        val.parse::<i32>().is_ok() && val.len() == 9
                    },
                    unknown => {
                        panic!("something's wrong, unknown field: {}", unknown);
                    }
                };
                if !valid_field_value {
                    valid_passport = false;
                    break;
                }
            } else {
                valid_passport = false;
                break;
            }
        }

        if valid_passport {
            //println!("  valid: {}", passport_data);
            num_valid_passports += 1;
        } else {
            //println!("invalid: {}", passport_data);
        }
    }

    println!("Stage 2: valid passports = {}", num_valid_passports);
}
