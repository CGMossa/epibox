//! Source: [Assignment 4](http://prac.im.pwr.wroc.pl/~szwabin/assets/abm/labs/l4.pdf)
use itertools::{zip, Itertools};
use ndarray::Array2;
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
    Size(u32),
}

//FIXME: there's a problem..
//impl Into<Neighbourhood> for Neighbourhood {
//    fn into(self) -> Neighbourhood::Radius {
//        match self {
//            Neighbourhood::Radius(a) => Neighbourhood::Radius(a),
//            neighbourhood_size @ Neighbourhood::Size(size) => match neighbourhood_size {
//                Self::Neighbours(8) => Self::Radius(1),
//                Self::Neighbours(24) => Self::Radius(2),
//                Self::Neighbours(48) => Self::Radius(3),
//                Self::Neighbours(80) => Self::Radius(4),
//                Self::Neighbours(120) => Self::Radius(5),
//                _ => panic!("invalid radius"),
//            },
//        }
//    }
//}

impl Neighbourhood {
    fn from_neighbourhood_size(size: u32) -> Self {
        match size {
            8 => Self::Radius(1),
            24 => Self::Radius(2),
            48 => Self::Radius(3),
            80 => Self::Radius(4),
            120 => Self::Radius(5),
            _ => panic!("invalid radius"),
        }
    }

