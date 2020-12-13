use aoclib::*;

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

    println!("Earliest possible bus time = {}, Bus Id = {}", earliest_bus_time, earliest_bus_id);
    let answer = (earliest_bus_time - earliest_departure) * earliest_bus_id;
    println!("Stage 1: answer = {}", answer);

    let bus_list: Vec<Option<u64>> = raw_bus_list
        .split(',')
        .map(|x| if x == "x" { None } else { Some(x.parse().unwrap()) })
        .collect();
    println!("Stage 2 list = {:?}", bus_list);

    // period is at least min bus_id * max bus_id?
    // t % min_bus_id == 0 && t+n % max_bus_id == 0
    // maybe find the first occurrence of this and then get a gcd
    // then step forward with the gcd as a step and validate?

    // Brute force it
    let mut t = 0;
    loop {
        let mut have_match = true;
        for (pos, maybe_bus_id) in bus_list.iter().enumerate() {
            if let Some(bus_id) = maybe_bus_id {
                if (t + pos as u64) % bus_id != 0 {
                    have_match = false;
                    break;
                }
            }
        }
        if have_match {
            println!("Found it at position {}", t);
        }
        t += 1;
    }
}
