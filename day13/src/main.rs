use aoclib::*;

fn main() {
    let input_data = read_input_data();

    let mut split = input_data.split('\n');
    let earliest_departure: u64 = split.next().unwrap().parse().unwrap();
    let bus_list: Vec<u64> = split.next().unwrap()
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
}
