use aoclib::*;

/*
 * Notes:
 *
 * We don't need to keep the whole tiles in part1, we just need the edges.
 * Always counting the bits clockwise
 * So total 4x 10bits. Testing for edge match is a straigthforward comparison.
 * Rotation clockwise is a shift right
 * Flips require reversing and swapping the fields.
 *
 * ##.  110 001 110 011
 * #..
 * .##
 *
 * Rotate left
 * .##  011 110 001 110
 * #.#
 * #..
 *
 * Once more
 * ##.  110 011 110 001
 * ..#
 * .##
 *
 * Flip the above L-R (reverse top & bottom, reverse & swap left & right)
 * .##  011 100 011 110
 * #..
 * ##.
 *
 * Flip T-B
 * ##.  110 001 110 011 (reverse & swap top & bottom, reverse left & right)
 * #..
 * .##
 *
 * For every orientation of t
 *    Add to set T
 *    For every orientation of u
 *        Add to set T
 *            For every orientation of v
 *                Add to set T
 *                   ...
 *    If T is a full rectangle then success
 *
 * 
 */
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
        let mut res = format!("Tile {}\n", self.tile_id);

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
}

impl Puzzle {

    fn is_square(solution: &[(i32, i32)]) -> bool {
        let width: i32 = solution.iter()
            .map(|(_x,y)| y.abs())
            .sum();
        let height: i32 = solution.iter()
            .map(|(x,_y)| x.abs())
            .sum();
        width == height
    }

    // Check if the tiles can be matched side by side and
    // in what direction the b side will be.
    // TODO: Tidy up
    fn match_tiles(a: &Tile, b: &Tile) -> Vec<(i32, i32)> {
        let mut result = Vec::new();
        if a.sides[0] == b.sides[2] {
            // A top matches B bottom
            result.push((0, 1));
        }
        if a.sides[2] == b.sides[0] {
            // A bottom matches B top
            result.push((0, -1));
        }
        if a.sides[1] == b.sides[3] {
            // A right matches B left
            result.push((1, 0));
        }
        if a.sides[3] == b.sides[1] {
            // A left matches B right
            result.push((-1, 0))
        }
        result
    }

    fn left_matches_right(a: &Option<Tile>, b: &Tile) -> bool {
        if let Some(left_tile) = a {
            left_tile.sides[1] == b.sides[3]
        } else {
            true
        }
    }

    fn top_matches_bottom(a: &Option<Tile>, b: &Tile) -> bool {
        if let Some(top_tile) = a {
            top_tile.sides[0] == b.sides[2]
        } else {
            true
        }
    }

    fn matches_neighbors(a: &Tile, left: &Option<Tile>, top: &Option<Tile>) -> bool {
        Puzzle::left_matches_right(left, a) && Puzzle::top_matches_bottom(top, a)
    }

