use std::collections::BTreeMap;

pub fn parse_range(text: &str) -> Option<(usize, usize)> {
    text.split_once('-').and_then(|(s, e)| {
        let start: Option<usize> = s.parse().ok();
        let end: Option<usize> = e.parse().ok();
        if let Some(start) = start
            && let Some(end) = end
        {
            return Some((start, end));
        }
        None
    })
}

pub struct Cafeteria {
    ranges: BTreeMap<usize, usize>,
}

impl Cafeteria {
    pub fn new() -> Self {
        Self {
            ranges: BTreeMap::new(),
        }
    }

    pub fn add_range(&mut self, range: (usize, usize)) {
        if let Some(end) = self.ranges.get(&range.0) {
            if range.1 > *end {
                self.ranges.insert(range.0, range.1);
            }
        } else {
            self.ranges.insert(range.0, range.1);
        }
    }

    pub fn is_fresh(&self, id: usize) -> bool {
        for range in self.ranges.iter() {
            if id >= *range.0 && id <= *range.1 {
                return true;
            }
        }

        false
    }

    pub fn count_range_ids(&self) -> usize {
        let mut count = 0;
        let mut last_end = 0;
        for range in self.ranges.iter() {
            let mut start = *range.0;
            let end = *range.1;
            if last_end >= start {
                start = last_end + 1;
            }
            if end < start {
                continue;
            }
            last_end = end;
            count += end - start + 1;
        }
        count
    }
}

impl Default for Cafeteria {
    fn default() -> Self {
        Self::new()
    }
}
