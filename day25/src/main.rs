fn transform(value: u64, subject_number: u64) -> u64 {
    (value * subject_number) % 20201227
}

fn transform_loop(subject_number: u64, loop_size: usize) -> u64 {
    let mut val = 1;

    for _ in 0..loop_size {
        val = transform(val, subject_number);
    }

    return val;
}

fn find_loop_size(public_key: u64) -> usize {
    let mut val = 1;
    let mut loop_size = 0;
    loop {
        loop_size += 1;
        val = transform(val, 7);
        if val == public_key {
            return loop_size;
        }
    }
}

fn main() {
    let door_public_key = 2084668;
    let card_public_key = 3704642;
    let door_loop_size = find_loop_size(door_public_key);
    let card_loop_size = find_loop_size(card_public_key);
    println!("Door loop size = {}", door_loop_size);
    println!("Card loop size = {}", card_loop_size);
    println!("Encryption key = {}", transform_loop(door_public_key, card_loop_size));
    println!("Encryption key = {}", transform_loop(card_public_key, door_loop_size));
}
