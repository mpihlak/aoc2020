pub fn argv1_or_default(default_value: &str) -> String {
    let mut args = std::env::args();
    let _ = args.next();
    match args.next() {
        Some(name) => name,
        _ => default_value.to_owned(),
    }
}

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

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
