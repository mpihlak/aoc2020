use aoclib::*;

fn turn(current_heading: i32, degrees: i32) -> i32 {
    let heading = (current_heading + degrees) % 360;
    if heading < 0 {
        heading + 360
    } else {
        heading
    }
}

// Heading in degrees to (x, y)
fn to_dir(heading: i32) -> (i32, i32) {
    match heading {
        0 => (0, -1),
        90 => (1, 0),
        180 => (0, 1),
        270 => (-1, 0),
        other => panic!("Invalid heading: {}", other),
    }
}

fn main() {
    let input_data = read_input_data();

    let directions: Vec<(char, i32)> = input_data
        .split('\n')
        .map(|x| (x.chars().next().unwrap(), x[1..].parse::<i32>().unwrap()))
        .collect();

    let mut heading = 90;
    let mut x_pos = 0;
    let mut y_pos = 0;

    for (cmd, val) in directions.iter() {
        match cmd {
            'N' => y_pos -= val,
            'S' => y_pos += val,
            'E' => x_pos += val,
            'W' => x_pos -= val,
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

    let d = x_pos.abs() + y_pos.abs();
    println!("Stage 1: Ship's Manhattan distance = {}", d);
}
