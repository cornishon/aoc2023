use winnow::{
    ascii::dec_int,
    combinator::{alt, preceded},
    token::take_while,
    PResult, Parser,
};

fn main() {
    let input = std::fs::read_to_string("inputs/day15").unwrap();
    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}

fn solve_part1(input: &str) -> u64 {
    input.trim().split(',').map(hash).sum()
}

fn solve_part2(input: &str) -> i32 {
    let mut hm = HashMap::new();
    input
        .trim()
        .split(',')
        .map(|s| Command::parser.parse(s).unwrap())
        .for_each(|cmd| hm.interpret(cmd));

    hm.buckets
        .into_iter()
        .enumerate()
        .map(|(h, bucket)| {
            let power = bucket
                .into_iter()
                .enumerate()
                .map(|(i, (_, v))| (i as i32 + 1) * v)
                .sum::<i32>();
            (h as i32 + 1) * power
        })
        .sum()
}

fn hash(s: &str) -> u64 {
    s.bytes().fold(0, |h, c| (17 * (h + c as u64)) & 255)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Command<'a> {
    Insert(&'a str, i32),
    Delete(&'a str),
}

impl<'a> Command<'a> {
    fn parser(i: &mut &'a str) -> PResult<Self> {
        let key = take_while(1.., char::is_alphabetic).parse_next(i)?;
        alt((
            '-'.value(Self::Delete(key)),
            preceded('=', dec_int).map(|value| Self::Insert(key, value)),
        ))
        .parse_next(i)
    }
}

struct HashMap<'a> {
    buckets: [Vec<(&'a str, i32)>; 256],
}

impl<'a> HashMap<'a> {
    fn new() -> Self {
        const EMPTY: Vec<(&str, i32)> = Vec::new();
        Self {
            buckets: [EMPTY; 256],
        }
    }

    fn interpret<'b: 'a>(&mut self, cmd: Command<'b>) {
        match cmd {
            Command::Insert(k, v) => self.insert(k, v),
            Command::Delete(k) => self.delete(k),
        }
    }

    fn insert<'b: 'a>(&mut self, key: &'b str, value: i32) {
        let h = hash(key) as usize;
        if let Some(i) = self.buckets[h].iter().position(|&(k, _)| k == key) {
            self.buckets[h][i] = (key, value);
        } else {
            self.buckets[h].push((key, value));
        }
    }

    // deleteHM :: Text -> HashMap -> HashMap
    // deleteHM key = flip M.alter (hash key) $ \case
    //   Nothing -> Nothing
    //   (Just [(k, _)]) | k == key -> Nothing
    //   (Just vs) -> Just $ L.deleteBy (\a b -> fst a == fst b) (key, 0) vs
    fn delete(&mut self, key: &str) {
        let h = hash(key) as usize;
        if let Some(i) = self.buckets[h].iter().position(|&(k, _)| k == key) {
            self.buckets[h].remove(i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve_part1() {
        assert_eq!(solve_part1(SAMPLE1), 1320);
    }

    #[test]
    fn can_solve_part2() {
        assert_eq!(solve_part2(SAMPLE1), 145);
    }

    const SAMPLE1: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
}
