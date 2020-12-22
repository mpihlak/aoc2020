use aoclib::*;
use std::collections::HashSet;

fn play_combat(player1: &[usize], player2: &[usize]) -> Vec<usize> {
    let mut player1 = player1.to_owned();
    let mut player2 = player2.to_owned();

    while !player1.is_empty() && !player2.is_empty() {
        let card1 = player1.remove(0);
        let card2 = player2.remove(0);
        
        if card1 > card2 {
            player1.push(card1);
            player1.push(card2);
        } else {
            player2.push(card2);
            player2.push(card1);
        }
    }

    if player1.len() > player2.len() {
        player1
    } else {
        player2
    }
}

fn key(hand1: &[usize], hand2: &[usize]) -> u128 {
    let u1: u128 = hand1.iter()
        .map(|x| 1 << *x)
        .fold(0, |acc, x| acc | x);

    let u2: u128 = hand2.iter()
        .map(|x| 1 << *x)
        .fold(0, |acc, x| acc | x);

    u1 << 64 | u2
}

fn play_recursive_combat(player1: &[usize], player2: &[usize]) -> (Vec<usize>, bool) {
    let mut played_hands = HashSet::new();
    let mut player1 = player1.to_owned();
    let mut player2 = player2.to_owned();

    while !player1.is_empty() && !player2.is_empty() {
        if played_hands.contains(&key(&player1, &player2)) {
            return (player1, true);
        }

        played_hands.insert(key(&player1, &player2));

        let card1 = player1.remove(0);
        let card2 = player2.remove(0);

        let player1_won = if player1.len() >= card1 && player2.len() >= card2 {
            let (_hand, was_it_player1) = play_recursive_combat(
                &(&player1[..card1]).to_vec(),
                &(&player2[..card2]).to_vec());

            was_it_player1
        } else {
            card1 > card2
        };

        if player1_won {
            player1.push(card1);
            player1.push(card2);
        } else {
            player2.push(card2);
            player2.push(card1);
        }
    }

    if player1.len() > player2.len() {
        (player1, true)
    } else {
        (player2, false)
    }
}

fn main() {
    let input_data = read_input_data();
    let mut players_split = input_data.split("\n\n");

    let player1: Vec<usize> = players_split.next().unwrap()
        .split('\n')
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();
    let player2: Vec<usize> = players_split.next().unwrap()
        .split('\n')
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();

    let result = play_combat(&player1, &player2);
    let answer: u64 = result.iter().enumerate()
        .map(|(pos, val)| (result.len() - pos) as u64 * *val as u64)
        .sum();
    println!("Part 1 answer = {}", answer);

    let (result, _winner) = play_recursive_combat(&player1, &player2);
    let answer: u64 = result.iter().enumerate()
        .map(|(pos, val)| (result.len() - pos) as u64 * *val as u64)
        .sum();
    println!("Part 2 answer = {}", answer);
}
