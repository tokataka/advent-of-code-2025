pub fn solution(lines: Vec<&str>) -> String {
    let ops = lines.last().unwrap().split_whitespace().collect::<Vec<_>>();

    let mut result = ops
        .iter()
        .map(|&op| match op {
            "+" => 0,
            "*" => 1,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    for line in lines.iter().take(lines.len() - 1) {
        for (i, x) in line.split_whitespace().enumerate() {
            let x = x.parse::<i64>().unwrap();
            match ops[i] {
                "+" => result[i] += x,
                "*" => result[i] *= x,
                _ => unreachable!(),
            }
        }
    }

    result.iter().sum::<i64>().to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day6_p1() {
        let lines = "
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "4277556");
    }
}
