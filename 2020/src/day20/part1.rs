use super::neighbor::NeighborSet;
use super::tile::{Tile, Projection};
use crate::error::NoSolution;
use std::collections::{HashMap, HashSet};
use std::error::Error;

struct Solver<'a> {
    neighbors: HashMap<&'a Projection, NeighborSet<'a>>,
    side: usize, // the edge length of the square image
    image: Vec<&'a Projection>,
    used: HashSet<u64>, // tile IDs,
}

impl<'a> Solver<'a> {
    fn new(tiles: &'a [Tile], projections: &'a [Projection]) -> Solver<'a> {
        Solver {
            neighbors: NeighborSet::graph(projections),
            side: (tiles.len() as f64).sqrt() as usize, // image is square
            image: Vec::new(),
            used: HashSet::new(),
        }
    }

    fn corner_id_product(&self) -> u64 {
        let m = self.side - 1;
        [(0, 0), (0, m), (m, 0), (m, m)]
            .iter()
            .map(|(i, j)| self.image[i * self.side + j].tile_id)
            .product()
    }

    fn candidates(&self) -> Vec<&'a Projection> {
        let (side, len) = (self.side, self.image.len());
        let (i, j) = (len / side, len % side);
        self.image
            .last()
            .map(|p| {
                let v = if j == 0 {
                    let above = self.image[(i - 1) * self.side];
                    &self.neighbors[above].downs
                } else {
                    &self.neighbors[p].rights
                };
                v.iter()
                    .cloned()
                    .filter(|q| !self.used.contains(&q.tile_id))
                    .collect()
            })
            .unwrap_or_else(|| self.neighbors.keys().cloned().collect())
    }

    fn recur(&mut self) -> Option<u64> {
        if self.image.len() == self.side * self.side {
            return Some(self.corner_id_product());
        }
        let candidates = self.candidates();
        for p in candidates {
            self.image.push(&p);
            self.used.insert(p.tile_id);
            let result = self.recur();
            if result.is_some() {
                return result;
            }
            self.used.remove(&p.tile_id);
            self.image.pop();
        }
        None
    }

    fn solve(mut self) -> Result<u64, NoSolution> {
        self.recur().ok_or(NoSolution)
    }
}

pub fn solve(text: &str) -> Result<u64, Box<dyn Error>> {
    let tiles = Tile::parse_all(text)?;
    let projections = Projection::collect(&tiles);
    Ok(Solver::new(&tiles, &projections).solve()?)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn solve_sample1() {
        let input_path = "tests/day20/sample1";
        let text = fs::read_to_string(input_path).unwrap();
        assert_eq!(20899048083289, solve(&text).unwrap());
    }
}
