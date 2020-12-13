use std::fs;

pub fn argv1_or_default(default_value: &str) -> String {
    let mut args = std::env::args();
    let _ = args.next();
    match args.next() {
        Some(name) => name,
        _ => default_value.to_owned(),
    }
}

pub fn read_input_data() -> String {
    let filename = argv1_or_default("input.txt");
    fs::read_to_string(filename).unwrap().trim_end().to_string()
}

#[derive(Clone)]
pub struct Grid {
    pub cells:  Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
}

impl Grid {

    pub fn from_str(s: &str) -> Self {
        let mut cells: Vec<Vec<char>> = Vec::new();

        for row in s.trim_end().split("\n") {
            let cols: Vec<_> = row.chars().collect();
            cells.push(cols);
        }

        let width = if cells.len() > 0 { cells[0].len() } else { 0 };
        let height = cells.len();

        Grid {
            cells,
            width,
            height,
        }
    }

    pub fn to_str(&self) -> String {
        let mut res = String::new();
        for row in self.cells.iter() {
            for col in row.iter() {
                res.push(*col);
            }
            res.push('\n');
        }
        res
    }

    pub fn at(&self, row: i32, col: i32) -> Option<char> {
        if row < 0 || row >= self.height as i32 || col < 0 || col >= self.width as i32 {
            None
        } else {
            Some(self.cells[row as usize][col as usize])
        }
    }

    pub fn count_elems(&self, c: char) -> u32 {
        let mut count = 0;

        for row in self.cells.iter() {
            for col in row.iter() {
                if *col == c {
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
