use aoclib::*;

fn adjust(instruction: &str, row: usize, col: usize) -> (usize,usize) {
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
    grid: Vec<Vec<i32>>,
}

impl Tiling {
    fn new(grid: Vec<Vec<i32>>) -> Self {
        Tiling {
            grid,
        }
    }

    fn count_neighbors(&self, row: usize, col: usize) -> i32 {
        let mut neighbors = 0;
        for direction in vec!["e", "se", "sw", "w", "nw", "ne"] {
            let (r, c) = adjust(direction, row, col);
            neighbors += self.grid[r][c];
        }
        neighbors
    }

    fn needs_to_flip(&self, row: usize, col: usize) -> bool {
        let n = self.count_neighbors(row, col);

        if self.grid[row][col] == 1 {
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
        let mut flips = Vec::new();
        let mut visited = std::collections::HashSet::new();

        // Now, this would be a lot cheaper if the black cells were kept in a HashMap
        let mut cells_to_visit = Vec::new();
        for (row_num, row) in self.grid.iter().enumerate() {
            for (col_num, x) in row.iter().copied().enumerate() {
                if x == 1 {
                    cells_to_visit.push((row_num, col_num));
                    visited.insert((row_num, col_num));
                }
            }
        }

        while let Some((row, col)) = cells_to_visit.pop() {
            visited.insert((row, col));

            // Visit only the black tiles, because the only flips are going to be 
            // the black tiles themselves or their white neighbors.
            if self.grid[row][col] == 1 {
                for direction in vec!["e", "se", "sw", "w", "nw", "ne"] {
                    let (r, c) = adjust(direction, row, col);
                    if self.grid[row][col] == 1 && !visited.contains(&(r, c)) {
                        // odd distances from the middle row get an offset
                        let middle = self.grid.len() / 2;
                        let offset = ((r as i32 - middle as i32).abs() % 2) as usize;
                        cells_to_visit.push((r, c + offset));
                    }
                }
            }

            if self.needs_to_flip(row, col) {
                flips.push((row, col));
            }
        }

        // Now, apply the flips
        for (row, col) in flips.iter().copied() {
            self.grid[row][col] = (self.grid[row][col] + 1) % 2;
        }
    }

    fn count_blacks(&self) -> i32 {
        self.grid.iter().flatten().sum()
    }

    #[allow(dead_code)]
    fn display(&self) {
        let mut min_row = 999;
        let mut max_row = 0;
        let mut min_col = 999;
        let mut max_col = 0;

        let mut display_grid: Vec<Vec<char>> = self.grid.iter()
            .map(|row| row.iter().map(|_| '.').collect()).collect();

        for r in 0..self.grid.len() {
            for c in 0..self.grid[r].len() {
                if self.grid[r][c] == 1 {
                    min_row = min_row.min(r-1);
                    max_row = max_row.max(r+1);
                    min_col = min_col.min(c);
                    max_col = max_col.max(c+1);

                    display_grid[r-1][c] = '/';
                    display_grid[r-1][c+1] = '\\';
                    display_grid[r][c] = '#';
                    display_grid[r][c+1] = '|';
                    display_grid[r+1][c] = '\\';
                    display_grid[r+1][c+1] = '/'
                }
            }
        }

        println!("Min row = {}, max row = {}", min_row, max_row);
        println!("Min col = {}, max col = {}", min_col, max_col);

        for r in min_row..max_row+1 {
            print!("{:03}: ", r);
            for c in min_col..max_col+1 {
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

    // Represent the hex grid in a 2d array, with extra blank space
    // for the 6 sides. Start with the reference tile at the middle
    // of the grid. Allocate generously so that we don't run outside of the grid.
    let grid_width = num_tiles*10;
    let grid_height = grid_width;
    let start_tile_row = grid_width/2;
    let start_tile_col = start_tile_row;
    let mut grid: Vec<Vec<i32>> = (0..grid_height).map(|_| vec![0; grid_width]).collect();

    println!("Grid len = {}", grid.len());
    println!("Init pos = {}", start_tile_row);

    for tile_instr in tile_instructions.iter() {
        let mut row = start_tile_row;
        let mut col = start_tile_col;

        for instruction in tile_instr {
            let (r, c) = adjust(instruction, row, col);
            row = r;
            col = c;
        }

        if grid[row][col] == 0 {
            grid[row][col] = 1;
        } else {
            grid[row][col] = 0;
        }
    }

    let num_black_tiles: i32 = grid.iter().flatten().sum();
    println!("Part 1: Number of black tiles = {}", num_black_tiles);
    let mut tiling = Tiling::new(grid);

    for _i in 0..100 {
        tiling.next_iteration();
    }

    println!("Part 2: After 100 days, black tiles = {}", tiling.count_blacks());
}
