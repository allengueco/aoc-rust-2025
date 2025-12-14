advent_of_code::solution!(2);

struct IdRange {
    start: u64,
    end: u64,
}

impl IdRange {
    fn from_input(input: &str) -> Self {
        let mut s = input.split('-');
        let l = s.next().unwrap().parse::<u64>().unwrap();
        let r = s.next().unwrap().parse::<u64>().unwrap();

        IdRange { start: l, end: r }
    }

    fn get_invalid_ranges(&self) -> Vec<u64> {
        let mut result: Vec<u64> = vec![];
        for r in self.start..=self.end {
            if Self::is_invalid(r) {
                result.push(r);
            }
        }
        result
    }

    fn get_invalid_ranges_part_2(&self) -> Vec<u64> {
        let mut result: Vec<u64> = vec![];
        for r in self.start..=self.end {
            if Self::is_invalid(r) || Self::repeats_at_least_2(r) {
                result.push(r);
            }
        }

        if result.len() < 2 {
            return vec![];
        }

        result
    }


    fn is_invalid(id: u64) -> bool {
        let id = id.to_string();

        if !id.len().is_multiple_of(2) {
            return false;
        }

        let half_index = id.len() / 2;

        let (first, second) = id.split_at(half_index);

        first == second
    }

    fn repeats_at_least_2(id: u64) -> bool {
        let id = id.to_string();

        let mut section = String::new();

        for c in id.chars() {

        }
        let half_index = id.len() / 2;

        let (first, second) = id.split_at(half_index);

        first == second
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let result = input
        .trim()
        .split(',')
        .map(IdRange::from_input)
        .flat_map(|range| range.get_invalid_ranges())
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let result = input
        .trim()
        .split(',')
        .map(IdRange::from_input)
        .flat_map(|range| range.get_invalid_ranges_part_2())
        .sum();

    Some(result)
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
