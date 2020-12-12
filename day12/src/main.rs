use aoclib::*;

fn turn(current_heading: i32, degrees: i32) -> i32 {
    let heading = (current_heading + degrees) % 360;
    if heading < 0 {
        heading + 360
    } else {
        heading
    }
}

fn to_dir(heading: i32) -> (i32, i32) {
    match heading {
        0 => (-1, 0),
        90 => (0, 1),
        180 => (1, 0),
        270 => (0, -1),
        other => panic!("Invalid heading: {}", other),
    }
}

fn main() {
    let input_data = read_input_data();

    let directions: Vec<(char, i32)> = input_data
        .split('\n')
        .map(|x| (x.chars().next().unwrap(), x[1..].parse::<i32>().unwrap()))
        .collect();
    println!("Directions = {:?}", directions);

    let mut heading = 90;
    let mut x_pos = 0;
    let mut y_pos = 0;

    for (cmd, val) in directions.iter() {
        match cmd {
            'N' => x_pos -= val,
            'S' => x_pos += val,
            'E' => y_pos += val,
            'W' => y_pos -= val,
            'L' => heading = turn(heading, -val),
            'R' => heading = turn(heading, *val),
            'F' => {
                let d = to_dir(heading);
                x_pos += d.0 * val;
                y_pos += d.1 * val;
            },
            other => panic!("Invalid direction command: {}", other),
        }
    }

    println!("Ship's position: x={}, y={}", x_pos, y_pos);
    let d = x_pos.abs() + y_pos.abs();
    println!("Stage 1: Ship's Manhattan distance = {}", d);
}
