use crate::helper_lib::{answer::Answer, solution::Solution};

pub struct Day12;

impl Solution for Day12 {
    fn part_a(&self, input: &[String]) -> Answer {
        let parsed = parse(input);
        parsed.count_arrangements().into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        todo!()
    }
}

struct Parsed {
    rows: Vec<Row>,
}

impl Parsed {
    pub fn count_arrangements(&self) -> usize {
        fn dfs(
            conditions: &Vec<SpringCondition>,
            group_sizes: &mut Vec<usize>,
            index: usize,
        ) -> usize {
            todo!();
        }
        let mut count = 0;
        for row in self.rows.iter() {
            count += dfs(&row.conditions, &mut row.group_sizes.clone(), 0);
        }
        count
    }
}

fn parse(input: &[String]) -> Parsed {
    let mut rows = vec![];
    for line in input {
        let (conditions, group_sized) = line.split_once(' ').unwrap();
        let group_sizes = group_sized
            .split(',')
            .map(|d| d.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let conditions = conditions
            .chars()
            .map(|c| SpringCondition::from(c))
            .collect::<Vec<_>>();
        rows.push(Row {
            conditions,
            group_sizes,
        })
    }
    Parsed { rows }
}

pub struct Row {
    conditions: Vec<SpringCondition>,
    group_sizes: Vec<usize>,
}

pub enum SpringCondition {
    Broken,
    Working,
    Unknown,
}

impl From<char> for SpringCondition {
    fn from(value: char) -> Self {
        match value {
            '.' => SpringCondition::Working,
            '#' => SpringCondition::Broken,
            '?' => SpringCondition::Unknown,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::helper_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day12;

    pub fn test_a() {
        let input =
            input::read_file(&format!("{}day_12_test.txt", helper_lib::FILES_PREFIX)).unwrap();
        let answer = Day12.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(21), answer);
    }

    pub fn test_b() {
        let input =
            input::read_file(&format!("{}day_12_test.txt", helper_lib::FILES_PREFIX)).unwrap();
        let answer = Day12.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(0), answer);
    }
}
