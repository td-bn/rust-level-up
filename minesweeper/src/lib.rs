struct MineField {
    grid: Vec< Vec<char>>,
    rows: usize,
    cols: usize
}

impl MineField {
    fn annotate_field(&self) -> Vec<String> {
        self.grid.iter().enumerate().map(|(i, r)| self.annotate_row(r, i)).collect()
    }

    fn annotate_row(&self, r: &Vec<char>, i: usize) -> String {
        r.iter().enumerate().map(|(j,_)| self.mine_count(i, j)).collect()
    }

    fn mine_count(&self, i: usize, j: usize) -> char {
        if self.grid[i][j] == '*' {return '*'};

        let mut count: u32 = 0;
        
        for ii in self.indices(i, self.rows) {
            for jj in self.indices(j, self.cols) {
                count += self.mine(ii, jj);
            }
        }
        if count == 0 {' '}
        else {std::char::from_digit(count, 10).unwrap()}
    }

    fn mine(&self, i: usize, j: usize) -> u32 {
        if self.grid[i][j] == '*' {1} else {0}
    }

    fn indices(&self, x: usize, max: usize) -> Vec<usize> {
        let mut indices = vec![x];

        if x>0 {indices.push(x-1)};
        if x+1<max {indices.push(x+1)};

        indices
    }
 }

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return Vec::new();
    }
    let field = MineField{
        grid: minefield.iter().map(|x| x.chars().collect()).collect(),
        rows: minefield.len(),
        cols: minefield[0].len()
    };

    field.annotate_field()
}
