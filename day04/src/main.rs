struct Range {
    left: u64,
    right: u64,
}

impl Range {
    fn new(l: u64, r: u64) -> Self {
        Range { left: l, right: r }
    }
    fn contains_completelly(&self, other: &Range) -> bool {
        if self.left <= other.left && self.right >= other.right {
            return true;
        }
        false
    }

    fn contains_partially(&self, other: &Range) -> Option<u64> {
        // contains left
        if self.left <= other.left && self.right >= other.left {
            return Some(self.right - other.left + 1);
        }
        // contains right
        if self.left <= other.right && self.right >= other.right {
            return Some(other.right - self.left + 1);
        }
        None
    }

    // fn is_contained(&self, other: &Range) -> bool {
    //     other.contains(self)
    // }
}

fn main() -> Result<(), std::io::Error> {
    let mut count = 0;
    let mut count_partial = 0;
    // for line in include_str!("input_example.txt")
    for line in include_str!("input.txt").replace("\r\n", "\n").split('\n') {
        let pair: Vec<_> = line.split(',').collect();
        let r1: Vec<_> = pair[0]
            .split('-')
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        let r2: Vec<_> = pair[1]
            .split('-')
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        let range1 = Range::new(r1[0], r1[1]);
        let range2 = Range::new(r2[0], r2[1]);
        if range1.contains_completelly(&range2) || range2.contains_completelly(&range1) {
            count += 1;
        }
        // partially
        if range1.contains_partially(&range2).is_some()
            || range2.contains_partially(&range1).is_some()
        {
            count_partial += 1;
        }
    }
    dbg!(count);
    dbg!(count_partial);

    Ok(())
}
