use aoclib::parse;
use color_eyre::{eyre::OptionExt, Result};
use std::{
    collections::{BTreeMap, HashMap},
    path::Path,
};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    parse_display::Display,
    parse_display::FromStr,
    derive_more::Add,
    derive_more::Sub,
)]
#[display("{x},{y},{z}")]
struct Point {
    x: u32,
    y: u32,
    z: u32,
}

impl Point {
    fn distance_square(&self, other: &Self) -> u64 {
        let dx = self.x.abs_diff(other.x) as u64;
        let dy = self.y.abs_diff(other.y) as u64;
        let dz = self.z.abs_diff(other.z) as u64;

        dx * dx + dy * dy + dz * dz
    }
}

/// Compute the circuit assignments for a given point set
///
/// The assignments are a map whose key is the circuit ID and whose value is the set of indices in the `points`
/// list which are assigned to this circuit.
///
/// If `connection_limit` is `None` then when the loop breaks due to all junction boxes forming a single circuit,
/// the final two points which were joined will be assigned to `break_points`. This value is never read from within this function.
fn compute_circuit_assignments(
    points: &[Point],
    connection_limit: Option<usize>,
    break_points: &mut Option<(Point, Point)>,
) -> HashMap<usize, Vec<usize>> {
    // each point starts assigned to its own circuit, where the circuit id is the index of that point in points
    let mut circuit_assignments = (0..points.len()).collect::<Vec<_>>();
    // but we also keep track of the indices assigned to each circuit
    let mut indices_by_circuit = circuit_assignments
        .iter()
        .copied()
        .enumerate()
        .map(|(idx, circuit_id)| (circuit_id, vec![idx]))
        .collect::<HashMap<_, _>>();

    // keep track of the pairs, by distance
    let mut pairs_by_distance = BTreeMap::<_, Vec<_>>::new();
    for (i, a) in points.iter().enumerate() {
        for (j, b) in points.iter().take(i).enumerate() {
            let distance = a.distance_square(b);
            pairs_by_distance.entry(distance).or_default().push((i, j));
        }
    }
    // we might limit this iterator, or not, depending on the arguments
    let mut pairs_iter = pairs_by_distance
        .values()
        .flat_map(|list| list.iter())
        .copied();
    let mut limited_iter;
    let iter = match connection_limit {
        None => &mut pairs_iter as &mut dyn Iterator<Item = _>,
        Some(connection_limit) => {
            limited_iter = pairs_iter.take(connection_limit);
            &mut limited_iter
        }
    };
    for (i, j) in iter {
        if circuit_assignments[i] == circuit_assignments[j] {
            // not actually a new connection
            // skip further work
            continue;
        }
        // this _is_ a new connection, which we implement by reassigning the circuit
        // assignment for everything on the bigger circuit to the smaller circuit
        let bigger = circuit_assignments[i].max(circuit_assignments[j]);
        let smaller = circuit_assignments[i].min(circuit_assignments[j]);

        // we have to update both circuit_assignments and indices_by_circuit
        // this is a 3-step process:
        // 1. remove the indices for the bigger circuit
        // 2. update all those indices in the assignments to the smaller
        // 3. append the big indices to those for the smaller circuit
        let mut big_indices = indices_by_circuit.remove(&bigger).expect(
            "removing the bigger always works because circuit assignments are kept current",
        );
        for idx in big_indices.iter().copied() {
            circuit_assignments[idx] = smaller;
        }
        let small_indices = indices_by_circuit.get_mut(&smaller).expect(
            "accessing the smaller always works because circuit assignments are kept current",
        );
        small_indices.append(&mut big_indices);

        if indices_by_circuit.len() <= 1 {
            *break_points = Some((points[i], points[j]));
            break;
        }
    }

    indices_by_circuit
}

/// Solve part 1
///
/// - compute distances between each point pair
/// - considering point pairs in order by distance, connect them
fn solve_part1(points: &[Point], connection_limit: usize) -> u64 {
    let indices_by_circuit = compute_circuit_assignments(points, Some(connection_limit), &mut None);
    let mut circuit_sizes = indices_by_circuit
        .values()
        .map(|indices| indices.len() as u64)
        .collect::<Vec<_>>();
    circuit_sizes.sort();
    circuit_sizes.into_iter().rev().take(3).product()
}

pub fn part1(input: &Path, connection_limit: usize) -> Result<()> {
    let points = parse::<Point>(input)?.collect::<Vec<_>>();
    let circuit_size_product = solve_part1(&points, connection_limit);
    println!("product of size of 3 largest circuits: {circuit_size_product}");
    Ok(())
}

pub fn part2(input: &Path) -> Result<()> {
    let points = parse::<Point>(input)?.collect::<Vec<_>>();
    let mut break_points = None;
    compute_circuit_assignments(&points, None, &mut break_points);
    let (a, b) = break_points.ok_or_eyre("no break points computed somehow")?;
    let x_product = a.x as u64 * b.x as u64;
    println!("x product (pt 2): {x_product}");
    Ok(())
}
