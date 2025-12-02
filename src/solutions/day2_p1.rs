pub fn solution(lines: Vec<&str>) -> String {
    let mut result = 0;

    for split in lines.join("").split(',') {
        let (a, b) = split.trim().split_once('-').unwrap();

        let a = a.parse::<i64>().unwrap();
        let b = b.parse::<i64>().unwrap();

        let left_a = match a.ilog10() + 1 {
            len_a if len_a % 2 == 0 => a / 10i64.pow(len_a / 2),
            len_a => 10i64.pow(len_a / 2),
        };

        let left_b = match b.ilog10() + 1 {
            len_b if len_b % 2 == 0 => b / 10i64.pow(len_b / 2),
            len_b => 10i64.pow(len_b / 2),
        };

        for left in left_a..=left_b {
            let cur = left * (1 + 10i64.pow(left.ilog10() + 1));

            if cur >= a && cur <= b {
                result += cur;
            }
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day2_p1() {
        let lines = "
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "1227775554");
    }
}
