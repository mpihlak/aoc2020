use aoclib::*;

#[derive(Debug,Clone)]
struct Tile {
    sides: Vec<String>,
    grid: Grid,
    tile_id: u64,
}

impl Tile {
    
    fn from_str(tile_str: &str) -> Self {
        let mut tile_id_data = tile_str.split(":\n");
        let tile_id: u64 = tile_id_data.next().unwrap()
            .strip_prefix("Tile ").unwrap()
            .parse().unwrap();
        let tile_data = tile_id_data.next().unwrap();
        let grid = Grid::from_str(tile_data);

        let top: String = grid.cells[0].iter()
            .map(|c| *c)
            .collect();
        let right: String = grid.cells.iter()
            .map(|v| v[grid.width-1])
            .collect();
        let bottom: String = grid.cells[grid.height-1].iter()
            .map(|c| *c)
            .collect();
        let left: String = grid.cells.iter()
            .map(|v| v[0])
            .collect();

        Tile {
            tile_id,
            grid,
            sides: vec![top, right, bottom, left],
        }
    }

    #[allow(dead_code)]
    fn to_str(&self) -> String {
        let w = self.grid.cells.len();
        println!("w={}",w);
        let mut res = String::new(); //format!("Tile {}\n", self.tile_id);

        for row in 0..w {
            let s: String = self.grid.cells[row].iter().collect();
            res.push_str(&format!("{}\n", s));
        }
        res.push('\n');
        res
    }


    // Rotate clockwise
    //  4 3 1    8 2 4
    //  2 7 6 -> 9 7 3
    //  8 9 0    0 6 1
    fn rotate(&self) -> Tile {
        let mut rotated_grid = self.grid.clone();

        assert_eq!(self.grid.width, self.grid.height);
        let side_len = self.grid.width;
        for i in 0..side_len {
            for j in 0..side_len {
                rotated_grid.cells[j][side_len-i-1] = self.grid.cells[i][j];
            }
        }

        Tile {
            tile_id: self.tile_id,
            grid: rotated_grid,
            sides: vec![
                self.sides[3].chars().rev().collect::<String>(),
                self.sides[0].clone(),
                self.sides[1].chars().rev().collect::<String>(),
                self.sides[2].clone() ],
        }
    }

    // Flip: 0 - nothing, 1 - upside down, 2 - left right
    //  4 3 1    8 9 0
    //  2 7 6 -> 2 7 6
    //  8 9 0    4 3 1
    fn flip(&self, how: usize) -> Tile {
        let mut flipped_grid = self.grid.clone();
        let side_len = self.grid.width;

        if how == 1 {
            for i in 0..side_len {
                for j in 0..side_len {
                    flipped_grid.cells[side_len-i-1][j] = self.grid.cells[i][j];
                }
            }

            Tile {
                tile_id: self.tile_id,
                grid: flipped_grid,
                sides: vec![
                    self.sides[2].clone(),
                    self.sides[1].chars().rev().collect::<String>(),
                    self.sides[0].clone(),
                    self.sides[3].chars().rev().collect::<String>(),
                ],
            }
        } else if how == 2 {
            for i in 0..side_len {
                for j in 0..side_len {
                    flipped_grid.cells[i][side_len-j-1] = self.grid.cells[i][j];
                }
            }

            Tile {
                tile_id: self.tile_id,
                grid: flipped_grid,
                sides: vec![
                    self.sides[0].chars().rev().collect::<String>(),
                    self.sides[3].clone(),
                    self.sides[2].chars().rev().collect::<String>(),
                    self.sides[1].clone(),
                ]
            }
        } else {
            self.clone()
        }
    }
}

#[derive(Debug)]
struct Puzzle {
    tiles: Vec<Tile>,
    side_len: usize,
}

impl Puzzle {

    fn new(tiles: Vec<Tile>) -> Self {
        let side_len = (tiles.len() as f64).sqrt() as usize;
        Puzzle {
            tiles,
            side_len,
        }
    }

    #[allow(dead_code)]
    fn left_matches_right(a: &Option<Tile>, b: &Tile) -> bool {
        if let Some(left_tile) = a {
            left_tile.sides[1] == b.sides[3]
        } else {
            true
        }
    }

    #[allow(dead_code)]
    fn top_matches_bottom(a: &Option<Tile>, b: &Tile) -> bool {
        if let Some(top_tile) = a {
            top_tile.sides[0] == b.sides[2]
        } else {
            true
        }
    }

    #[allow(dead_code)]
    fn matches_top_left(a: &Tile, left: &Option<Tile>, top: &Option<Tile>) -> bool {
        Puzzle::left_matches_right(left, a) && Puzzle::top_matches_bottom(top, a)
    }

