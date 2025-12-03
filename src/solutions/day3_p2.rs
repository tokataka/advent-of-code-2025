pub fn solution(lines: Vec<&str>) -> String {
    const MAX_BATTERIES: usize = 12;

    let mut result = 0;

    for line in &lines {
        let mut dp = [0; MAX_BATTERIES + 1];

        for ch in line.bytes() {
            let digit = (ch - b'0') as i64;

            for i in (0..MAX_BATTERIES).rev() {
                dp[i + 1] = dp[i + 1].max(dp[i] * 10 + digit);
            }
        }

        result += dp[MAX_BATTERIES];
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day3_p2() {
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

        assert_eq!(solution(lines), "3121910778619");
    }
}
