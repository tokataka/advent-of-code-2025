pub fn solution(lines: Vec<&str>) -> String {
    let mut cur = 50;
    let mut result = 0;

    for line in lines {
        let (direction, count) = line.split_at(1);
        let count = count.parse::<i32>().unwrap();

        match direction {
            "R" => {
                result += (cur + count) / 100;
                cur = (cur + count) % 100;
            }
            "L" => {
                result += ((100 - cur) % 100 + count) / 100;
                cur = (cur + 100 - count % 100) % 100;
            }
            _ => (),
        };
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day1_p2() {
        let lines = "
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "6");
    }
}
