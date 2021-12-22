use anyhow::{bail, ensure, Context, Result};
use std::{
    collections::HashMap,
    fmt::{Display, Write},
    io::{self, Read},
    ops::Range,
    str::FromStr,
};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn read_input(input: &str) -> Result<(ImageEnhancer, Image)> {
    let (head, tail) = input.split_once("\n\n").context("bad input")?;

    Ok((head.parse()?, tail.parse()?))
}

fn part1(input: &str) -> Result<()> {
    let (enhancer, image) = read_input(input)?;
    let image = enhancer.enhance(&image)?;
    let image = enhancer.enhance(&image)?;

    let sum = image.pixels.values().filter(|p| p.is_light()).count();

    println!("Part 1 answer: {}", sum);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let (enhancer, mut image) = read_input(input)?;
    for _ in 0..50 {
        image = enhancer.enhance(&image)?;
    }

    let sum = image.pixels.values().filter(|p| p.is_light()).count();

    println!("Part 2 answer: {}", sum);

    Ok(())
}

struct ImageEnhancer {
    pixels: Vec<Pixel>,
    invert_mode: bool,
}

impl ImageEnhancer {
    // We use a 3x3 lookup square in each step.
    const STEP_SIZE: usize = 3;

    // The lookup vector needs to have at least 2^(step-size^2) elements.
    // E.g., if the step size is 3 for a 3x3 lookup square then the lookup
    // vector needs to contain at least 2^9 = 512 elements.
    const MIN_PIXELS: usize = 2^(Self::STEP_SIZE^2);

    fn new(pixels: Vec<Pixel>) -> Result<Self> {
        ensure!(
            pixels.len() >= Self::MIN_PIXELS,
            "image enhancer requires at least {} enhanced pixels",
            Self::MIN_PIXELS
        );

        // If the first pixel lookup value differs from the default image background
        // then what we consider to be the image's background will flip on each
        // enhancement.
        let invert_mode = pixels[0] != Image::DEFAULT_BACKGROUND;

        Ok(Self { pixels, invert_mode })
    }

    fn enhance(&self, image: &Image) -> Result<Image> {
        let mut pixels = HashMap::new();
        let mut top_left = Point::new(i32::MAX, i32::MAX);
        let mut bot_right = Point::new(i32::MIN, i32::MIN);

        for point in image.iter() {
            let idx = point
                .square()
                .iter()
                .map(|&p| image.pixel_at(p))
                .map(|p| if p.is_light() { "1" } else { "0" })
                .collect::<Vec<_>>()
                .join("");
            let idx = usize::from_str_radix(idx.as_str(), 2)?;

            let pixel = *self
                .pixels
                .get(idx)
                .context(format!("invalid index: {}", idx))?;

            pixels.insert(point, pixel);

            if pixel == image.foreground {
                top_left.x = top_left.x.min(point.x);
                top_left.y = top_left.y.min(point.y);
                bot_right.x = bot_right.x.max(point.x);
                bot_right.y = bot_right.x.max(point.y);
            }
        }

        let (background, foreground) = if self.invert_mode {
            (image.foreground, image.background)
        } else {
            (image.background, image.foreground)
        };

        Ok(Image {
            pixels,
            top_left,
            bot_right,
            background,
            foreground,
        })
    }
}

impl FromStr for ImageEnhancer {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pixels = vec![];
        for ch in s.chars() {
            pixels.push(ch.to_string().parse()?);
        }

        ImageEnhancer::new(pixels)
    }
}

#[derive(Clone)]
struct Image {
    pixels: HashMap<Point, Pixel>,
    top_left: Point,
    bot_right: Point,
    background: Pixel,
    foreground: Pixel,
}

impl Image {
    const BORDER_WIDTH: usize = 4;
    const DEFAULT_BACKGROUND: Pixel = Pixel::Dark;
    const DEFAULT_FOREGROUND: Pixel = Pixel::Light;

    fn pixel_at(&self, point: Point) -> Pixel {
        self.pixels.get(&point).cloned().unwrap_or(self.background)
    }

    fn iter(&self) -> ImageIter {
        let border = Self::BORDER_WIDTH as i32;
        ImageIter::new(
            (self.top_left.x - border)..(self.bot_right.x + border + 1),
            (self.top_left.y - border)..(self.bot_right.y + border + 1),
        )
    }
}

impl FromStr for Image {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pixels = HashMap::new();
        let mut top_left = Point::new(i32::MAX, i32::MAX);
        let mut bot_right = Point::new(i32::MIN, i32::MIN);

        for (y, line) in s.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let pixel: Pixel = ch.to_string().parse()?;
                let point = Point::new(x as i32, y as i32);
                pixels.insert(point, pixel);

                if pixel == Self::DEFAULT_FOREGROUND {
                    top_left.x = top_left.x.min(point.x);
                    top_left.y = top_left.y.min(point.y);
                    bot_right.x = bot_right.x.max(point.x);
                    bot_right.y = bot_right.x.max(point.y);
                }
            }
        }

        Ok(Image {
            pixels,
            top_left,
            bot_right,
            background: Self::DEFAULT_BACKGROUND,
            foreground: Self::DEFAULT_FOREGROUND,
        })
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::new();

        let mut line = None;
        for point in self.iter() {
            buf += match line {
                Some(y) if point.y > y => "\n",
                _ => "",
            };

            line = Some(point.y);

            let pixel = self.pixel_at(point);
            buf += pixel.to_string().as_str();
        }

        f.write_str(&buf)?;

        Ok(())
    }
}

struct ImageIter {
    xr: Range<i32>,
    yr: Range<i32>,
    curr: Point,
}

impl ImageIter {
    fn new(xr: Range<i32>, yr: Range<i32>) -> ImageIter {
        let curr = Point::new(xr.start, yr.start);
        ImageIter { xr, yr, curr }
    }
}

impl Iterator for ImageIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr.y >= self.yr.end {
            return None;
        }

        let point = self.curr;

        self.curr.x += 1;
        if self.curr.x >= self.xr.end {
            self.curr.x = self.xr.start;
            self.curr.y += 1;
        }

        Some(point)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Pixel {
    Light,
    Dark,
}

impl Pixel {
    fn is_light(&self) -> bool {
        *self == Pixel::Light
    }
}

impl FromStr for Pixel {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#" => Ok(Pixel::Light),
            "." => Ok(Pixel::Dark),
            _ => bail!("bad pixel: {}", s),
        }
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Pixel::Light => '#',
            Pixel::Dark => '.',
        };

        f.write_char(ch)?;

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn square(&self) -> Vec<Point> {
        vec![
            Self::new(self.x - 1, self.y - 1), // Above left.
            Self::new(self.x, self.y - 1),     // Above.
            Self::new(self.x + 1, self.y - 1), // Above right.
            Self::new(self.x - 1, self.y),     // Left.
            *self,                             // Self.
            Self::new(self.x + 1, self.y),     // Right.
            Self::new(self.x - 1, self.y + 1), // Below left.
            Self::new(self.x, self.y + 1),     // Below.
            Self::new(self.x + 1, self.y + 1), // Below right.
        ]
    }
}
