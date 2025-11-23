use aoc_runner_derive::aoc;

#[aoc(day14, part1)]
fn part1(input: &str) -> String {
    let num_recipes: usize = input.parse().unwrap();

    let mut scoreboard = vec![3usize, 7];
    let mut first_elf_recipe = 0usize;
    let mut second_elf_recipe = 1usize;

    let mut ten_recipes = Vec::with_capacity(10);

    while scoreboard.len() <= num_recipes + 10 {
        let new_recipes = scoreboard[first_elf_recipe] + scoreboard[second_elf_recipe];

        if new_recipes / 10 > 0 {
            if scoreboard.len() >= num_recipes && ten_recipes.len() < 10 {
                ten_recipes.push(new_recipes / 10);
            }

            scoreboard.push(new_recipes / 10);
        }

        if scoreboard.len() >= num_recipes && ten_recipes.len() < 10 {
            ten_recipes.push(new_recipes % 10);
        }

        scoreboard.push(new_recipes % 10);

        first_elf_recipe = (first_elf_recipe + 1 + scoreboard[first_elf_recipe]) % scoreboard.len();
        second_elf_recipe =
            (second_elf_recipe + 1 + scoreboard[second_elf_recipe]) % scoreboard.len();
    }

    let mut result = String::with_capacity(10);

    for recipe in ten_recipes {
        result.push(char::from_digit(recipe as u32, 10).unwrap());
    }

    result
}

#[aoc(day14, part2)]
fn part2(input: &str) -> usize {
    let score_sequence: Vec<usize> = input
        .chars()
        .map(|char| char.to_digit(10).unwrap() as usize)
        .collect();

    let mut scoreboard = vec![3usize, 7];
    let mut first_elf_recipe = 0usize;
    let mut second_elf_recipe = 1usize;

    loop {
        let new_recipes = scoreboard[first_elf_recipe] + scoreboard[second_elf_recipe];

        if new_recipes / 10 > 0 {
            scoreboard.push(new_recipes / 10);
        }

        scoreboard.push(new_recipes % 10);

        first_elf_recipe = (first_elf_recipe + 1 + scoreboard[first_elf_recipe]) % scoreboard.len();
        second_elf_recipe =
            (second_elf_recipe + 1 + scoreboard[second_elf_recipe]) % scoreboard.len();

        if scoreboard.len() >= score_sequence.len()
            && scoreboard[scoreboard.len() - score_sequence.len()..] == score_sequence
        {
            return scoreboard.len() - score_sequence.len();
        }

        if scoreboard.len() > score_sequence.len()
            && scoreboard[scoreboard.len() - score_sequence.len() - 1..scoreboard.len() - 1]
                == score_sequence
        {
            return scoreboard.len() - score_sequence.len() - 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: &str = "9";
    static TEST_INPUT_2: &str = "5";
    static TEST_INPUT_3: &str = "18";
    static TEST_INPUT_4: &str = "2018";
    static TEST_INPUT_5: &str = "51589";
    static TEST_INPUT_6: &str = "01245";
    static TEST_INPUT_7: &str = "92510";
    static TEST_INPUT_8: &str = "59414";

    #[test]
    fn part1_example1() {
        assert_eq!(part1(TEST_INPUT_1), "5158916779");
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1(TEST_INPUT_2), "0124515891");
    }

    #[test]
    fn part1_example3() {
        assert_eq!(part1(TEST_INPUT_3), "9251071085");
    }

    #[test]
    fn part1_example4() {
        assert_eq!(part1(TEST_INPUT_4), "5941429882");
    }

    #[test]
    fn part2_example1() {
        assert_eq!(part2(TEST_INPUT_5), 9);
    }

    #[test]
    fn part2_example2() {
        assert_eq!(part2(TEST_INPUT_6), 5);
    }

    #[test]
    fn part2_example3() {
        assert_eq!(part2(TEST_INPUT_7), 18);
    }

    #[test]
    fn part2_example4() {
        assert_eq!(part2(TEST_INPUT_8), 2018);
    }
}
