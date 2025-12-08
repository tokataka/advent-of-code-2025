pub fn solution(lines: Vec<&str>) -> String {
    let mut result = 0;
    let mut tachyons = vec![false; lines[0].len()];

    tachyons[lines[0].find('S').unwrap()] = true;

    for line in lines.iter().skip(1) {
        let mut next = tachyons.clone();

        for (i, ch) in line.char_indices() {
            if ch == '^' && tachyons[i] {
                next[i - 1] = true;
                next[i] = false;
                next[i + 1] = true;

                result += 1;
            }
        }

        tachyons = next;
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day7_p1() {
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

        assert_eq!(solution(lines), "21");
    }
}
