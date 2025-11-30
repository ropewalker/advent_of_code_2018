use aoc_runner_derive::aoc;

#[aoc(day20, part1)]
fn part1(regex: &str) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: &str = "^WNE$";
    static TEST_INPUT_2: &str = "^ENWWW(NEEE|SSE(EE|N))$";
    static TEST_INPUT_3: &str = "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$";
    static TEST_INPUT_4: &str = "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$";
    static TEST_INPUT_5: &str = "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$";

    #[test]
    fn part1_example1() {
        assert_eq!(part1(TEST_INPUT_1), 3);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1(TEST_INPUT_2), 10);
    }

    #[test]
    fn part1_example3() {
        assert_eq!(part1(TEST_INPUT_3), 18);
    }

    #[test]
    fn part1_example4() {
        assert_eq!(part1(TEST_INPUT_4), 23);
    }

    #[test]
    fn part1_example5() {
        assert_eq!(part1(TEST_INPUT_5), 31);
    }
}
