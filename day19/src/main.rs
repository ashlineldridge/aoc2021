use anyhow::{anyhow, ensure, Context, Result};
use itertools::Itertools;
use lazy_static::lazy_static;
use ndarray::array;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
    ops::{Add, Neg, Sub},
    str::FromStr,
};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let scanners = read_scanners(&input)?;

    part1(&scanners)?;
    part2(&scanners)?;

    Ok(())
}

fn part1(scanners: &[Scanner]) -> Result<()> {
    let scanners = Scanner::align_all(scanners);

    let mut beacons = PointSet::new();
    for scanner in &scanners {
        beacons = beacons.union(&scanner.beacons).cloned().collect();
    }

    println!("Part 1 answer: {}", beacons.len());

    Ok(())
}

fn part2(scanners: &[Scanner]) -> Result<()> {
    let scanners = Scanner::align_all(scanners);
    let positions = scanners.iter().map(|s| s.position).collect::<Vec<_>>();

    let mut max_dist = 0;
    for (p1, p2) in positions.iter().cartesian_product(&positions) {
        let dist = p1.dist(p2);
        max_dist = max_dist.max(dist);
    }

    println!("Part 2 answer: {}", max_dist);

    Ok(())
}

type PointSet = HashSet<Point>;

#[derive(Debug, Clone)]
struct Scanner {
    id: String,
    position: Point,
    beacons: PointSet,
}

impl Scanner {
    const MIN_ALIGN_POINTS: usize = 12;

    fn new(id: String, beacons: PointSet) -> Self {
        Self {
            id,
            beacons,
            position: Point::new(0, 0, 0),
        }
    }

    fn align(&self, other: &Self) -> Option<Self> {
        for f in ROT_SCANNER_FNS.values() {
            let other = f(other);
            for (&p1, &p2) in self.beacons.iter().cartesian_product(&other.beacons) {
                let d = p1 - p2;
                let other = other.transpose(d);

                let overlap = self
                    .beacons
                    .intersection(&other.beacons)
                    .collect::<HashSet<_>>();

                if overlap.len() >= Self::MIN_ALIGN_POINTS {
                    return Some(other);
                }
            }
        }

        None
    }

    fn align_all(scanners: &[Scanner]) -> Vec<Scanner> {
        if scanners.is_empty() {
            return vec![];
        }

        let mut acc = scanners.first().unwrap().clone();
        let mut aligned_scanners: Vec<Scanner> = vec![];
        'outer: loop {
            for s in scanners {
                if aligned_scanners.iter().any(|a| a.id == s.id) {
                    continue;
                }

                if let Some(aligned) = acc.align(s) {
                    acc.beacons = acc.beacons.union(&aligned.beacons).cloned().collect();
                    aligned_scanners.push(aligned);
                    continue 'outer;
                }
            }

            break;
        }

        aligned_scanners
    }

    fn transpose(&self, delta: Point) -> Self {
        Self {
            id: self.id.clone(),
            position: self.position + delta,
            beacons: self.beacons.iter().map(|&b| b + delta).collect(),
        }
    }
}

impl FromStr for Scanner {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^--- scanner (?P<id>\d+) ---$").unwrap();
        }

        let (head, tail) = s.split_once("\n").context("bad input")?;

        let caps = RE.captures(head).ok_or_else(|| anyhow!("bad input"))?;
        let id = caps["id"].into();

        let beacons = tail
            .lines()
            .map(|line| line.parse())
            .collect::<Result<_>>()?;

        Ok(Scanner::new(id, beacons))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn dist(&self, other: &Point) -> usize {
        let d = *self - *other;
        (d.x.abs() + d.y.abs() + d.z.abs()) as usize
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let xyz = s.split(',').collect::<Vec<_>>();
        ensure!(xyz.len() == 3, "bad point: {}", s);

        Ok(Point::new(
            xyz[0].parse()?,
            xyz[1].parse()?,
            xyz[2].parse()?,
        ))
    }
}

