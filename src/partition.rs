// Generate ORDERED partitions

pub struct PartitionGenerator {
    cur: Vec<usize>,
    exhausted: bool,
}

impl Iterator for PartitionGenerator {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted {
            return None;
        }

        if self.cur.len() == 0 {
            self.exhausted = true;
            return Some(self.cur.clone());
        }

        let c = self.cur.clone();
        let cl = self.cur.len() - 1;

        let mut n = cl;
        while self.cur[n] == 1 && n > 0 {
            n -= 1;
        }

        if n == 0 && !self.exhausted {
            self.exhausted = true;
            return Some(self.cur.clone());
        }

        self.cur[n-1] += 1;
        let ol = self.cur[n] - 1;
        self.cur[n] = 1;
        self.cur[cl] = ol;


        Some(c)
    }
}

impl PartitionGenerator {
    pub fn new(n: usize, l: usize) -> Self {
        if l == 0 && n == 0 { return PartitionGenerator { cur: vec![], exhausted: false } }
        if l == 0 { return PartitionGenerator { cur: vec![], exhausted: true } }
        if n < l { return PartitionGenerator { cur: vec![], exhausted: true }}

        let mut cur = vec![1; l];
        let cl = cur.len() - 1;
        cur[cl] += n - l;

        PartitionGenerator { cur, exhausted: false }
    }
}

pub fn num_partitions(n: usize, l: usize) -> usize {
    if l <= 1 || l == n { return 1; }
    if l == 2 { return n-1; }

    let mut sum = 0;
    for i in 1..=(n-l+1) {
        sum += num_partitions(n-i, l-1);
    }

    sum
}

pub fn num_partitions_fast(n: usize, l: usize) -> usize {
    let mut p = 1;
    for i in 1..l {
        p *= n-i;
        p /= i;
    }

    p
}