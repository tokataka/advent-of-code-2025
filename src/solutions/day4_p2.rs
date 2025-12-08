pub fn solution(lines: Vec<&str>) -> String {
    let n = lines.len();
    let m = lines[0].len();

    let mut result = 0;

    let mut grid = lines
        .into_iter()
        .map(|x| x.as_bytes().to_vec())
        .collect::<Vec<_>>();

    const DIRECTIONS: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    loop {
        let mut removed = vec![];

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == b'.' {
                    continue;
                }

                let mut roll_count = 0;

                for (di, dj) in DIRECTIONS {
                    let (ii, jj) = match (i.checked_add_signed(di), j.checked_add_signed(dj)) {
                        (Some(ii), Some(jj)) if ii < n && jj < m => (ii, jj),
                        _ => continue,
                    };

                    if grid[ii][jj] == b'@' {
                        roll_count += 1;
                    }
                }

                if roll_count < 4 {
                    result += 1;
                    removed.push((i, j));
                }
            }
        }

        if removed.is_empty() {
            break;
        }

        for (i, j) in removed {
            grid[i][j] = b'.';
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day4_p2() {
        let lines = "
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "43");
    }
}
