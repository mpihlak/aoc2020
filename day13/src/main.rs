use aoclib::*;
use num::integer::lcm;

fn main() {
    let input_data = read_input_data();

    let mut split = input_data.split('\n');
    let earliest_departure: u64 = split.next().unwrap().parse().unwrap();
    let raw_bus_list = split.next().unwrap();
    let bus_list: Vec<u64> = raw_bus_list
        .split(',')
        .filter(|x| *x != "x")
        .map(|x| x.parse().unwrap())
        .collect();

    println!("Earliest departure = {}", earliest_departure);
    println!("Busses = {:?}", bus_list);

    let mut earliest_bus_time = std::u64::MAX;
    let mut earliest_bus_id = std::u64::MAX;
    for bus_id in bus_list.iter() {
        let d = earliest_departure / bus_id;
        let closest_bus_time = if earliest_departure % bus_id == 0 {
            d
        } else {
            (d + 1) * bus_id
        };

        if closest_bus_time < earliest_bus_time {
            earliest_bus_time = closest_bus_time;
            earliest_bus_id = *bus_id;
        }
    }

    let answer = (earliest_bus_time - earliest_departure) * earliest_bus_id;
    println!("Stage 1: answer = {}", answer);

    // Convert the bus list into Vec<bus_id, offset>
    let bus_list: Vec<(u64, u64)> = raw_bus_list
        .split(',')
        .enumerate()
        .filter(|x| x.1 != "x")
        .map(|x| (x.1.parse().unwrap(), x.0 as u64))
        .collect();

    let mut t = 0;
    let mut step = bus_list[0].0;
    let mut pos = 1;
    while pos < bus_list.len() {
        t += step;

        let (bus_id, offset) = bus_list[pos];

        if (t + offset) % bus_id == 0 {
            step = lcm(step, bus_id);
            pos += 1;
        }
    }

    println!("Stage 2: answer = {:?}", t);
}
