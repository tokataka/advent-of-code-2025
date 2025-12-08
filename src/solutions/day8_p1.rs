struct UnionFind {
    data: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            data: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, a: usize) -> usize {
        if self.data[a] == a {
            return a;
        }

        self.data[a] = self.find(self.data[a]);
        self.data[a]
    }

    fn union(&mut self, a: usize, b: usize) {
        let a = self.find(a);
        let b = self.find(b);

        if a == b {
            return;
        }

        let (a, b) = match self.size[a] < self.size[b] {
            true => (a, b),
            false => (b, a),
        };

        self.data[a] = b;
        self.size[b] += self.size[a];
    }
}

pub fn solution(lines: Vec<&str>) -> String {
    let n = lines.len();

    let lines = lines
        .iter()
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut distances = vec![];

    for (i, line1) in lines.iter().enumerate() {
        for (j, line2) in lines.iter().enumerate().skip(i + 1) {
            let d = line1
                .iter()
                .zip(line2)
                .map(|(&a1, &a2)| (a1 - a2) * (a1 - a2))
                .sum::<i64>();

            distances.push((d, i, j));
        }
    }

    distances.sort_unstable();

    let mut uf = UnionFind::new(n);

    for &(_, i, j) in distances.iter().take(1000 /* 10 to pass unit test */) {
        uf.union(i, j);
    }

    let mut circuit_sizes = vec![];

    for i in 0..n {
        if i == uf.find(i) {
            circuit_sizes.push(uf.size[i]);
        }
    }

    circuit_sizes.sort_unstable();

    circuit_sizes
        .iter()
        .rev()
        .take(3)
        .product::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day8_p1() {
        let lines = "
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "40");
    }
}
