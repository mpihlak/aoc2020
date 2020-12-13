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

    let bus_list: Vec<Option<u64>> = raw_bus_list
        .split(',')
        .map(|x| if x == "x" { None } else { Some(x.parse().unwrap()) })
        .collect();

    let mut t = bus_list[0].unwrap();
    let mut step = t;
    let mut offset = 1;

    while offset < bus_list.len() {
        if let Some(bus_id) = bus_list[offset] {
            if (t + offset as u64) % bus_id == 0 {
                offset += 1;
                step = lcm(step, bus_id);
                println!("t = {}, new_step = {}, new_offset = {}", t, step, offset);
            }
            t += step;
        } else {
            offset += 1;
        }
    }

}