    #[allow(dead_code)]
    fn place_tile(
        &self,
        pos: usize,
        mut used: &mut Vec<bool>,
        used_count: usize,
        mut solution: &mut Vec<Vec<Option<Tile>>>,
    ) -> Option<Vec<Vec<Option<Tile>>>>
    {
        if used_count >= self.tiles.len() {
            let a = solution[0][0].as_ref().unwrap().tile_id;
            let b = solution[0][self.side_len-1].as_ref().unwrap().tile_id;
            let c = solution[self.side_len-1][0].as_ref().unwrap().tile_id;
            let d = solution[self.side_len-1][self.side_len-1].as_ref().unwrap().tile_id;
            let answer = a*b*c*d;
            println!("Found a solution: {} [{}, {}, {}, {}]", answer, a, b, c, d);
            return Some(solution.to_vec());
        }

        let row = pos / self.side_len;
        let col = pos % self.side_len;

        for tile_pos in 0..self.tiles.len() {
            if used[tile_pos] {
                continue;
            }

            let left_tile = if col > 0 { solution[row][col-1].clone() } else { None };
            let top_tile  = if row > 0 { solution[row-1][col].clone() } else { None };
            let mut tile = self.tiles[tile_pos].clone();

            for flip_how in 0..3 {
                tile = tile.flip(flip_how);
                for _rotation in 0..4 {
                    if Puzzle::matches_top_left(&tile, &left_tile, &top_tile) {
                        used[tile_pos] = true;
                        solution[row][col] = Some(tile.clone());
                        if let Some(res) = self.place_tile(
                                pos+1,
                                &mut used,
                                used_count+1,
                                &mut solution) {
                            return Some(res);
                        }
                    }
                    tile = tile.rotate();
                }
            }

            solution[row][col] = None;
            used[tile_pos] = false;
        }

        None
    }

    // Knowing the corner pieces try to put together the whole image.
    fn solve(&self, corners: &[u64]) -> Option<Vec<Vec<Option<Tile>>>> {
        let mut used: Vec<bool> = (0..self.tiles.len()).map(|_| false).collect();
        let mut solution = Vec::new();
        for _ in 0..self.side_len {
            let v: Vec<_> = (0..self.side_len).map(|_| None).collect();
            solution.push(v);
        }

        for corner in corners.iter() {
            let mut tile = self.tiles.iter()
                .find(|x| x.tile_id == *corner)
                .unwrap()
                .clone();
            let tile_pos = self.tiles.iter().enumerate()
                .find(|(_, x)| x.tile_id == *corner)
                .map(|(pos, _)| pos)
                .unwrap();

            used[tile_pos] = true;
            for how in 0..3 {
                tile = tile.flip(how);
                for _ in 0..4 {
                    solution[0][0] = Some(tile.clone());
                    if let Some(answer) = self.place_tile(1, &mut used, 1, &mut solution) {
                        return Some(answer);
                    }
                    tile = tile.rotate();
                }
            }
            used[tile_pos] = false;

            break;
        }
        None
    }

    // Find the edges of the image. We know that the edges will not match any
    // other tile, so this should give us 4 tiles that have 2 sides that don't
    // have any matches to other tiles (no matter how rotated or flipped).
    fn find_corner_tiles(&self) -> Vec<u64> {
        let mut corners = Vec::new();

        for i in 0..self.tiles.len() {
            let tile = &self.tiles[i];
            let mut count_matching_sides = vec![0; 4];

            for j in 0..self.tiles.len() {
                if i == j {
                    continue;
                }

                let mut other = self.tiles[j].clone();

                for how in 0..3 {
                    other = other.flip(how);
                    for _rotation in 0..4 {
                        for k in 0..4 {
                            if tile.sides[k] == other.sides[k] {
                                count_matching_sides[k] = 1;
                            }
                        }

                        other = other.rotate();
                    }
                }
            }

            let count: i32 = count_matching_sides.iter().sum();
            if count == 2 {
                corners.push(tile.tile_id);
            }
        }
        corners
    }

    fn merge_to_tile(&self, image: &Vec<Vec<Option<Tile>>>) -> Tile {
        let mut image_str = String::new();
        let tile_side_len = image[0][0].as_ref().unwrap().grid.cells.len();

        for row in 0..image.len() {
            for tile_row in 1..tile_side_len-1 {
                let mut image_line_str = String::new();

                for col in 0..image[row].len() {
                    let tile = image[row][col].as_ref().unwrap();

                    for tile_col in 1..tile_side_len-1 {
                        // XXX: Why are the rows reversed here?
                        image_line_str.push(tile.grid.cells[tile_side_len - tile_row - 1][tile_col]);
                    }
                }

                image_str.push_str(&image_line_str);
                image_str.push('\n');
            }
        }

        Tile {
            tile_id: 0,
            sides: vec![],
            grid: Grid::from_str(&image_str),
        }
    }

}

#[allow(dead_code)]
fn test_rotate() {
    let mut tile = Tile::from_str(
"Tile 1:
##.#
#...
#.##
.#..");
    println!("Original\n{}", tile.to_str());
    for i in 0..4 {
        tile = tile.rotate();
        println!("Rotate {}\n{}", i, tile.to_str());
    }
}

#[allow(dead_code)]
fn test_flip() {
    let mut tile = Tile::from_str(
"Tile 1:
##.#
#...
#.##
.#..");
    println!("Original\n{}", tile.to_str());
    for i in 0..3 {
        tile = tile.flip(i);
        println!("Flip {}\n{}", i, tile.to_str());
    }
}

// Find the sea monsters and count the '#' characters that are not
// part of the monster.
fn calculate_roughness(image_tile: &Tile) -> u32 {
    0
}

fn main() {
    let input_data = read_input_data();
    let mut tiles = Vec::new();
    for tile_str in input_data.split("\n\n") {
        tiles.push(Tile::from_str(tile_str));
    }

    let puzzle = Puzzle::new(tiles);
    let corners = puzzle.find_corner_tiles();
    let answer: u64 = corners.iter().product();
    println!("Stage 1: answer = {:?}", answer);

    let unprocessed_image = puzzle.solve(&corners).unwrap();
    println!("Removing edges");
    let image_tile = puzzle.merge_to_tile(&unprocessed_image);
    println!("Merged image:\n{}", image_tile.to_str());

    let answer = calculate_roughness(&image_tile);
    println!("Stage 2: answer = {}", answer);
}