    fn from_radius(radius: u32) -> Self {
        match radius {
            1 => Self::Size(8),
            2 => Self::Size(24),
            3 => Self::Size(48),
            4 => Self::Size(80),
            5 => Self::Size(120),
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
    fn closest_neighbours(&self, position: (isize, isize)) -> Vec<Mark> {
        let left_x = position.0 - 1;
        let right_x = position.0 + 2;
        let left_y = position.1 - 1;
        let right_y = position.1 + 2;

        //        periodic_boundary_intervals = vec![];

        todo!()
    }

    fn run() {
        todo!()
    }

    /// Similar neighbor index
    fn segregation_index(&self) -> f64 {
        // for all individuals of a certain type, find the number of their neighbors that are of
        // the same type, and average over this.
        self.agents
            .iter()
            .map(
                |agent| {
                    self.closest_neighbours(agent.position)
                        .iter()
                        .map(|x| if *x == agent.mark { 1. } else { 0. })
                        //                        .map(|x| if agent.mark == x.mark { 1. } else { 0. })
                        .sum::<f64>()
                        / self.m_red as f64
                }, //FIXME: no way to select m based on Mark
            )
            .sum::<f64>()
            / self.agents.len() as f64
    }
}

fn find_all_periodic_boundary_neighbours_2d(
    lattice: Array2<i32>,
    neighbourhood: Neighbourhood,
) -> Vec<Vec<i32>> {
    use itertools::iproduct;
    use ndarray::s;
    let n = lattice.dim().0 as isize;
    //    dbg!(n);
    let radius = match neighbourhood {
        Neighbourhood::Radius(a) => a,
        _ => todo!(),
    };

    if radius >= lattice.len() as u32 {
        panic!("neighbourhood radius is too large")
    }

    lattice
        .indexed_iter()
        .map(|((idx, idy), _)| {
            let intervalsx;
            let intervalsy;
            let leftx = idx as isize - radius as isize;
            let lefty = idy as isize - radius as isize;
            let rightx = idx as isize + radius as isize + 1;
            let righty = idy as isize + radius as isize + 1;

            if leftx < 0 {
                //intervals = vec![(n + left) as usize..n as usize, 0..right as usize];
                intervalsx = vec![
                    (n + leftx) as usize..n as usize,
                    0..idx,
                    idx + 1..rightx as usize,
                ];
            } else if rightx >= n {
                //intervals = vec![left as usize..n as usize, 0..(right % n) as usize];
                intervalsx = vec![
                    leftx as usize..idx,
                    idx + 1..n as usize,
                    0..(rightx % n) as usize,
                ];
            } else {
                //intervals = vec![left as usize..right as usize];
                intervalsx = vec![leftx as usize..idx, idx + 1..rightx as usize];
            }
            if lefty < 0 {
                //intervals = vec![(n + left) as usize..n as usize, 0..right as usize];
                intervalsy = vec![
                    (n + lefty) as usize..n as usize,
                    0..idy,
                    idy + 1..righty as usize,
                ];
            } else if righty >= n {
                //intervals = vec![left as usize..n as usize, 0..(right % n) as usize];
                intervalsy = vec![
                    lefty as usize..idy,
                    idy + 1..n as usize,
                    0..(righty % n) as usize,
                ];
            } else {
                //intervals = vec![left as usize..right as usize];
                intervalsy = vec![lefty as usize..idy, idy + 1..righty as usize];
            }

            // APPROACH 1
            //            iproduct!(intervalsx.into_iter(), intervalsy.into_iter())
            // APPROACH 2
            intervalsx
                .into_iter()
                .cartesian_product(intervalsy.into_iter())
                // APPROACH 3
                //            intervalsx
                //                .clone()
                //                .into_iter()
                //                .interleave(intervalsx.into_iter())
                //                .zip(
                //                    intervalsy
                //                        .clone()
                //                        .into_iter()
                //                        .interleave(intervalsy.into_iter()),
                //                )
                .map(|(x,y)|
//                    lattice.get(x).clone().unwrap_or_default().to_vec()
                    lattice.slice(s![x, y])
                        .into_iter().cloned().collect_vec())
                .flatten()
                .collect_vec()
        })
        .collect_vec()
}

#[test]
fn figuring_out_boundary_slicing() {
    let n = 5;
    let arr = Array2::from_shape_vec((5, 5), (0..5_i32.pow(2)).collect_vec()).unwrap_or_default();

    println!("{:>2}", arr);
    let neighbourhood = find_all_periodic_boundary_neighbours_2d(arr, Neighbourhood::Radius(1));
    println!("{:#?}\nSize = {}", neighbourhood, neighbourhood.len());
    println!("{:?}", neighbourhood.iter().map(|x| x.len()).collect_vec());
}

#[test]
fn example() {
    let sketch_model = Model::new(20, 5, 8, 8, 0, 0);
    println!("{:}", sketch_model.lattice);
    println!("{:?}", sketch_model.lattice.dim());

    println!(
        "No. of neighbours {:?}",
        sketch_model
            .lattice
            .indexed_iter()
            .map(|((x, y), _)| sketch_model
                .closest_neighbours((x as isize, y as isize))
                .len())
            .collect_vec()
    );

    println!(
        "Closest indices of (9, 9): {:?}\n",
        sketch_model.closest_neighbours((9, 9))
    );
    println!(
        "Closest indices of (0, 0): {:?}\n",
        sketch_model.closest_neighbours((0, 0))
    );
    println!(
        "Closest indices of (4, 0): {:?}\n",
        sketch_model.closest_neighbours((4, 0))
    );
    println!(
        "Closest indices of (0, 4): {:?}\n",
        sketch_model.closest_neighbours((0, 4))
    );

    println!("Segregation index: {:?}.", sketch_model.segregation_index());
}

/// For all elements in `lattice`, returns themselves ~including~ excluding their neighbours.
/// I.e. one is a neighbour to thyself.
/// One could remove the element from its neighbour-slice.
/// Currently, we remove the origin from each neighbourhood.
fn find_all_periodic_neighbours<T: Copy>(
    lattice: &[T],
    neighbourhood: Neighbourhood,
) -> Vec<Vec<T>> {
    let n = lattice.len() as isize;

    //    let Neighbourhood::Radius(radius) = neighbourhood;

    let radius = match neighbourhood {
        Neighbourhood::Radius(a) => a,
        _ => todo!(),
    };

    if radius >= lattice.len() as u32 {
        panic!("neighbourhood radius is too large")
    }

    lattice
        .iter()
        .enumerate()
        .map(|(id, _)| {
            let intervals;
            let left = id as isize - radius as isize;
            let right = id as isize + radius as isize + 1;
            if left < 0 {
                //intervals = vec![(n + left) as usize..n as usize, 0..right as usize];
                intervals = vec![
                    (n + left) as usize..n as usize,
                    0..id,
                    id + 1..right as usize,
                ];
            } else if right >= n {
                //intervals = vec![left as usize..n as usize, 0..(right % n) as usize];
                intervals = vec![
                    left as usize..id,
                    id + 1..n as usize,
                    0..(right % n) as usize,
                ];
            } else {
                //intervals = vec![left as usize..right as usize];
                intervals = vec![left as usize..id, id + 1..right as usize];
            }
            intervals
                .into_iter()
                .map(|x| lattice.get(x).clone().unwrap_or_default().to_vec())
                .flatten()
                .collect_vec()
        })
        .collect_vec()
}

#[test]
fn one_dim_periodic_boundary_neighbours() {
    let lattice = vec![43, 24, 10, 20, 4];
    let neighbourhood = find_all_periodic_neighbours(&lattice, Neighbourhood::Radius(1));
    println!("{:?}", lattice);
    println!("{:?}\nSize = {:}", neighbourhood, neighbourhood.len());
    println!("{:?}", neighbourhood.iter().map(Vec::len).collect_vec());
    print!("\n\n");
    use Mark::*;
    let lattice = vec![Red, None, None, Blue, Blue, Red, Blue, None, Blue];
    let neighbourhood = find_all_periodic_neighbours(&lattice, Neighbourhood::Radius(1));
    println!("{:?}", lattice);
    println!("{:?}\nSize = {:}", neighbourhood, neighbourhood.len());
    println!("{:?}", neighbourhood.iter().map(Vec::len).collect_vec());
    print!("\n\n");
    let lattice = vec![43, 24, 10, 20, 4];
    let neighbourhood = find_all_periodic_neighbours(&lattice, Neighbourhood::Radius(2));
    println!("{:?}", lattice);
    println!("{:?}\nSize = {:}", neighbourhood, neighbourhood.len());
    println!("{:?}", neighbourhood.iter().map(Vec::len).collect_vec());
    print!("\n\n");
    use Mark::*;
    let lattice = vec![Red, None, None, Blue, Blue, Red, Blue, None, Blue];
    let neighbourhood = find_all_periodic_neighbours(&lattice, Neighbourhood::Radius(2));
    println!("{:?}", lattice);
    println!("{:?}\nSize = {:}", neighbourhood, neighbourhood.len());
    println!("{:?}", neighbourhood.iter().map(Vec::len).collect_vec());
}
