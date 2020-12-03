use std::fs;

struct PasswordWithPolicy {
    min: usize,
    max: usize, 
    ch: char,
    passwd: String,
}

impl PasswordWithPolicy {
    // Parse from "5-9 g: ggccggmgn"
    fn from_str(s: &str) -> Self {
        let v: Vec<&str> = s.splitn(3, " ").collect();
        let range: Vec<&str> = v[0].splitn(2, "-").collect();

        PasswordWithPolicy {
            min:    range[0].parse::<usize>().unwrap(),
            max:    range[1].parse::<usize>().unwrap(),
            ch:     v[1].chars().next().unwrap(),
            passwd: v[2].to_string(),
        }
    }

    fn valid_policy_1(&self) -> bool {
        let mut count = 0;
        for c in self.passwd.chars() {
            if c == self.ch {
                count += 1;
            }
        }

        count >= self.min && count <= self.max
    }

    fn valid_policy_2(&self) -> bool {
        let pos1 = self.min - 1;
        let pos2 = self.max - 1;
        let chars: Vec<char> = self.passwd.chars().collect();

        (chars[pos1] == self.ch || chars[pos2] == self.ch) && (chars[pos1] != chars[pos2])
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut valid_passwords = 0;
    for line in input.trim_end().split("\n") {
        let p = PasswordWithPolicy::from_str(&line);

        if p.valid_policy_1() {
            valid_passwords += 1;
        }
    }

    println!("Stage 1, valid passwords = {}", valid_passwords);

    let mut valid_passwords = 0;
    for line in input.trim_end().split("\n") {
        let p = PasswordWithPolicy::from_str(&line);

        if p.valid_policy_2() {
            valid_passwords += 1;
        }
    }

    println!("Stage 2, valid passwords = {}", valid_passwords);
}
