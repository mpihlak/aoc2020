use aoclib::*;

fn count_neighbors(g: &Grid, row: usize, col: usize) -> i32 {
    let mut neighbors = 0;
    for row_pos in row as i32 - 1 .. row as i32 + 2 {
        for col_pos in col as i32 - 1 .. col as i32 + 2 {
            if row_pos == row as i32 && col_pos == col as i32 {
                continue;
            }
            match g.at(row_pos, col_pos) {
                Some('L') | Some('.') | None => {},
                Some('#') => neighbors += 1,
                Some(other) => panic!("Unexpect char at row={}, col={}: {}", row_pos, col_pos, other),
            }
        }
    }
    neighbors
}

fn iterate<F>(g: &Grid, tolerance: i32, count_fn: F) -> (i32, Grid)
    where F: Fn(&Grid, usize, usize) -> i32
{
    let mut changes = 0;
    let mut result = g.clone();

    for row in 0..g.height {
        for col in 0..g.width {
            let neighbors = count_fn(g, row, col);

            match g.at(row as i32, col as i32) {
                Some('#') => {
                    if neighbors >= tolerance {
                        result.cells[row][col] = 'L';
                        changes += 1;
                    }
                },
                Some('L') => {
                    if neighbors == 0 {
                        result.cells[row][col] = '#';
                        changes += 1;
                    }
                }
                _ => {},
            }
        }
    }
    (changes, result)
}

fn main() {
    let input_data = read_input_data();
    let mut g = Grid::from_str(&input_data);

    let mut iteration = 0;

    loop {
        iteration += 1;
        let (changes, new_grid) = iterate(&g, 4, count_neighbors);
        g = new_grid;

        if changes == 0 {
            println!("Iteration {}, nothing changed. Done here.", iteration);
            break;
        }
    }

    let occupied_seats = g.count_elems('#');
    println!("Stage 1: Done after {} iterations. {} seats are occupied.", iteration, occupied_seats);
}
