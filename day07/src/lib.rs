use aoclib::geometry::{tile::DisplayWidth, Direction, Map};
use color_eyre::{
    eyre::{eyre, Context},
    Result,
};
use std::path::Path;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, strum::EnumString, strum::Display)]
enum Tile {
    #[default]
    #[strum(serialize = ".")]
    Empty,
    #[strum(serialize = "S")]
    Start,
    #[strum(serialize = "^")]
    Splitter,
    #[strum(serialize = "|")]
    Beam(u64),
}

impl DisplayWidth for Tile {
    const DISPLAY_WIDTH: usize = 1;
}

impl Tile {
    /// How many timelines led to the beam being present in this tile?
    fn timelines(&self) -> Option<u64> {
        match self {
            Tile::Empty | Tile::Splitter => None,
            Tile::Start => Some(1),
            Tile::Beam(n) => Some(*n),
        }
    }

    /// Adjust the number of timelines in this tile.
    ///
    /// If the function returns a number greater than 0, this tile becomes a beam with the given number of timelines.
    /// Otherwise, it becomes empty.
    fn adjust_timelines(&mut self, adjustment: impl FnOnce(u64) -> u64) -> Result<()> {
        if matches!(self, Self::Start | Self::Splitter) {
            return Err(eyre!("cannot mutate start or splitter tiles"));
        }
        let existing_timelines = self.timelines().unwrap_or_default();
        let timelines = adjustment(existing_timelines);
        if timelines == 0 {
            *self = Tile::Empty
        } else {
            *self = Tile::Beam(timelines)
        }
        Ok(())
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
    ///
    /// The beam at any given point records the number of timelines which lead to a beam existing at this point.
    fn project(&mut self) -> Result<u64> {
        let mut new_split_beams = 0;
        let (dx, dy) = Direction::Down.deltas();
        for left_edge in self
            .diagram
            .project(self.diagram.top_left(), dx, dy)
            .skip(1)
        {
            let (dx, dy) = Direction::Right.deltas();
            for point in self.diagram.project(left_edge, dx, dy) {
                let Some(timelines_from_above) = self.diagram[point + Direction::Up].timelines()
                else {
                    // no projections from above; the rest of this loop is irrelevant
                    continue;
                };
                match self.diagram[point] {
                    Tile::Start => {
                        return Err(eyre!("beam intersected a start point; are there two?"))
                    }
                    Tile::Empty | Tile::Beam(_) => self.diagram[point]
                        .adjust_timelines(|current| current + timelines_from_above)?,
                    Tile::Splitter => {
                        new_split_beams += 1;
                        for direction in [Direction::Left, Direction::Right] {
                            self.diagram[point + direction]
                                .adjust_timelines(|current| current + timelines_from_above)?;
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
    let mut manifold = TachyonManifold::parse(input)?;
    manifold.project()?;
    let timelines = manifold
        .diagram
        .edge(Direction::Down)
        .map(|point| manifold.diagram[point].timelines().unwrap_or_default())
        .sum::<u64>();
    println!("timelines (pt 2): {timelines}");
    Ok(())
}