type RotPointFn = dyn Fn(&Point) -> Point + Sync;
type RotScannerFn = dyn Fn(&Scanner) -> Scanner + Sync;
type RotTuple = (i32, i32, i32);

lazy_static! {
    // Map of point rotation functions keyed by rotation tuple. Rotatation tuples
    // are in the form (x-degrees, y-degrees, z-degrees) and the rotation
    // function for each tuple will rotate the specified point by those angles
    // within their respective planes.
    static ref ROT_POINT_FNS: HashMap<RotTuple, Box<RotPointFn>> = vec![
        // Set z = 0:
        (0, 0, 0),    // Rotate around x by 0.
        (90, 0, 0),   // Rotate around x by 90.
        (180, 0, 0),  // Rotate around x by 180.
        (270, 0, 0),  // Rotate around x by 270.
        // Set z = 90:
        (0, 0, 90),   // Rotate around x by 0.
        (90, 0, 90),  // Rotate around x by 90.
        (180, 0, 90), // Rotate around x by 180.
        (270, 0, 90), // Rotate around x by 270.
        // Set z = 180:
        (0, 0, 180),   // Rotate around x by 0.
        (90, 0, 180),  // Rotate around x by 90.
        (180, 0, 180), // Rotate around x by 180.
        (270, 0, 180), // Rotate around x by 270.
        // Set z = 270:
        (0, 0, 270),   // Rotate around x by 0.
        (90, 0, 270),  // Rotate around x by 90.
        (180, 0, 270), // Rotate around x by 180.
        (270, 0, 270), // Rotate around x by 270.
        // Set y = 90:
        (0, 90, 0),    // Rotate around z by 0.
        (0, 90, 90),   // Rotate around z by 90.
        (0, 90, 180),  // Rotate around z by 180.
        (0, 90, 270),  // Rotate around z by 270.
        // Set y = 270:
        (0, 270, 0),   // Rotate around z by 0.
        (0, 270, 90),  // Rotate around z by 90.
        (0, 270, 180), // Rotate around z by 180.
        (0, 270, 270), // Rotate around z by 270.
    ]
        .into_iter()
        .map(|(xd, yd, zd)| {
            // Build a 3D rotation matrix for the supplied angles. See:
            // https://en.wikipedia.org/wiki/Rotation_matrix#General_rotations.
            let (sx, cx) = (xd as f32).to_radians().sin_cos();
            let (sy, cy) = (yd as f32).to_radians().sin_cos();
            let (sz, cz) = (zd as f32).to_radians().sin_cos();

            let m = array![
                [cz * cy, cz * sy * sx - sz * cx, cz * sy * cx + sz * sx],
                [sz * cy, sz * sy * sx + cz * cx, sz * sy * cx - cz * sx],
                [-sy, cy * sx, cy * cx],
            ].map(|v| v.round() as i32);

            let b: Box<RotPointFn> = Box::new(move |p: &Point| {
                let v = array![p.x, p.y, p.z];
                let r = m.dot(&v);
                Point::new(r[0], r[1], r[2])
            });

            ((xd, yd, zd), b)
        })
        .collect::<HashMap<(i32, i32, i32), Box<RotPointFn>>>();

    // Map of scanner rotation functions keyed by rotation tuple.
    static ref ROT_SCANNER_FNS: HashMap<RotTuple, Box<RotScannerFn>> = ROT_POINT_FNS
        .iter()
        .map(|(&t, f)| {
            let b: Box<RotScannerFn> = Box::new(move |s: &Scanner| {
                Scanner {
                    id: s.id.clone(),
                    position: f(&s.position),
                    beacons: s.beacons.iter().map(f).collect(),
                }
            });

            (t, b)
        })
        .collect::<HashMap<RotTuple, Box<RotScannerFn>>>();
}

fn read_scanners(input: &str) -> Result<Vec<Scanner>> {
    input.split("\n\n").map(|block| block.parse()).collect()
}
