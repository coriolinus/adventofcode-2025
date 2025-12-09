use aoclib::{
    geometry::{tile::DisplayWidth, Direction},
    parse,
};
use color_eyre::{
    eyre::{bail, OptionExt},
    Result,
};
use itertools::Itertools;
use std::{cmp::Ordering, path::Path};

#[derive(Debug, Clone, Copy, parse_display::FromStr, parse_display::Display)]
#[display("{x},{y}")]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn area(&self, other: &Self) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

pub fn part1(input: &Path) -> Result<()> {
    let points = parse::<Point>(input)?.collect::<Vec<_>>();
    let max_area = points
        .iter()
        .cartesian_product(points.iter())
        .map(|(left, right)| left.area(right))
        .max()
        .ok_or_eyre("no points to consider")?;

    println!("largest area: {max_area}");
    Ok(())
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, parse_display::Display)]
enum Tile {
    #[default]
    #[display(".")]
    Empty,
    #[display("#")]
    Red,
    #[display("X")]
    EdgeGreen,
    #[display("x")]
    InnerGreen,
}

impl DisplayWidth for Tile {
    const DISPLAY_WIDTH: usize = 1;
}

impl Tile {
    fn is_legal_for_pt2_rectangle(&self) -> bool {
        matches!(self, Self::Red | Self::EdgeGreen | Self::InnerGreen)
    }
}

type Map = aoclib::geometry::Map<Tile>;

fn walk_rectangle(
    a: aoclib::geometry::Point,
    b: aoclib::geometry::Point,
) -> impl Iterator<Item = aoclib::geometry::Point> {
    let low_x = a.x.min(b.x);
    let low_y = a.y.min(b.y);
    let high_x = a.x.max(b.x);
    let high_y = a.y.max(b.y);

    let left_edge = (low_y..high_y).map(move |y| aoclib::geometry::Point::new(low_x, y));
    let top_edge = (low_x..high_x).map(move |x| aoclib::geometry::Point::new(x, high_y));
    let right_edge = (low_y..(high_y + 1))
        .skip(1)
        .rev()
        .map(move |y| aoclib::geometry::Point::new(high_x, y));
    let bottom_edge = (low_x..(high_x + 1))
        .skip(1)
        .rev()
        .map(move |x| aoclib::geometry::Point::new(x, low_y));

    left_edge
        .chain(top_edge)
        .chain(right_edge)
        .chain(bottom_edge)
}

pub fn part2(input: &Path) -> Result<()> {
    // too tired to do this properly, let's see if this works
    let points = parse::<Point>(input)?
        .map(|point| aoclib::geometry::Point::new(point.x as _, point.y as _))
        .collect::<Vec<_>>();
    let mut max_x = None;
    let mut max_y = None;
    for point in &points {
        max_x = max_x.max(Some(point.x as usize));
        max_y = max_y.max(Some(point.y as usize));
    }
    let (width, height) = max_x.zip(max_y).ok_or_eyre("no points")?;

    let mut map = Map::new(width + 1, height + 1);
    for (from, to) in points.iter().copied().circular_tuple_windows() {
        debug_assert_ne!(from, to, "points should not be equal");
        let diff = to - from;
        let dx = match diff.x.cmp(&0) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        };
        let dy = match diff.y.cmp(&0) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        };
        map[from] = Tile::Red;
        // take the manhattan distance, to stop before hitting the destination point
        // and skip the origin point
        for point in map
            .project(from, dx, dy)
            .take(diff.manhattan() as _)
            .skip(1)
        {
            if map[point] != Tile::Empty {
                bail!(
                    "encountered a non-empty point while drawing border: {point:?} -> {:?}",
                    map[point]
                );
            }
            map[point] = Tile::EdgeGreen;
        }
    }

    let (dx, dy) = Direction::Right.deltas();
    for edge_point in map.edge(Direction::Left) {
        let mut is_interior = false;
        let mut prev_point = None;
        for point in map.project(edge_point, dx, dy) {
            match map[point] {
                Tile::Empty => {
                    if is_interior {
                        map[point] = Tile::InnerGreen;
                    }
                }
                Tile::Red | Tile::EdgeGreen => {
                    if !matches!(prev_point, Some(Tile::Red | Tile::EdgeGreen)) {
                        is_interior = !is_interior
                    }
                }
                Tile::InnerGreen => bail!("encountered an unexpected inner gren while filling"),
            }
            prev_point = Some(map[point]);
        }
    }

    // assumption: there are no point pairs such that they enclose a large empty rectangle
    let max_area = points
        .iter()
        .copied()
        .cartesian_product(points.iter().copied())
        .filter(|(a, b)| {
            walk_rectangle(*a, *b).all(|point| map[point].is_legal_for_pt2_rectangle())
        })
        .map(|(a, b)| {
            let a = Point {
                x: a.x as _,
                y: a.y as _,
            };
            let b = Point {
                x: b.x as _,
                y: b.y as _,
            };
            a.area(&b)
        })
        .max()
        .ok_or_eyre("no areas computed")?;
    println!("max area (pt 2): {max_area}");

    Ok(())
}
