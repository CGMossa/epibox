//! Source: [Assignment 4](http://prac.im.pwr.wroc.pl/~szwabin/assets/abm/labs/l4.pdf)

use ndarray::Array2;
use ndarray_rand::RandomExt;
use rayon::prelude::IntoParallelRefIterator;
use std::fmt::{Display, Error, Formatter};
use std::iter::once;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mark {
    None,
    Blue,
    Red,
}

impl Display for Mark {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{:}",
            match self {
                Mark::None => ' ',
                Mark::Blue => 'B',
                Mark::Red => 'R',
            }
        )
    }
}
/// `j_red` and `j_blue` can be thought of as thresholds.
/// `m_red` and `m_blue` are no. of closest neighbours to consider
struct Model {
    //    no_agents: usize,
    no_red: usize,
    no_blue: usize,
    m_red: usize,
    m_blue: usize,
    j_red: usize,
    j_blue: usize,
    lattice: Array2<Mark>,
    agents: Vec<Agent>,
}

enum Neighbourhood {
    Radius(u32),
    Neighbours(u32),
}

impl Neighbourhood {
    fn from_radius(radius: u32) -> Self {
        match radius {
            1 => Self::Neighbours(8),
            2 => Self::Neighbours(24),
            3 => Self::Neighbours(48),
            4 => Self::Neighbours(80),
            5 => Self::Neighbours(120),
            _ => panic!("invalid radius"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Agent {
    position: (isize, isize),
    mark: Mark,
    moving: bool,
}

impl Iterator for Agent {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        //        unimplemented!()
        Some(*self)
    }
}

impl Model {
    fn no_agents(&self) -> usize {
        self.no_blue + self.no_red
    }

    fn new(
        no_blue: usize,
        no_red: usize,
        m_red: usize,
        m_blue: usize,
        j_red: usize,
        j_blue: usize,
    ) -> Self {
        let lattice_size = 10;

        // FIXME: incorporate m_t closest neighbours
        assert_eq!(m_red, 8);
        assert_eq!(m_blue, 8);

        assert!(j_red <= m_red);
        assert!(j_blue <= m_blue);

        assert!(m_red <= lattice_size);
        assert!(m_blue <= lattice_size);

        let mut lattice = Array2::from_elem((lattice_size, lattice_size), Mark::None);

        use rand::prelude::*;

        let mut agents = Vec::with_capacity(no_red + no_blue);
        let marks = once(Mark::Red)
            .cycle()
            .take(no_red)
            .chain(once(Mark::Blue).cycle().take(no_blue));
        for (((x, y), cell), mark) in lattice
            .indexed_iter_mut()
            .choose_multiple(&mut thread_rng(), no_red + no_blue)
            .into_iter()
            .zip(marks)
        {
            *cell = mark.clone();
            agents.push(Agent {
                position: (x as isize, y as isize),
                mark: mark.clone(),
                moving: true,
            });
        }

        // The update-scheme first starts off with the red individuals and onto the blue individuals
        Self {
            no_blue,
            no_red,
            m_red,
            m_blue,
            j_red,
            j_blue,
            lattice,
            agents,
        }
    }

    /// TODO: Add range of cells where it is considered neighbours
    fn closest_neighbours(&self, position: (isize, isize)) -> Vec<Agent> {
        use ndarray::s;
        let (x, y) = position;
        let grid_size = self.lattice.dim().0 as isize;
        self.lattice
            .slice(s![
                x - 1..(x + 2).min(grid_size),
                y - 1..(y + 2).min(grid_size)
            ])
            .indexed_iter()
            // includes the origin cell as well
            .map(|(position, mark)| Agent {
                position: (position.0 as isize + x - 1, position.1 as isize + y - 1),
                mark: mark.clone(),
                moving: true,
            })
            // removes the origin cell
            .filter(|agent| agent.position.0 != x && agent.position.1 != y)
            .collect()
    }

    fn run() {
        todo!()
    }

    /// Similar neighbor index
    fn segregation_index(&self) -> f64 {
        // for all individuals of a certain type, find the number of their neighbors that are of
        // the same type, and average over this.
        //        todo!()
        self.agents
            .iter()
            .map(
                |agent| {
                    self.closest_neighbours(agent.position)
                        .iter()
                        .map(|x| if agent.mark == x.mark { 1. } else { 0. })
                        .sum::<f64>()
                        / self.m_red as f64
                }, //FIXME: no way to select m based on Mark
            )
            .sum::<f64>()
            / self.agents.len() as f64
    }
}

#[test]
fn example() {
    let sketch_model = Model::new(12, 2, 8, 8, 0, 0);
    println!("{:}", sketch_model.lattice);

    println!("{:?}", sketch_model.closest_neighbours((1, 1)));
    println!("{:?}", sketch_model.segregation_index());

    //    println!("{:}", Model::new(10, 2).lattice);
    //    println!("{:}", Model::new(10, 2).lattice);

fn find_all_periodic_neighbours(lattice: &[i32]) -> Vec<Vec<i32>> {
    let n = lattice.len() as isize;
    lattice
        .iter()
        .enumerate()
        .map(|(id, _)| {
            let intervals;
            let left = id as isize - 1;
            let right = id as isize + 2;
            if left < 0 {
                intervals = vec![(n + left) as usize..n as usize, 0..right as usize];
            } else if right >= n {
                intervals = vec![left as usize..n as usize, 0..(right % n) as usize];
            } else {
                intervals = vec![left as usize..right as usize];
            }
            intervals
                .into_iter()
                .map(|x| lattice.get(x).clone().unwrap().to_vec())
                .flatten()
                .collect_vec()
        })
        .collect_vec()
}

#[test]
fn one_dim_periodic_boundary_neighbours() {
    let lattice = vec![43, 24, 10, 20, 4];
    println!("{:?}", lattice);
    println!("{:?}", find_all_periodic_neighbours(&lattice));
}
