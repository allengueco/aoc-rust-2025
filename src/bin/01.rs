advent_of_code::solution!(1);

const LIMIT: isize = 100;
pub fn part_one(input: &str) -> Option<u64> {
    let mut start: isize = 50;
    let mut result = 0;
    for (r, num) in input.split_whitespace().map(|x| x.split_at(1)) {
        match r {
            "L" => {
                start += num.parse::<isize>().unwrap();
                start %= LIMIT;
            }
            "R" => {
                start -= num.parse::<isize>().unwrap();
                start %= LIMIT;
            }
            _ => unimplemented!(),
        }
        if start == 0 {
            result += 1
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut start: isize = 50;
    let mut result = 0;
    for (r, num) in input.split_whitespace().map(|x| x.split_at(1)) {
        match r {
            "L" => {
                start += num.parse::<isize>().unwrap();
                if start >= LIMIT {
                    result += 1;
                }
                start %= LIMIT;
            }
            "R" => {
                start -= num.parse::<isize>().unwrap();
                if start <= 0 {
                    result += 1;
                }
                start %= LIMIT;
            }
            _ => unimplemented!(),
        }
    }
    Some(result)
}

enum Rotation {
    L(u8),
    R(u8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
