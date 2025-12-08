pub fn solution(lines: Vec<&str>) -> String {
    let n = lines.len();
    let m = lines[0].len();

    let mut result = 0;

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

    for i in 0..n {
        for j in 0..m {
            if &lines[i][j..j + 1] != "@" {
                continue;
            }

            let mut roll_count = 0;

            for (di, dj) in DIRECTIONS {
                let (ii, jj) = match (i.checked_add_signed(di), j.checked_add_signed(dj)) {
                    (Some(ii), Some(jj)) if ii < n && jj < m => (ii, jj),
                    _ => continue,
                };

                if &lines[ii][jj..jj + 1] == "@" {
                    roll_count += 1;
                }
            }

            if roll_count < 4 {
                result += 1;
            }
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day4_p1() {
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

        assert_eq!(solution(lines), "13");
    }
}
