pub fn solution(lines: Vec<&str>) -> String {
    let mut result = 0;
    let mut j = 0;

    for op in lines.last().unwrap().split_whitespace() {
        let mut numbers = vec![];

        while j < lines.iter().map(|line| line.len()).max().unwrap() {
            let mut number = 0;

            for line in &lines {
                if let Some(Ok(x)) = line.get(j..j + 1).map(|x| x.parse::<i64>()) {
                    number = number * 10 + x
                };
            }

            j += 1;

            if number == 0 {
                break;
            }

            numbers.push(number);
        }

        match op {
            "+" => result += numbers.iter().sum::<i64>(),
            "*" => result += numbers.iter().product::<i64>(),
            _ => unreachable!(),
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day6_p2() {
        let lines = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +"
            .split('\n')
            .collect();

        assert_eq!(solution(lines), "3263827");
    }
}
