pub fn solution(lines: Vec<&str>) -> String {
    let mut tachyons = vec![0; lines[0].len()];

    tachyons[lines[0].find('S').unwrap()] = 1_i64;

    for line in lines.iter().skip(1) {
        let mut next = tachyons.clone();

        for (i, ch) in line.char_indices() {
            if ch == '^' {
                next[i - 1] += tachyons[i];
                next[i] -= tachyons[i];
                next[i + 1] += tachyons[i];
            }
        }

        tachyons = next;
    }

    tachyons.iter().sum::<i64>().to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day7_p2() {
        let lines = "
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "40");
    }
}
