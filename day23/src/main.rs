const MAX_CUP: usize = 9;
const MIN_CUP: usize = 1;

fn one_move(cups: &[usize], current_pos: usize) -> Vec<usize> {
    let mut result = Vec::new();

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
        if dst_cup < MIN_CUP {
            dst_cup = MAX_CUP;
        }

        if selected.iter().find(|x| **x == dst_cup).is_none() {
            break;
        }
    }

    /*
    println!("cups: {:?}", cups);
    println!("current cup: {}", cups[current_pos]);
    println!("picked up: {:?}", selected);
    println!("next cup id: {}", dst_cup);
    */

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

fn main() {
    let input_data = vec![4, 7, 6, 1, 3, 8, 2, 5, 9];       // actual input
    let _input_data = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];       // sample

    let mut cups = input_data.to_owned();
    for nth_move in 0..100 {
        cups = one_move(&cups, nth_move);
    }

    println!("Final cups = {:?}", cups);
    println!("Labels = {}", cup_labels(&cups));
}
