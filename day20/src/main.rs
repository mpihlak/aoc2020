use aoclib::*;

#[derive(Debug,Clone)]
struct Tile {
    sides: Vec<String>,
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
            sides: vec![top, right, bottom, left],
        }
    }

    #[allow(dead_code)]
    fn to_str(&self) -> String {
        let w = self.sides[0].len();
        let h = w;
        let mut res = String::new(); // format!("Tile {}\n", self.tile_id);

        res.push_str(&self.sides[0]);
        res.push('\n');

        for i in 1..h-1 {
            let c = self.sides[3].chars().nth(i).unwrap();
            res.push(c);
            res.push_str(&".".repeat(w-2));
            let c = self.sides[1].chars().nth(i).unwrap();
            res.push(c);
            res.push('\n');
        }

        res.push_str(&self.sides[2]);
        res.push('\n');
        res
    }


    // Rotate clockwise
    fn rotate(&self) -> Tile {
        Tile {
            tile_id: self.tile_id,
            sides: vec![
                self.sides[3].chars().rev().collect::<String>(),
                self.sides[0].clone(),
                self.sides[1].chars().rev().collect::<String>(),
                self.sides[2].clone() ],
        }
    }

    // Flip: 0 - nothiong, 1 - upside down, 2 - left right
    fn flip(&self, how: usize) -> Tile {
        let sides = if how == 1 {
            vec![
                self.sides[2].clone(),
                self.sides[1].chars().rev().collect::<String>(),
                self.sides[0].clone(),
                self.sides[3].chars().rev().collect::<String>(),
            ]
        } else if how == 2 {
            vec![
                self.sides[0].chars().rev().collect::<String>(),
                self.sides[3].clone(),
                self.sides[2].chars().rev().collect::<String>(),
                self.sides[1].clone(),
            ]
        } else {
            self.sides.clone()
        };

        Tile {
            tile_id: self.tile_id,
            sides
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
    ) -> Option<u64>
    {
        if used_count >= self.tiles.len() {
            let a = solution[0][0].as_ref().unwrap().tile_id;
            let b = solution[0][self.side_len-1].as_ref().unwrap().tile_id;
            let c = solution[self.side_len-1][0].as_ref().unwrap().tile_id;
            let d = solution[self.side_len-1][self.side_len-1].as_ref().unwrap().tile_id;
            println!("Found a solution: [{}, {}, {}, {}]", a, b, c, d);
            return Some(a*b*c*d);
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

    #[allow(dead_code)]
    fn count_unique_tiles(&self) -> usize {
        let mut h = std::collections::HashMap::new();

        for tile in self.tiles.iter() {
            let entry = h.entry(tile.to_str()).or_insert(0);
            *entry += 1;
        }

        h.keys().len()
    }

    #[allow(dead_code)]
    fn solve_by_bruteforce(&self) -> Option<u64> {
        let mut used: Vec<bool> = (0..self.tiles.len()).map(|_| false).collect();
        let mut solution = Vec::new();
        for _ in 0..self.side_len {
            let v: Vec<_> = (0..self.side_len).map(|_| None).collect();
            solution.push(v);
        }

        self.place_tile(0, &mut used, 0, &mut solution)
    }

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
                println!("Tile {}: matching sides = {}", tile.tile_id, count);
                corners.push(tile.tile_id);
            }
        }
        corners
    }

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
}
