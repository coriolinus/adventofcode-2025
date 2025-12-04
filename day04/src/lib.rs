use aoclib::geometry::{point::PointTrait, tile::DisplayWidth, Map, Point};
use color_eyre::Result;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, parse_display::Display, parse_display::FromStr)]
enum Tile {
    #[display(".")]
    Empty,
    #[display("@")]
    PaperRoll,
}

impl DisplayWidth for Tile {
    const DISPLAY_WIDTH: usize = 1;
}

// this code goes live if we uncomment the debug map section of pt1
#[allow(dead_code)]
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, parse_display::Display, parse_display::FromStr,
)]
enum DebugTile {
    #[default]
    #[display(".")]
    Empty,
    #[display("@")]
    PaperRoll,
    #[display("x")]
    Accessable,
}

impl DisplayWidth for DebugTile {
    const DISPLAY_WIDTH: usize = 1;
}

fn is_accessable_by_forklift(map: &Map<Tile>, point: Point) -> bool {
    map.in_bounds(point)
        && map[point] == Tile::PaperRoll
        && point
            .adjacent()
            .filter(|adj| map.in_bounds(*adj) && map[*adj] == Tile::PaperRoll)
            .count()
            < 4
}

fn remove_accessable(map: &Map<Tile>, next_map: &mut Map<Tile>) -> u32 {
    let mut removed = 0;
    for (point, &tile) in map.iter() {
        if tile == Tile::PaperRoll && is_accessable_by_forklift(map, point) {
            removed += 1;
            next_map[point] = Tile::Empty;
        }
    }
    removed
}

pub fn part1(input: &Path) -> Result<()> {
    let map = <Map<Tile> as TryFrom<&Path>>::try_from(input)?;
    // let mut debug_map = Map::<DebugTile>::new(map.width(), map.height());
    // for (point, tile) in map.iter() {
    //     if *tile == Tile::PaperRoll {
    //         if is_accessable_by_forklift(&map, point) {
    //             debug_map[point] = DebugTile::Accessable;
    //         } else {
    //             debug_map[point] = DebugTile::PaperRoll;
    //         }
    //     }
    // }
    // eprintln!("{debug_map}");
    let accessable_by_forklift = map
        .iter()
        .filter(|(point, _tile)| is_accessable_by_forklift(&map, *point))
        .count();
    println!("points accessable by forklift: {accessable_by_forklift}");
    Ok(())
}

pub fn part2(input: &Path) -> Result<()> {
    let mut map = <Map<Tile> as TryFrom<&Path>>::try_from(input)?;
    let mut next_map = map.clone();

    let mut total_removed = 0;
    while {
        let removed = remove_accessable(&map, &mut next_map);
        total_removed += removed;
        removed > 0
    } {
        map = next_map.clone();
    }

    println!("total removed (pt2): {total_removed}");
    Ok(())
}
