use winnow::{
    ascii::{dec_int, dec_uint, space0, space1},
    combinator::{alt, delimited, separated, separated_pair},
    prelude::*,
};

fn main() {
    let input = std::fs::read_to_string("inputs/day2").unwrap();
    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}

fn solve_part1(input: &str) -> usize {
    let limits = [12, 13, 14];
    input
        .lines()
        .filter_map(|line| game_parser.parse(line).ok())
        .filter_map(|(id, game)| game.validate(limits).then_some(id))
        .sum()
}

fn solve_part2(input: &str) -> i32 {
    input
        .lines()
        .filter_map(|line| game_parser.parse(line).ok())
        .map(|(_, game)| {
            let [r, g, b] = game.minimal_set();
            r * g * b
        })
        .sum()
}

#[derive(Debug, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Game {
    draws: Vec<[i32; 3]>,
}

impl Game {
    fn validate(&self, limits: [i32; 3]) -> bool {
        let [r_limit, g_limit, b_limit] = limits;
        self.draws
            .iter()
            .all(|&[r, g, b]| r <= r_limit && g <= g_limit && b <= b_limit)
    }

    fn minimal_set(&self) -> [i32; 3] {
        let mut amounts = [0, 0, 0];
        for draw in &self.draws {
            amounts[Color::Red as usize] =
                amounts[Color::Red as usize].max(draw[Color::Red as usize]);
            amounts[Color::Green as usize] =
                amounts[Color::Green as usize].max(draw[Color::Green as usize]);
            amounts[Color::Blue as usize] =
                amounts[Color::Blue as usize].max(draw[Color::Blue as usize]);
        }
        amounts
    }
}

fn color_parser(i: &mut &str) -> PResult<Color> {
    alt((
        "red".value(Color::Red),
        "green".value(Color::Green),
        "blue".value(Color::Blue),
    ))
    .parse_next(i)
}

fn draw_parser(i: &mut &str) -> PResult<[i32; 3]> {
    let mut v = [0, 0, 0];
    separated(
        1..,
        separated_pair(dec_int, space1, color_parser)
            .map(|(amount, color)| v[color as usize] = amount),
        (',', space0),
    )
    .parse_next(i)?;
    Ok(v)
}

fn game_parser(i: &mut &str) -> PResult<(usize, Game)> {
    let idx = delimited("Game ", dec_uint, (':', space0)).parse_next(i)?;
    let draws = separated(1.., draw_parser, (';', space0)).parse_next(i)?;
    Ok((idx, Game { draws }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_game() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected = Game {
            draws: vec![[4, 0, 3], [1, 2, 6], [0, 2, 0]],
        };
        assert_eq!(game_parser.parse_peek(input), Ok(("", (1, expected))))
    }

    const INPUT: &str = "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part1() {
        assert_eq!(solve_part1(INPUT), 8);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(INPUT), 2286);
    }
}
