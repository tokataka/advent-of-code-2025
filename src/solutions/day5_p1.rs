use std::{cmp::Reverse, collections::BinaryHeap};

pub fn solution(lines: Vec<&str>) -> String {
    let mut s = lines.split(|line| line.trim().is_empty());

    let fresh_ranges: Vec<(i64, i64)> = s
        .next()
        .unwrap()
        .iter()
        .map(|id_range| {
            let (from, to) = id_range.split_once('-').unwrap();
            (from.parse().unwrap(), to.parse().unwrap())
        })
        .collect();

    let mut available_ids: Vec<i64> = s
        .next()
        .unwrap()
        .iter()
        .map(|id| id.parse().unwrap())
        .collect();

    available_ids.sort_unstable();

    let mut fresh_prefix = BinaryHeap::new();

    for &(from, to) in &fresh_ranges {
        fresh_prefix.push((Reverse(from), 1));
        fresh_prefix.push((Reverse(to + 1), -1));
    }

    let mut result = 0;
    let mut count = 0;

    for id in available_ids {
        while let Some(&(Reverse(x), d)) = fresh_prefix.peek() {
            if x > id {
                break;
            }

            fresh_prefix.pop();
            count += d;
        }

        if count > 0 {
            result += 1;
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day5_p1() {
        let lines = "
3-5
10-14
16-20
12-18

1
5
8
11
17
32
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "3");
    }
}
