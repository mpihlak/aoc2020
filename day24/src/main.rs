use aoclib::*;
use std::collections::{HashMap, HashSet};

fn adjust(instruction: &str, row: i32, col: i32) -> (i32,i32) {
    match instruction.as_ref() {
        "se" => ( row +  2, col +  1 ),
        "sw" => ( row +  2, col -  1 ),
        "ne" => ( row -  2, col +  1 ),
        "nw" => ( row -  2, col -  1 ),
        "w" => (row, col - 2),
        "e" => (row, col + 2),
        other => panic!("invalid instruction: {}", other),
    }
}

struct Tiling {
    grid: HashMap<(i32,i32), i32>,
}

impl Tiling {
    fn new(grid: HashMap<(i32,i32), i32>) -> Self {
        Tiling {
            grid,
        }
    }

    fn count_neighbors(&self, row: i32, col: i32) -> i32 {
        let mut neighbors = 0;
        for direction in vec!["e", "se", "sw", "w", "nw", "ne"] {
            let (r, c) = adjust(direction, row, col);
            if let Some(cell_value) = self.grid.get(&(r, c)) {
                neighbors += cell_value;
            }
        }
        neighbors
    }

    fn needs_to_flip(&self, tile_color: i32, row: i32, col: i32) -> bool {
        let n = self.count_neighbors(row, col);

        if tile_color == 1 {
            if n == 0 || n > 2 {
                return true;
            }
        } else {
            if n == 2 {
                return true;
            }
        }
        false
    }

    fn next_iteration(&mut self) {
        // Create a list of all the black cells in the grid
        let mut cells_to_visit: Vec<(i32,i32)> = self.grid.iter()
            .filter(|(_key, val)| **val == 1)
            .map(|(key, _val)| *key)
            .collect();
        let mut visited: HashSet<(i32,i32)> = cells_to_visit.iter().copied().collect();

        let mut flips = Vec::new();
        while let Some((row, col)) = cells_to_visit.pop() {
            visited.insert((row, col));

            let cell = *self.grid.get(&(row, col)).unwrap_or(&0);

            // Visit only the black tiles, because the only flips are going to be 
            // the black tiles themselves or their white neighbors.

            if cell == 1 {
                for direction in vec!["e", "se", "sw", "w", "nw", "ne"] {
                    let (r, c) = adjust(direction, row, col);
                    if !visited.contains(&(r, c)) {
                        cells_to_visit.push((r, c));
                    }
                }
            }

            if self.needs_to_flip(cell, row, col) {
                flips.push((row, col));
            }
        }

        // Now, apply the flips
        for (row, col) in flips.iter().copied() {
            let cell = self.grid.entry((row, col)).or_insert(0);
            *cell = (*cell + 1) % 2;
        }
    }

    fn count_blacks(&self) -> i32 {
        self.grid.values().sum()
    }

    #[allow(dead_code)]
    fn display(&self) {

        // 1000x1000 should be enough for everyone
        let width  = 1000;
        let height = 1000;
        let mid_row = height / 2;
        let mid_col = width / 2;

        let mut display_grid: Vec<Vec<char>> =
            (0..height).map(|_|
                (0..width).map(|_| '.').collect()
            ).collect();

        let mut min_row = std::usize::MAX;
        let mut min_col = std::usize::MAX;
        let mut max_row = std::usize::MIN;
        let mut max_col = std::usize::MIN;

        for ((r, c), v) in self.grid.iter() {
            let r = ((mid_row as i32) + r) as usize;
            let c = ((mid_col as i32) + c) as usize;

            if *v == 1 {
                display_grid[r-1][c] = '/';
                display_grid[r-1][c+1] = '\\';
                display_grid[r][c] = '#';
                display_grid[r][c+1] = '|';
                display_grid[r+1][c] = '\\';
                display_grid[r+1][c+1] = '/';

                min_row = min_row.min(r);
                max_row = max_row.max(r);
                min_col = min_col.min(c);
                max_col = max_col.max(c);
            }
        }

        for r in min_row-1..max_row+2 {
            print!("{:03}: ", r);
            for c in min_col..max_col+2 {
                print!("{}", display_grid[r][c]);
            }
            println!();
        }

    }
}

fn main() {
    let input_data = read_input_data();

    let mut tile_instructions = Vec::new();
    for instr in input_data.split('\n') {
        let mut parsed_instructions: Vec<&str> = Vec::new();
        let mut instr = instr;

        while instr.len() > 0 {
            for prefix in vec!["se", "sw", "ne", "nw", "w", "e"] {
                if let Some(remainder) = instr.strip_prefix(prefix) {
                    parsed_instructions.push(prefix);
                    instr = remainder;
                    break;
                }
            }
        }
        tile_instructions.push(parsed_instructions);
    }

    let num_tiles = tile_instructions.len();
    println!("num tiles = {}", num_tiles);

    // Think about the hex grid as a 2d array, with extra space allocated
    // for the 6 sides. Like this:
    // ..........
    // ./\/\.....
    // .#|#|.....
    // /\/\/\../\
    // #|..#|..#|
    // \/\.\/..\/
    // .#|.......
    // .\/.......
    // ..........

    let start_tile_row = 0i32;
    let start_tile_col = 0i32;

    let mut grid = HashMap::new();
    for tile_instr in tile_instructions.iter() {
        let mut row = start_tile_row;
        let mut col = start_tile_col;

        for instruction in tile_instr {
            let (r, c) = adjust(instruction, row, col);
            row = r;
            col = c;
        }

        let entry = grid.entry((row, col)).or_insert(0);
        *entry = (*entry + 1) % 2;
    }

    let num_black_tiles: i32 = grid.values().sum();
    println!("Part 1: Number of black tiles = {}", num_black_tiles);

    let mut tiling = Tiling::new(grid);
    tiling.display();

    for _i in 0..100 {
        tiling.next_iteration();
    }

    println!("Part 2: After 100 days, black tiles = {}", tiling.count_blacks());
}
