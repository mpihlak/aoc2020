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
#[derive(Debug)]
struct Tile {
    sides: Vec<String>,
    tile_id: u32,
}

impl Tile {
    
    fn from_str(tile_str: &str) -> Self {
        let mut tile_id_data = tile_str.split(":\n");
        let tile_id: u32 = tile_id_data.next().unwrap()
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

    fn to_str(&self) -> String {
        let w = self.sides[0].len();
        let h = w;
        println!("h = {}, w = {}", h, w);
        let mut res = String::new();

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

    // Flip: 0 - upside down, 1 - left right
    fn flip(&self, how: usize) -> Tile {
        let sides = if how == 0 {
            vec![
                self.sides[2].clone(),
                self.sides[1].chars().rev().collect::<String>(),
                self.sides[0].clone(),
                self.sides[3].chars().rev().collect::<String>(),
            ]
        } else {
            vec![
                self.sides[0].chars().rev().collect::<String>(),
                self.sides[3].clone(),
                self.sides[2].chars().rev().collect::<String>(),
                self.sides[1].clone(),
            ]
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
        //println!("matching:\na={:?}\nb={:?}\n", a, b);
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

    fn solve(&self) -> u64 {
        for _rotation in 0..4 {
            for _flip in 0..2 {
            }
        }
        0
    }

    #[allow(dead_code)]
    fn solve_aux(_solution: Vec<(i32,i32)>) -> u64 {
        for _rotation in 0..4 {
            for _flip in 0..2 {
            }
        }
        0
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

    let b = a.flip(0);
    assert_eq!(vec!["#.##", "##..", "##..", "#.##"], b.sides);
    let b = b.flip(0);
    assert_eq!(a.sides, b.sides);

    let b = a.flip(1);
    assert_eq!(vec!["..##", "##.#", "##.#", "..##"], b.sides);
    let b = b.flip(1);
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
    println!("Stage 1: answer = {}", answer);

    println!("Tile 0:\n{}", puzzle.tiles[0].to_str());
}