    fn place_tile(
        &self,
        pos: usize,
        mut used: &mut Vec<bool>,
        mut solution: &mut Vec<Vec<Option<Tile>>>,
    ) -> Option<u64>
    {
        let side_len = (self.tiles.len() as f64).sqrt() as usize;
        let used_count = used.iter().filter(|x| **x).count();

        if used_count >= self.tiles.len() {
            let a = solution[0][0].as_ref().unwrap().tile_id;
            let b = solution[0][side_len-1].as_ref().unwrap().tile_id;
            let c = solution[side_len-1][side_len-1].as_ref().unwrap().tile_id;
            let d = solution[side_len-1][0].as_ref().unwrap().tile_id;
            println!("Found a solution: [{}, {}, {}, {}]", a, b, c, d);
            return Some(a*b*c*d);
        }

        let row = pos / side_len;
        let col = pos % side_len;

        //println!("Trying pos row={}, col={}", row, col);
        //println!("{} tiles used", used_count);

        for tile_pos in 0..self.tiles.len() {
            if used[tile_pos] {
                continue;
            }

            let left_tile = if col > 0 { solution[row][col-1].clone() } else { None };
            let top_tile  = if row > 0 { solution[row-1][col].clone() } else { None };
            let mut tile = self.tiles[tile_pos].clone();

            //println!("left = {:?}", left_tile);
            //println!("top = {:?}", top_tile);

            for flip_how in 0..3 {
                tile = tile.flip(flip_how);
                for _rotation in 0..4 {
                    if Puzzle::matches_neighbors(&tile, &left_tile, &top_tile) {
                        //println!("{}: Placed tile at row={}, col={}: {}", pos, row, col, tile.to_str());
                        used[tile_pos] = true;
                        solution[row][col] = Some(tile.clone());
                        if let Some(res) = self.place_tile(pos + 1, &mut used, &mut solution) {
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

    fn solve(&self) -> Option<u64> {
        let side_len = (self.tiles.len() as f64).sqrt() as usize;
        let mut used: Vec<bool> = (0..self.tiles.len()).map(|_| false).collect();
        let mut solution = Vec::new();
        for _ in 0..side_len {
            let v: Vec<_> = (0..side_len).map(|_| None).collect();
            solution.push(v);
        }

        self.place_tile(0, &mut used, &mut solution)
    }

}

fn test_tile_matches() {
    assert!(Puzzle::is_square(&vec![
            (-1,-1), (1, -1),
            (-1, 1), (1,  1),
    ]));
    assert!(!Puzzle::is_square(&vec![
            (-2,-1), (2, -1),
            (-2, 1), (2,  1),
    ]));

    let a = Tile::from_str(
"Tile 1:
##.
..#
#..");

    assert!(
        Puzzle::match_tiles(&a,
            &Tile::from_str(
"Tile 2:
...
...
..."),
    ).is_empty());

    assert_eq!(vec![(0, 1)],
        Puzzle::match_tiles(
            &a,
            &Tile::from_str(
"Tile 2:
...
...
##.")));

    assert_eq!(vec![(1, 0)],
        Puzzle::match_tiles(
            &a,
            &Tile::from_str(
"Tile 2:
...
#..
...")));

    assert_eq!(vec![(-1, 0)],
        Puzzle::match_tiles(
            &a,
            &Tile::from_str(
"Tile 2:
..#
...
..#")));

    assert_eq!(vec![(0, -1)],
        Puzzle::match_tiles(
            &a,
            &Tile::from_str(
"Tile 2:
#..
...
...")));

    assert_eq!(vec![(0, 1),(0,-1)],
        Puzzle::match_tiles(
            &a,
            &Tile::from_str(
"Tile 2:
#..
...
##.")));
}

fn test_rotation() {
    let a = Tile::from_str(
"Tile 1:
##.
#..
.##");

    let b = a.rotate();
    assert_eq!(vec![".##", "##.", "#..", ".##"], b.sides);
    let b = b.rotate();
    assert_eq!(vec!["##.", ".##", ".##", "#.."], b.sides);
    let b = b.rotate();
    assert_eq!(vec!["..#", "##.", "##.", ".##"], b.sides);
    let b = b.rotate();
    assert_eq!(a.sides, b.sides);
}

fn test_flips() {
    let a = Tile::from_str(
"Tile 1:
##..
#...
...#
#.##");

    let b = a.flip(1);
    assert_eq!(vec!["#.##", "##..", "##..", "#.##"], b.sides);
    let b = b.flip(1);
    assert_eq!(a.sides, b.sides);

    let b = a.flip(2);
    assert_eq!(vec!["..##", "##.#", "##.#", "..##"], b.sides);
    let b = b.flip(2);
    assert_eq!(a.sides, b.sides);
}

fn main() {
    let input_data = read_input_data();
    let mut tiles = Vec::new();
    for tile_str in input_data.split("\n\n") {
        tiles.push(Tile::from_str(tile_str));
    }

    test_tile_matches();
    test_rotation();
    test_flips();

    let puzzle = Puzzle { tiles };
    let answer = puzzle.solve();
    println!("Stage 1: answer = {:?}", answer);
}
