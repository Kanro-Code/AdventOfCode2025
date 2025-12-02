advent_of_code::solution!(1);

#[derive(Debug)]
pub struct Instruction {
    pub direction: Direction,
    pub value: isize,
}

impl Instruction {
    pub fn parse(s: &str) -> Self {
        let (dir, val) = s.split_at(1);
        let direction = match dir {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction"),
        };
        let value = val.parse::<isize>().expect("Invalid value");
        Instruction { direction, value }
    }

    pub fn apply(&self, position: isize) -> isize {
        match self.direction {
            Direction::Left => position - self.value,
            Direction::Right => position + self.value,
        }
    }
}

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

pub fn part_one(input: &str) -> Option<u64> {
    let instructions = input
        .lines()
        .map(Instruction::parse)
        .collect::<Vec<Instruction>>();

    let mut current_position = 50;
    let mut count = 0;

    for instruction in instructions {
        current_position = instruction.apply(current_position).rem_euclid(100);

        if current_position == 0 {
            count += 1;
        }
    }

    Some(count)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
