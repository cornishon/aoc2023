use std::{cmp::Reverse, ops::Sub};

use fxhash::{FxHashMap, FxHashSet};
use winnow::{ascii::dec_uint, error::StrContext, PResult, Parser};

fn main() {
    let input = std::fs::read_to_string("inputs/day22").unwrap();
    let bricks = apply_gravity(parse_input(&input));
    println!("Part 1: {}", solve_part1(&bricks));
    println!("Part 2: {}", solve_part2(&bricks));
}

fn solve_part1(bricks: &[Brick]) -> usize {
    let non_removable = bricks
        .iter()
        .map(|b| b.supporting_bricks(bricks))
        .filter_map(|bs| if let &[b] = bs.as_slice() { Some(b) } else { None })
        .collect::<FxHashSet<_>>();
    bricks.len() - non_removable.len()
}

fn solve_part2(bricks: &[Brick]) -> usize {
    let supported_by = bricks
        .iter()
        .map(|b| b.supporting_bricks(bricks))
        .collect::<Vec<_>>();

    let mut supporting = vec![FxHashSet::default(); bricks.len()];
    for (i, ss) in supported_by.iter().enumerate() {
        for s in ss {
            supporting[*s].insert(i);
        }
    }

    supported_by
        .iter()
        .filter_map(|bs| if let &[b] = bs.as_slice() { Some(b) } else { None })
        .collect::<FxHashSet<_>>()
        .into_iter()
        .map(|i| {
            let mut fallen = FxHashSet::default();
            fallen.insert(i);
            chain_reaction(i, &supporting, &supported_by, &mut fallen);
            fallen.len() - 1
        })
        .sum()
}

fn chain_reaction(
    idx: usize,
    supporting: &[FxHashSet<usize>],
    supported_by: &[Vec<usize>],
    fallen: &mut FxHashSet<usize>,
) {
    for s in &supporting[idx] {
        if supported_by[*s].iter().all(|s| fallen.contains(s)) {
            fallen.insert(*s);
        }
    }
    for s in &supporting[idx] {
        chain_reaction(*s, supporting, supported_by, fallen)
    }
}

fn apply_gravity(mut bricks: Vec<Brick>) -> Vec<Brick> {
    bricks.sort_unstable_by_key(|v| Reverse(v.origin.z));
    let mut v = Vec::with_capacity(bricks.len());
    let mut height_map = FxHashMap::<(u16, u16), u16>::default();
    let get = |m: &FxHashMap<_, _>, x, y| m.get(&(x, y)).copied().unwrap_or_default();
    while let Some(b) = bricks.pop() {
        let Vec3 { x, y, z: _ } = b.origin;
        match b.axis {
            Axis::X(h) => {
                let h0 = (0..h)
                    .map(|i| get(&height_map, x + i, y))
                    .max()
                    .unwrap_or_default();
                v.push(Brick {
                    origin: Vec3::new(x, y, h0 + 1),
                    ..b
                });
                for i in 0..h {
                    height_map.insert((x + i, y), h0 + 1);
                }
            }
            Axis::Y(h) => {
                let h0 = (0..h)
                    .map(|i| get(&height_map, x, y + i))
                    .max()
                    .unwrap_or_default();
                v.push(Brick {
                    origin: Vec3::new(x, y, h0 + 1),
                    ..b
                });
                for i in 0..h {
                    height_map.insert((x, y + i), h0 + 1);
                }
            }
            Axis::Z(h) => {
                let h0 = get(&height_map, x, y);
                v.push(Brick {
                    origin: Vec3::new(x, y, h0 + 1),
                    ..b
                });
                height_map.insert((x, y), h0 + h);
            }
        }
    }
    v
}

fn parse_input(input: &str) -> Vec<Brick> {
    input
        .trim()
        .lines()
        .map(|ln| Brick::parser.parse(ln))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| eprintln!("{e}"))
        .unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec3 {
    x: u16,
    y: u16,
    z: u16,
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Vec3 {
    fn new(x: u16, y: u16, z: u16) -> Self {
        Self { x, y, z }
    }

    fn parser(i: &mut &str) -> PResult<Self> {
        (dec_uint, ',', dec_uint, ',', dec_uint)
            .map(|(x, _, y, _, z)| Vec3::new(x, y, z))
            .parse_next(i)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Brick {
    origin: Vec3,
    axis: Axis,
}

impl Brick {
    fn parser(i: &mut &str) -> PResult<Self> {
        let origin = Vec3::parser.parse_next(i)?;
        let _ = '~'.parse_next(i)?;
        let axis = Vec3::parser
            .verify_map(|v| match v - origin {
                Vec3 { x, y: 0, z: 0 } => Some(Axis::X(x + 1)),
                Vec3 { x: 0, y, z: 0 } => Some(Axis::Y(y + 1)),
                Vec3 { x: 0, y: 0, z } => Some(Axis::Z(z + 1)),
                _ => None,
            })
            .context(StrContext::Label(
                "brick is not axis-aligned in positive direction",
            ))
            .parse_next(i)?;
        Ok(Self { origin, axis })
    }

    fn supporting_bricks(&self, bricks: &[Brick]) -> Vec<usize> {
        bricks
            .iter()
            .enumerate()
            .filter_map(|(i, b)| self.sits_on_top_of(b).then_some(i))
            .collect()
    }

    fn sits_on_top_of(&self, other: &Brick) -> bool {
        let top_z = match other.axis {
            Axis::X(_) | Axis::Y(_) => other.origin.z,
            Axis::Z(h) => other.origin.z + h - 1,
        };
        if self.origin.z != 1 + top_z {
            return false;
        }
        let area = self.xy_area();
        for p in other.xy_area() {
            if area.contains(&p) {
                return true;
            }
        }
        false
    }

    fn xy_area(&self) -> FxHashSet<(u16, u16)> {
        let Vec3 { x, y, .. } = self.origin;
        match self.axis {
            Axis::X(h) => (0..h).map(|i| (x + i, y)).collect(),
            Axis::Y(h) => (0..h).map(|i| (x, y + i)).collect(),
            Axis::Z(_) => [(x, y)].into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Axis {
    X(u16),
    Y(u16),
    Z(u16),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve_part1() {
        let bricks = apply_gravity(parse_input(SAMPLE1));
        assert_eq!(solve_part1(&bricks), 5);
    }

    #[test]
    fn can_solve_part2() {
        let bricks = apply_gravity(parse_input(SAMPLE1));
        assert_eq!(solve_part2(&bricks), 7);
    }

    const SAMPLE1: &str =
        "1,0,1~1,2,1\n0,0,2~2,0,2\n0,2,3~2,2,3\n0,0,4~0,2,4\n2,0,5~2,2,5\n0,1,6~2,1,6\n1,1,8~1,1,9";
}
