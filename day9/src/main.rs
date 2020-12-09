use aoclib::*;

fn main() {
    let data = read_input_data();
    let numbers: Vec<u64> = data
        .split('\n')
        .map(|x| x.parse().unwrap())
        .collect();

    let preamble = 25;

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
            break;
        }
    }
}
