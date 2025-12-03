pub fn solution(lines: Vec<&str>) -> String {
    let mut result = 0;

    for line in &lines {
        let mut max_joltage = 0;
        let mut max_digit = 0;

        for ch in line.bytes() {
            let digit = (ch - b'0') as i32;
            max_joltage = max_joltage.max(max_digit * 10 + digit);
            max_digit = max_digit.max(digit);
        }

        result += max_joltage;
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day3_p1() {
        let lines = "
987654321111111
811111111111119
234234234234278
818181911112111
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "357");
    }
}
