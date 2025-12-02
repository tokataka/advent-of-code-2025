use std::collections::HashSet;

pub fn solution(lines: Vec<&str>) -> String {
    let mut result = 0;
    let mut visited = HashSet::new();

    for split in lines[0].split(',') {
        let (a, b) = split.trim().split_once('-').unwrap();

        let a = a.parse::<i64>().unwrap();
        let b = b.parse::<i64>().unwrap();

        for repeat in 2..=b.ilog10() + 1 {
            let left_a = match a.ilog10() + 1 {
                len if len % repeat == 0 => a / 10i64.pow(len / repeat * (repeat - 1)),
                len => 10i64.pow(len / repeat),
            };

            let left_b = match b.ilog10() + 1 {
                len if len % repeat == 0 => b / 10i64.pow(len / repeat * (repeat - 1)),
                len => 10i64.pow(len / repeat),
            };

            for left in left_a..=left_b {
                let mut cur = 0;

                for _ in 0..repeat {
                    cur = (cur * 10i64.pow(left.ilog10() + 1)) + left;
                }

                if cur >= a && cur <= b && visited.insert(cur) {
                    result += cur;
                }
            }
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day2_p2() {
        let lines = "
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "4174379265");
    }
}
