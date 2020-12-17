use aoclib::*;
use std::collections::HashMap;

struct ConwayCubes3d {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32,
    cells: HashMap<(i32,i32,i32), i32>,
}

impl ConwayCubes3d {

    #[allow(dead_code)]
    fn display(&self) {
        println!("Xmin = {}, Xmax = {}", self.x_min, self.x_max);
        println!("Ymin = {}, Ymax = {}", self.y_min, self.y_max);
        println!("Zmin = {}, Zmax = {}", self.z_min, self.z_max);

        for z in self.z_min .. self.z_max {
            println!("z={}", z);

            for y in self.y_min .. self.y_max {
                for x in self.x_min .. self.x_max {
                    let coords = (x, y, z);
                    match self.cells.get(&coords) {
                        Some(0) | None => print!("."),
                        Some(1) => print!("#"),
                        Some(x) => panic!("invalid value: {}", x),
                    }
                }
                println!();
            }
            println!();
        }
        println!();
    }

    fn count_active_neighbors(&self, at_x: i32, at_y: i32, at_z: i32) -> i32 {
        let mut res = 0;

        //println!("checking {},{},{}", at_x, at_y, at_z);
        for z in at_z-1 .. at_z+2 {
            for x in at_x-1 .. at_x+2 {
                for y in at_y-1 .. at_y+2 {
                    if !(x == at_x && y == at_y && z == at_z) {
                        let coords = (x, y, z);
                        let v = *self.cells.get(&coords).unwrap_or(&0);
                        //println!("v at {},{},{} = {}", x,y,z, v);
                        res += v;
                    }
                }
            }
        }
        res
    }

    fn iterate(&mut self) {
        let mut new_cells = HashMap::new();

        for x in self.x_min-1 .. self.x_max+1 {
            for y in self.y_min-1 .. self.y_max+1 {
                for z in self.z_min-1 .. self.z_max+1 {
                    let coords = (x, y, z);
                    let neighbors = self.count_active_neighbors(x, y, z);
                    let old_me = *self.cells.get(&coords).unwrap_or(&0);
                    let new_me = new_cells.entry(coords).or_insert(old_me);

                    if old_me != 0 {
                        if neighbors < 2 || neighbors > 3 {
                            *new_me = 0;
                        }
                    } else if neighbors == 3 {
                        *new_me = 1;
                        self.x_min = self.x_min.min(x);
                        self.x_max = self.x_max.max(x+1);
                        self.y_min = self.y_min.min(y);
                        self.y_max = self.y_max.max(y+1);
                        self.z_min = self.z_min.min(z);
                        self.z_max = self.z_max.max(z+1);
                    }
                }
            }
        }
        self.cells = new_cells;
    }

}

struct ConwayCubes4d {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32,
    w_min: i32,
    w_max: i32,
    cells: HashMap<(i32,i32,i32,i32), i32>,
}

impl ConwayCubes4d {

    #[allow(dead_code)]
    fn display(&self) {
        println!("Xmin = {}, Xmax = {}", self.x_min, self.x_max);
        println!("Ymin = {}, Ymax = {}", self.y_min, self.y_max);
        println!("Zmin = {}, Zmax = {}", self.z_min, self.z_max);
        println!("Wmin = {}, Wmax = {}", self.w_min, self.w_max);

        for w in self.w_min .. self.w_max {
            for z in self.z_min .. self.z_max {
                println!("z={}, w={}", z, w);
                for y in self.y_min .. self.y_max {
                    for x in self.x_min .. self.x_max {
                        let coords = (x, y, z, w);
                        match self.cells.get(&coords) {
                            Some(0) | None => print!("."),
                            Some(1) => print!("#"),
                            Some(x) => panic!("invalid value: {}", x),
                        }
                    }
                    println!();
                }
                println!();
            }
            println!();
        }
        println!();
    }

    fn count_active_neighbors(&self, at_x: i32, at_y: i32, at_z: i32, at_w: i32) -> i32 {
        let mut res = 0;

        //println!("checking {},{},{}", at_x, at_y, at_z);
        for w in at_w-1 .. at_w+2 {
            for z in at_z-1 .. at_z+2 {
                for x in at_x-1 .. at_x+2 {
                    for y in at_y-1 .. at_y+2 {
                        if !(x == at_x && y == at_y && z == at_z && w == at_w) {
                            let coords = (x, y, z, w);
                            let v = *self.cells.get(&coords).unwrap_or(&0);
                            //println!("v at {},{},{} = {}", x,y,z, v);
                            res += v;
                        }
                    }
                }
            }
        }
        res
    }

    fn iterate(&mut self) {
        let mut new_cells = HashMap::new();

        for x in self.x_min-1 .. self.x_max+1 {
            for y in self.y_min-1 .. self.y_max+1 {
                for z in self.z_min-1 .. self.z_max+1 {
                    for w in self.w_min-1 .. self.w_max+1 {
                        let coords = (x, y, z, w);
                        let neighbors = self.count_active_neighbors(x, y, z, w);
                        let old_me = *self.cells.get(&coords).unwrap_or(&0);
                        let new_me = new_cells.entry(coords).or_insert(old_me);

                        if old_me != 0 {
                            if neighbors < 2 || neighbors > 3 {
                                *new_me = 0;
                            }
                        } else if neighbors == 3 {
                            *new_me = 1;
                            self.x_min = self.x_min.min(x);
                            self.x_max = self.x_max.max(x+1);
                            self.y_min = self.y_min.min(y);
                            self.y_max = self.y_max.max(y+1);
                            self.z_min = self.z_min.min(z);
                            self.z_max = self.z_max.max(z+1);
                            self.w_min = self.w_min.min(w);
                            self.w_max = self.w_max.max(w+1);
                        }
                    }
                }
            }
        }
        self.cells = new_cells;
    }

}

fn main() {
    let input_data = read_input_data();

    let mut x_dim = 0;
    let mut y_dim = 0;
    let mut cells: HashMap<(i32,i32,i32), i32> = HashMap::new();

    for line in input_data.split('\n') {
        for (x_pos, c) in line.chars().enumerate() {
            let coords = (x_pos as i32, y_dim, 0);
            let v = if c == '#' { 1 } else { 0 };
            cells.insert(coords, v);
            x_dim = x_dim.max(x_pos + 1);
        }
        y_dim += 1;
    }

    let mut conways = ConwayCubes3d {
        x_min: 0,
        x_max: x_dim as i32,
        y_min: 0,
        y_max: y_dim as i32,
        z_min: 0,
        z_max: 1,
        cells,
    };

    for _ in 0..6 {
        conways.iterate();
    }

    let count: i32 = conways.cells.values().sum();
    println!("Stage 1: answer = {}", count);

    let mut cells: HashMap<(i32,i32,i32,i32), i32> = HashMap::new();

    for (y_pos, line) in input_data.split('\n').enumerate() {
        for (x_pos, c) in line.chars().enumerate() {
            let coords = (x_pos as i32, y_pos as i32, 0, 0);
            let v = if c == '#' { 1 } else { 0 };
            cells.insert(coords, v);
        }
    }

    let mut conways = ConwayCubes4d {
        x_min: 0,
        x_max: x_dim as i32,
        y_min: 0,
        y_max: y_dim as i32,
        z_min: 0,
        z_max: 1,
        w_min: 0,
        w_max: 1,
        cells,
    };

    for _ in 0..6 {
        conways.iterate();
    }

    let count: i32 = conways.cells.values().sum();
    println!("Stage 2: answer = {}", count);
}
