use std::ops::{Add, Div, Mul, RangeInclusive, Sub};

use winnow::{
    ascii::{float, space0},
    PResult, Parser,
};

const RANGE: RangeInclusive<f64> = 200000000000000.0..=400000000000000.0;

fn main() {
    let input = std::fs::read_to_string("inputs/day24").unwrap();
    let particles = parse_input(&input);
    println!("Part 1: {}", solve_part1(&particles, RANGE, RANGE));
    println!("Part 2: {}", solve_part2(&particles));
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Particle {
    pos: Vec3<f64>,
    vel: Vec3<f64>,
}

impl Particle {
    fn line(self) -> Vec3<f64> {
        Vec3::new(
            -self.vel.y,
            self.vel.x,
            self.vel.x * self.pos.y - self.vel.y * self.pos.x,
        )
    }
}

fn intersect_xy(line1: Vec3<f64>, line2: Vec3<f64>) -> Option<(f64, f64)> {
    let det = line1.x * line2.y - line2.x * line1.y;
    if det == 0.0 {
        return None;
    }
    let x = (line1.z * line2.y - line2.z * line1.y) / det;
    let y = (line1.x * line2.z - line2.x * line1.z) / det;
    Some((x, y))
}

fn parse_input(input: &str) -> Vec<Particle> {
    let mut vecp = Vec3::<f64>::parser(float);
    input
        .trim()
        .lines()
        .map(|line| {
            let (l, r) = line.split_once('@').unwrap();
            let pos = vecp.parse(l.trim()).map_err(|e| println!("{e}")).unwrap();
            let vel = vecp.parse(r.trim()).map_err(|e| println!("{e}")).unwrap();
            Particle { pos, vel }
        })
        .collect()
}

fn solve_part1(
    particles: &[Particle],
    x_range: RangeInclusive<f64>,
    y_range: RangeInclusive<f64>,
) -> usize {
    let lines = particles.iter().map(|p| p.line()).collect::<Vec<_>>();
    let mut intersections = 0;
    for (i, &pi) in particles.iter().enumerate() {
        for (j, &pj) in particles.iter().enumerate().skip(i + 1) {
            // print!("({i}, {j}): ");
            let Some((x, y)) = intersect_xy(lines[i], lines[j]) else {
                // println!("parallel");
                continue;
            };
            let ti = (x - pi.pos.x) / pi.vel.x;
            let tj = (x - pj.pos.x) / pj.vel.x;
            match (ti < 0., tj < 0.) {
                (true, true) => {}  // println!("in the past (both)"),
                (true, false) => {} // println!("in the past (A)"),
                (false, true) => {} // println!("in the past (B)"),
                (false, false) => {
                    // println!("{x:.3}, {y:.3}");
                    if x_range.contains(&x) && y_range.contains(&y) {
                        intersections += 1;
                    }
                }
            }
        }
    }
    intersections
}

// https://www.reddit.com/r/adventofcode/comments/18pnycy/comment/kxqjg33
fn solve_part2(particles: &[Particle]) -> usize {
    let p1 = particles[1].pos - particles[0].pos;
    let p2 = particles[2].pos - particles[0].pos;
    let v1 = particles[1].vel - particles[0].vel;
    let v2 = particles[2].vel - particles[0].vel;
    let t1 = -p1.cross(p2).dot(v2) / v1.cross(p2).dot(v2);
    let t2 = -p1.cross(p2).dot(v1) / p1.cross(v2).dot(v1);
    let c1 = particles[1].pos + particles[1].vel * t1;
    let c2 = particles[2].pos + particles[2].vel * t2;
    let v = (c2 - c1) / (t2 - t1);
    let p = c1 - v * t1;
    let answer = p.x + p.y + p.z;
    assert!(answer.fract().abs() < 1e-10);
    answer as _
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Vec3<T> {
    fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T> Vec3<T> {
    fn parser<'i>(
        p: impl FnMut(&mut &'i str) -> PResult<T> + Copy,
    ) -> impl FnMut(&mut &'i str) -> PResult<Vec3<T>> {
        move |i: &mut &str| {
            (p, (space0, ',', space0), p, (space0, ',', space0), p)
                .map(|(x, _, y, _, z)| Vec3::new(x, y, z))
                .parse_next(i)
        }
    }
}

impl<T> Vec3<T>
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy,
{
    fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn cross(self, other: Self) -> Self {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Add<Output = T>> Add for Vec3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vec3<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Vec3<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve_part1() {
        let p = parse_input(SAMPLE1);
        const RANGE: RangeInclusive<f64> = 7.0..=27.0;
        assert_eq!(solve_part1(&p, RANGE, RANGE), 2);
    }

    #[test]
    fn can_solve_part2() {
        let p = parse_input(SAMPLE1);
        assert_eq!(solve_part2(&p), 47);
    }

    const SAMPLE1: &str = "19, 13, 30 @ -2,  1, -2\n18, 19, 22 @ -1, -1, -2\n20, 25, 34 @ -2, -2, -4\n12, 31, 28 @ -1, -2, -1\n20, 19, 15 @  1, -5, -3";
}
