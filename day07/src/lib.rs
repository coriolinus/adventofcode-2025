use aoclib::geometry::{tile::DisplayWidth, Direction, Map};
use color_eyre::{
    eyre::{eyre, Context},
    Result,
};
use std::path::Path;

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, parse_display::FromStr, parse_display::Display,
)]
enum Tile {
    #[default]
    #[display(".")]
    Empty,
    #[display("S")]
    Start,
    #[display("^")]
    Splitter,
    #[display("|")]
    Beam,
}

impl DisplayWidth for Tile {
    const DISPLAY_WIDTH: usize = 1;
}

impl Tile {
    fn projects_beam(&self) -> bool {
        matches!(self, Self::Start | Self::Beam)
    }
}

struct TachyonManifold {
    diagram: Map<Tile>,
}

impl TachyonManifold {
    fn parse(input: &Path) -> Result<Self> {
        let diagram =
            <Map<Tile> as TryFrom<&Path>>::try_from(input).wrap_err("parsing tachyon manifold")?;
        Ok(Self { diagram })
    }

    /// Project the tachyon beam through the manifold, counting the number of times the beam is split.
    ///
    /// The number of splits is just the number of splitters which were impacted by a beam.
    fn project(&mut self) -> Result<u32> {
        let mut new_split_beams = 0;
        let (dx, dy) = Direction::Down.deltas();
        for left_edge in self
            .diagram
            .project(self.diagram.top_left(), dx, dy)
            .skip(1)
        {
            let (dx, dy) = Direction::Right.deltas();
            for point in self.diagram.project(left_edge, dx, dy) {
                if !self.diagram[point + Direction::Up].projects_beam() {
                    continue;
                }
                match self.diagram[point] {
                    Tile::Start => {
                        return Err(eyre!("beam intersected a start point; are there two?"))
                    }
                    Tile::Beam => {}
                    Tile::Empty => self.diagram[point] = Tile::Beam,
                    Tile::Splitter => {
                        new_split_beams += 1;
                        for direction in [Direction::Left, Direction::Right] {
                            if self.diagram[point + direction] == Tile::Empty {
                                self.diagram[point + direction] = Tile::Beam;
                            }
                        }
                    }
                }
            }
        }
        Ok(new_split_beams)
    }
}

pub fn part1(input: &Path) -> Result<()> {
    let mut manifold = TachyonManifold::parse(input)?;
    let new_split_beams = manifold.project()?;
    println!("beam splits: {new_split_beams}");
    Ok(())
}

pub fn part2(input: &Path) -> Result<()> {
    unimplemented!("input file: {:?}", input)
}
