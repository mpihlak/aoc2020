use aoclib::*;

fn main() {
    let input_data = read_input_data();
    //let input_data= "nwwswee";

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
    // of the grid.
    let grid_width = num_tiles*8;
    let grid_height = grid_width;
    let start_tile_row = num_tiles*4;
    let start_tile_col = start_tile_row;
    let mut grid: Vec<Vec<bool>> = (0..grid_height).map(|_| vec![true; grid_width]).collect();

    for tile_instr in tile_instructions.iter() {
        let mut row = start_tile_row;
        let mut col = start_tile_col;

        for instruction in tile_instr {
            match instruction.as_ref() {
                "se" => { row += 2; col += 1; },
                "sw" => { row += 2; col -= 1; },
                "ne" => { row -= 2; col += 1; },
                "nw" => { row -= 2; col -= 1; },
                "w" => col -= 2,
                "e" => col += 2,
                other => panic!("invalid instruction: {}", other),
            }
        }

        grid[row][col] = !grid[row][col];
    }

    let num_black_tiles: u32 = grid.iter().flatten().map(|x| if *x { 0 } else { 1 }).sum();
    println!("Number of black tiles = {}", num_black_tiles);
}
