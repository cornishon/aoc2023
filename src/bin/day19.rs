use std::ops::Range;

use fxhash::FxHashMap;
use winnow::{
    ascii::{dec_int, newline},
    combinator::{alt, delimited, opt, repeat, separated, separated_pair, seq, terminated},
    token::take_while,
    PResult, Parser,
};

fn main() {
    let input = std::fs::read_to_string("inputs/day19").unwrap();
    let (ws, rs) = input.split_once("\n\n").unwrap();
    let ratings = parse_ratings(rs);
    let bounds = compute_bounds(&parse_workflows(ws));
    println!("Part 1: {}", solve_part1(&bounds, &ratings));
    println!("Part 2: {}", solve_part2(&bounds));
}

fn solve_part1(bounds: &[Rating<Range<i32>>], ratings: &[Rating<i32>]) -> i32 {
    ratings
        .iter()
        .filter(|&r| bounds.iter().any(|b| b.contains(r)))
        .map(|r| r.score())
        .sum()
}

fn solve_part2(bounds: &[Rating<Range<i32>>]) -> usize {
    bounds.iter().map(|b| b.num_combinations()).sum()
}

type Workflow<'a> = FxHashMap<&'a str, Vec<Rule<'a>>>;

fn parse_workflows(ws: &str) -> FxHashMap<&str, Vec<Rule<'_>>> {
    repeat(1.., terminated(entry_parser, opt(newline)))
        .parse(ws.trim())
        .map_err(|e| eprintln!("{e}"))
        .unwrap()
}

#[derive(Debug, Clone, Copy)]
struct Rating<T>([T; 4]);

fn parse_ratings(rs: &str) -> Vec<Rating<i32>> {
    repeat(1.., terminated(rating_parser, opt(newline)))
        .parse(rs.trim())
        .map_err(|e| eprintln!("{e}"))
        .unwrap()
}

fn compute_bounds(workflows: &Workflow<'_>) -> Vec<Rating<Range<i32>>> {
    let mut result = Vec::new();
    recur(
        workflows,
        &mut result,
        &mut workflows["in"].clone(),
        Rating([1..4001, 1..4001, 1..4001, 1..4001]),
    );
    result
}

fn jump(
    workflows: &Workflow<'_>,
    out: &mut Vec<Rating<Range<i32>>>,
    s: &str,
    r: Rating<Range<i32>>,
) {
    match s {
        "A" => out.push(r),
        "R" => (),
        other => recur(workflows, out, &mut workflows[other].clone(), r),
    }
}

fn recur(
    workflows: &Workflow<'_>,
    out: &mut Vec<Rating<Range<i32>>>,
    rules: &mut Vec<Rule>,
    r: Rating<Range<i32>>,
) {
    if let Some(rule) = rules.pop() {
        match rule {
            Rule::Less(f, v, next) => {
                jump(workflows, out, next, r.clone().with_upper_bound(f, v));
                recur(workflows, out, rules, r.with_lower_bound(f, v));
            }
            Rule::More(f, v, next) => {
                jump(workflows, out, next, r.clone().with_lower_bound(f, v + 1));
                recur(workflows, out, rules, r.with_upper_bound(f, v + 1));
            }
            Rule::Final(next) => jump(workflows, out, next, r),
        }
    }
}

impl Rating<Range<i32>> {
    fn with_upper_bound(mut self, field: Field, value: i32) -> Self {
        self.0[field as usize].end = self.0[field as usize].end.min(value);
        self
    }

    fn with_lower_bound(mut self, field: Field, value: i32) -> Self {
        self.0[field as usize].start = self.0[field as usize].start.max(value);
        self
    }

    fn contains(&self, r: &Rating<i32>) -> bool {
        std::iter::zip(&self.0, r.0).all(|(range, rating)| range.contains(&rating))
    }

    fn num_combinations(&self) -> usize {
        self.0.iter().map(|r| r.len()).product()
    }
}

impl Rating<i32> {
    fn score(&self) -> i32 {
        self.0.iter().sum()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    X = 0,
    M = 1,
    A = 2,
    S = 3,
}

impl Field {
    fn parser(i: &mut &str) -> PResult<Self> {
        alt((
            'x'.value(Self::X),
            'm'.value(Self::M),
            'a'.value(Self::A),
            's'.value(Self::S),
        ))
        .parse_next(i)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rule<'a> {
    Less(Field, i32, &'a str),
    More(Field, i32, &'a str),
    Final(&'a str),
}

impl<'a> Rule<'a> {
    fn parser(i: &mut &'a str) -> PResult<Self> {
        use Rule::*;
        alt((
            seq!(Less(
                terminated(Field::parser, '<'),
                terminated(dec_int, ':'),
                take_while(1.., char::is_alphabetic),
            )),
            seq!(More(
                terminated(Field::parser, '>'),
                terminated(dec_int, ':'),
                take_while(1.., char::is_alphabetic),
            )),
            take_while(1.., char::is_alphabetic).map(Final),
        ))
        .parse_next(i)
    }
}

fn entry_parser<'a>(i: &mut &'a str) -> PResult<(&'a str, Vec<Rule<'a>>)> {
    let name = take_while(1.., char::is_alphabetic).parse_next(i)?;
    let mut rules: Vec<Rule<'_>> =
        delimited('{', separated(1.., Rule::parser, ','), '}').parse_next(i)?;
    rules.reverse();
    Ok((name, rules))
}

fn rating_parser(i: &mut &str) -> PResult<Rating<i32>> {
    let mut rating = Rating([0; 4]);
    delimited(
        '{',
        separated(
            4,
            separated_pair(Field::parser, '=', dec_int).map(|(f, r)| rating.0[f as usize] = r),
            ',',
        ),
        '}',
    )
    .parse_next(i)?;
    Ok(rating)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_rule_final() {
        assert_eq!(Rule::parser.parse_peek("rfg"), Ok(("", Rule::Final("rfg"))));
    }

    #[test]
    fn parse_rule_lt() {
        assert_eq!(
            Rule::parser.parse_peek("a<2006:qkq"),
            Ok(("", Rule::Less(Field::A, 2006, "qkq")))
        );
    }

    #[test]
    fn parse_rule_gt() {
        assert_eq!(
            Rule::parser.parse_peek("m>2090:A"),
            Ok(("", Rule::More(Field::M, 2090, "A")))
        );
    }

    #[test]
    fn can_parse_workflow() {
        let (input, _) = SAMPLE1.split_once("\n\n").unwrap();
        let map = parse_workflows(input);
        assert_eq!(map.len(), 11);
    }

    #[test]
    fn can_parse_ratings() {
        let (_, input) = SAMPLE1.split_once("\n\n").unwrap();
        let ratings: Vec<_> = parse_ratings(input);
        assert_eq!(ratings.len(), 5);
    }

    #[test]
    fn can_solve_part1() {
        let (ws, rs) = SAMPLE1.split_once("\n\n").unwrap();
        let workflows = parse_workflows(ws);
        let ratings = parse_ratings(rs);
        let bounds = compute_bounds(&workflows);
        assert_eq!(solve_part1(&bounds, &ratings), 19114);
    }

    #[test]
    fn can_solve_part2() {
        let (ws, _) = SAMPLE1.split_once("\n\n").unwrap();
        let workflows = parse_workflows(ws);
        let bounds = compute_bounds(&workflows);
        assert_eq!(solve_part2(&bounds), 167409079868000);
    }

    const SAMPLE1: &str = "
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
    ";
}
