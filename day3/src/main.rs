use std::{fs, env};
use aoclib::Grid;

fn count_trees(grid: &Grid, right: usize, down: usize) -> u32 {
    let mut trees_found = 0;
    let mut row = 0;
    let mut col = 0;

    while row < grid.height {
        if grid.cells[row][col] == '#' {
            trees_found += 1;
        }

        col = (col + right) % grid.width;
        row = row + down;
    }
    
    trees_found
}

fn main() {
    let mut args = env::args();
    let _ = args.next();
    let filename = args.next().unwrap_or("input.txt".to_string());
    println!("arg = {}", filename);

    let grid_data = fs::read_to_string(filename).unwrap();
    let grid = Grid::from_str(&grid_data);
    println!("grid:\n{}\n", grid.to_str());

    let trees_found = count_trees(&grid, 3, 1);
    println!("Stage 1: trees found = {}", trees_found);

    println!("\nStage 2:");
    let mut total_trees_mult: u64 = 1;
    for slope in vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        let trees_found = count_trees(&grid, slope.0, slope.1);
        total_trees_mult *= trees_found as u64;
        println!("Right {}, down {} = {}", slope.0, slope.1, trees_found);
    }

    println!("Trees multiplied = {}", total_trees_mult);
}
