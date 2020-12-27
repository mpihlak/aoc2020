use std::{env, fs};

fn partition(range_start: i32, range_end: i32, by: &str) -> i32 {
    let range_len = range_end - range_start;

    if range_len <= 1 {
        return range_start;
    }

    for direction in by.chars() {
        match direction {
            'F' | 'L' => return partition(range_start, range_start + range_len/2, &by[1..]),
            'B' | 'R' => return partition(range_end - range_len/2, range_end, &by[1..]),
             other => panic!("Invalid partitioner: {}", other),
        }
    }
    panic!("Why here?");
}

fn main() {
    let filename = env::args().nth(1).unwrap_or("input.txt".into());
    let boarding_pass_data = fs::read_to_string(filename).unwrap();

    let mut max_seat_id = 0;
    let mut seats = Vec::new();
    for boarding_pass in boarding_pass_data.trim_end().split("\n") {
        let row = partition(0, 128, &boarding_pass[..7]);
        let column = partition(0, 8, &boarding_pass[7..]);
        let seat_id = row*8 + column;
        seats.push(seat_id);

        if seat_id > max_seat_id {
            max_seat_id = seat_id;
        }
    }

    println!("Stage 1: Max seat ID = {}", max_seat_id);

    seats.sort();
    for pos in 1..seats.len() {
        if seats[pos] != seats[pos-1] + 1 {
            println!("Stage 2: My seat ID = {}", seats[pos-1] + 1);
            break;
        }
    }
}
