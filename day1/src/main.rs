use std::fs;

fn main() {
    let digits_str = fs::read_to_string("input.txt").unwrap();
    let digits: Vec<i32> = digits_str
        .trim_end()
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap()).collect();

    // Sum of 2 entries == 2020
    for i in 0..digits.len()-1 {
        for j in i+1..digits.len() {
            let (a, b) = (digits[i], digits[j]);
            if a + b == 2020 {
                println!("Stage 1 answer = {}", a*b);
            }
        }
    }

    // 3 entries == 2020
    for i in 0..digits.len()-2 {
        for j in i+1..digits.len()-1 {
            for k in j+1..digits.len() {
                let (a, b, c) = (digits[i], digits[j], digits[k]);
                if a + b + c == 2020 {
                    println!("Stage 2 answer = {}", a*b*c);
                }
            }
        }
    }

}
