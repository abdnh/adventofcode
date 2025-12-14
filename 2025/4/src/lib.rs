use std::collections::HashSet;

pub struct PaperGrid {
    pub n: usize,
    grid: Vec<bool>,
}

impl PaperGrid {
    pub fn new(n: usize) -> Self {
        Self { n, grid: vec![] }
    }

    pub fn add_row<I>(&mut self, row: I)
    where
        I: IntoIterator<Item = bool>,
    {
        self.grid.extend(row);
    }

    pub fn get_accessible_rolls(&mut self) -> Vec<usize> {
        let mut accessible_rolls: Vec<usize> = vec![];
        let n = self.n;
        for i in 0..self.grid.len() {
            if !self.grid[i] {
                continue;
            }
            let nearest_n = (i / n) * n;

            let mut positions = HashSet::new();
            positions.insert(i + n);
            positions.insert(i.saturating_sub(n));
            if i != nearest_n {
                positions.insert(i.saturating_sub(n + 1));
                positions.insert(i + n - 1);
            }
            if i != nearest_n + n - 1 {
                positions.insert(i.saturating_sub(n) + 1);
                positions.insert(i + n + 1);
            }
            if i % n != 0 {
                positions.insert(i - 1);
            }
            if i % n != n - 1 {
                positions.insert(i + 1);
            }
            positions.remove(&i);
            let rolls_count = positions
                .into_iter()
                .filter(|p| {
                    if *p < self.grid.len() {
                        return self.grid[*p];
                    }
                    false
                })
                .count();
            if rolls_count < 4 {
                accessible_rolls.push(i);
            }
        }

        accessible_rolls
    }

    pub fn remove_accessible_rolls(&mut self) -> usize {
        let mut removed = 0;
        loop {
            let rolls = self.get_accessible_rolls();
            if rolls.is_empty() {
                break;
            }
            removed += rolls.len();
            for roll in rolls {
                self.grid[roll] = false;
            }
        }

        removed
    }
}
