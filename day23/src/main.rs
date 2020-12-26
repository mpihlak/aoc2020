fn one_move(cups: &[usize], current_pos: usize) -> Vec<usize> {
    let mut result = Vec::new();

    let _current_pos = current_pos;
    let current_pos = current_pos % cups.len();
    let current_cup = cups[current_pos];

    let selected = vec![
        cups[(current_pos+1) % cups.len()],
        cups[(current_pos+2) % cups.len()],
        cups[(current_pos+3) % cups.len()],
    ];

    let mut dst_cup = cups[current_pos];
    loop {
        dst_cup -= 1;
        if dst_cup < 1 {
            dst_cup = cups.len();
        }

        if selected.iter().find(|x| **x == dst_cup).is_none() {
            break;
        }
    }

    let mut i = 0;
    while i < cups.len() {
        let pos = (current_pos + i) % cups.len();

        result.push(cups[pos]);

        if cups[pos] == current_cup {
            i += 3;
        } else if cups[pos] == dst_cup {
            result.extend(&selected);
        }
        i += 1;
    }

    // Rotate the cups to get the correct setup
    for _ in 0..current_pos {
        let cup = result.pop().unwrap();
        result.insert(0, cup);
    }

    result
}

// Move the cups in place returning the next current cup
fn move_cups(cup_index: &mut [usize], current_cup: usize) -> usize {
    // a, b and c are the next 3 cups
    let a = cup_index[current_cup];
    let b = cup_index[a];
    let c = cup_index[b];

    // set new next for the current cup
    cup_index[current_cup] = cup_index[c];

    let mut target_cup = current_cup;
    loop {
        target_cup -= 1;
        if target_cup < 1 {
            target_cup = cup_index.len() - 1;
        }

        if target_cup != a && target_cup != b && target_cup != c {
            break;
        }
    }

    cup_index[c] = cup_index[target_cup];
    cup_index[target_cup] = a;

    cup_index[current_cup]
}

fn cup_labels(cups: &[usize]) -> String {
    let one_pos = cups.iter()
        .enumerate()
        .find(|(_pos, x)| **x == 1)
        .map(|(pos,_)| pos)
        .unwrap();

    let mut result = String::new();
    let chars = b"0123456789";
    for i in 1..cups.len() {
        let pos = (one_pos + i) % cups.len();
        result.push(chars[cups[pos]] as char);
    }
    result
}

fn main() {
    let input_data = vec![4, 7, 6, 1, 3, 8, 2, 5, 9];

    let mut cups = input_data.to_owned();
    for nth_move in 0..100 {
        cups = one_move(&cups, nth_move);
    }

    println!("Part1 answer, final cups = {:?}", cups);
    println!("Labels = {}", cup_labels(&cups));
    println!();

    let mut cups = input_data.to_owned();
    let padding: Vec<usize> = (10..1000001).collect();
    cups.extend(padding);

    // Map of next cups, keyed by cup label
    let mut cup_index = vec![0; cups.len()+1];
    for i in 0..cups.len() {
        let cup = cups[i];
        cup_index[cup] = cups[(i+1) % cups.len()]
    }

    println!("Part2 cup count = {}", cups.len());

    let mut current_cup = cups[0];
    for _nth_move in 0..10000000 {
        current_cup = move_cups(&mut cup_index, current_cup);
    }

    let a = cup_index[1];
    let b = cup_index[a];

    println!("Part2: a = {}, b = {}, a*b = {}", a, b, a as u64 * b as u64);
}
