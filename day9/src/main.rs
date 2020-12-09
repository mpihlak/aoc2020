use aoclib::*;

fn main() {
    let data = read_input_data();
    let numbers: Vec<u64> = data
        .split('\n')
        .map(|x| x.parse().unwrap())
        .collect();

    let preamble = 25;
    let mut invalid_number = None;

    for pos in preamble..numbers.len() {
        let window_start = pos - preamble;
        let window_end = pos;
        let mut sum_found = false;
'outer:
        for i in window_start .. (window_end-1) {
            for j in (window_start+1) .. window_end {
                if numbers[pos] == numbers[i] + numbers[j] {
                    sum_found = true;
                    break 'outer;
                }
            }
        }

        if !sum_found {
            println!("Stage 1: {} is not sum of previous {}", numbers[pos], preamble);
            invalid_number = Some(numbers[pos]);
            break;
        }
    }

    let need_this_sum = invalid_number.unwrap();
'outer2:
    for i in 0..numbers.len() {
        let mut sum = numbers[i];
        let mut min_number = numbers[i];
        let mut max_number = numbers[i];
        for num in numbers.iter().skip(i+1) {
            sum += *num;
            max_number = *num.max(&max_number);
            min_number = *num.min(&min_number);
            if sum == need_this_sum {
                println!("Stage 2: sum of min and max = {}", min_number + max_number);
                break 'outer2;
            } else if sum > need_this_sum {
                break;
            }
        }
    }

}
