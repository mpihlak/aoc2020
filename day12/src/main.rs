use aoclib::*;

fn turn(current_heading: i32, degrees: i32) -> i32 {
    (360 + (current_heading + degrees)) % 360
}

// Heading in degrees to (x, y)
fn to_dir(heading: i32) -> (i32, i32) {
    match heading {
          0 => ( 0, -1),
         90 => ( 1,  0),
        180 => ( 0,  1),
        270 => (-1,  0),
        other => panic!("Invalid heading: {}", other),
    }
}

// Rotate coordinates left or right by degrees (multiple of 90)
fn rotate(x: i32, y: i32, degrees: i32) -> (i32, i32) {
    match degrees {
          0 |  360 => ( x,  y),
        180 | -180 => (-x, -y),
         90 | -270 => (-y,  x),
        270 |  -90 => ( y, -x),
        other => panic!("Invalid rotation: {}", other),
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

    let mut ship_x = 0;
    let mut ship_y = 0;
    let mut wp_x = 10;
    let mut wp_y = -1;

    for (cmd, val) in directions.iter() {
        match cmd {
            'N' => wp_y -= val,
            'S' => wp_y += val,
            'E' => wp_x += val,
            'W' => wp_x -= val,
            'L' => {
                // Rotate wp left N degrees
                let (x, y) = rotate(wp_x, wp_y, -val);
                wp_x = x;
                wp_y = y;
            },
            'R' => {
                // Rotate wp right N degrees
                let (x, y) = rotate(wp_x, wp_y, *val);
                wp_x = x;
                wp_y = y;
            },
            'F' => {
                // Move ship towards waypoint N times
                ship_x += val * wp_x;
                ship_y += val * wp_y;
            },
            other => panic!("Invalid direction command: {}", other),
        }
    }

    let d = ship_x.abs() + ship_y.abs();
    println!("Stage 2: Ship's Manhattan distance = {}", d);
}
