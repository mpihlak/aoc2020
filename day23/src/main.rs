const MAX_CUP: usize = 9;
const MIN_CUP: usize = 1;

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

    if selected.contains(&1) {
        println!("{:05}: 1 selected", _current_pos);
    }

    let mut dst_cup = cups[current_pos];
    loop {
        dst_cup -= 1;
        if dst_cup < MIN_CUP {
            dst_cup = MAX_CUP;
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

#[allow(dead_code)]
fn cup_labels(cups: &[usize]) -> String {
    let one_pos = cups.iter()
        .enumerate()
        .find(|(_pos, x)| **x == 1)
        .map(|(pos,_)| pos)
        .unwrap();

    let mut result = String::new();
    let chars = b"0123456789";
    for i in 1..cups.len() {
        let pos = (one_pos + i) % MAX_CUP;
        result.push(chars[cups[pos]] as char);
    }
    result
}

#[allow(dead_code)]
fn diff(a: &[usize], b: &[usize]) -> usize {
    let mut diffs = 0;
    for i in 0..a.len() {
        if a[i] != b[i] {
            diffs += 1;
        }
    }
    diffs
}

fn main() {
    let _input_data = vec![4, 7, 6, 1, 3, 8, 2, 5, 9];       // actual input
    let input_data = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];       // sample

    let mut cups = input_data.to_owned();
    for nth_move in 0..100 {
        cups = one_move(&cups, nth_move);
    }

    println!("Part1 answer, final cups = {:?}", cups);
    println!();

    let mut cups = input_data.to_owned();
    let padding: Vec<usize> = (10..101).collect();
    cups.extend(padding);

    let mut counts = std::collections::HashMap::new();

    //println!("Part2 initial cups       = {:?}", cups);
    println!("Part2 cup count = {}", cups.len());
    for nth_move in 0..10000 {
        cups = one_move(&cups, nth_move);

        let pos = cups.iter().position(|x| *x == 1).unwrap();
        let v = vec![
            cups[pos],
            cups[(pos+1) % cups.len()],
            cups[(pos+2) % cups.len()],
        ];

        let entry = counts.entry(v.clone()).or_insert(0);
        *entry += 1;
    }

    println!("Part2 answer, final cups = {:?}", cups);

    /*
    for (k, v) in counts.iter() {
        println!("{:?}: {}", k, v);
    }
    */
}
