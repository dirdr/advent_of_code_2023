use crate::helper_lib::{answer::Answer, solution::Solution};

pub struct Day15;

impl Solution for Day15 {
    fn part_a(&self, input: &[String]) -> Answer {
        todo!()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::helper_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day15;

    pub fn test_a() {
        let input = input::read_file(&format!(
            "{}day_15_test.txt",
            helper_lib::consts::FILES_PREFIX
        ))
        .unwrap();
        let answer = Day15.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(136), answer);
    }

    pub fn test_b() {
        let input = input::read_file(&format!(
            "{}day_15_test.txt",
            helper_lib::consts::FILES_PREFIX
        ))
        .unwrap();
        let answer = Day15.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(64), answer);
    }
}
