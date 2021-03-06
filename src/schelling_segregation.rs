//! Source: [Assignment 4](http://prac.im.pwr.wroc.pl/~szwabin/assets/abm/labs/l4.pdf)
use itertools::Itertools;
use ndarray::Array2;
use rand::seq::IteratorRandom;
use rand::thread_rng;
use std::cell::RefCell;
use std::collections::HashSet;
use std::convert::identity;
use std::fmt::{Display, Error, Formatter};
use std::iter::once;
use std::ops::Deref;
use std::rc::Rc;

const MAX_ITERATIONS: i32 = 10000;

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
/// `m_red` and `m_blue` are no. of closest neighbours to consider.
/// Presumably `j_red` and `j_blue` are percentages.
#[derive(Clone, Debug)]
struct Model {
    //    no_agents: usize,
    no_red: usize,
    no_blue: usize,
    m_red: usize,
    m_blue: usize,
    j_red: usize,
    j_blue: usize,
    //    lattice: Array2<Mark>,
    lattice: Array2<Option<Agent>>,
    //    agents: Vec<&'a Agent>,
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

#[derive(Debug, Clone, Copy)]
struct Agent {
    position: (isize, isize),
    mark: Mark,
    moving: bool,
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
        j_red: f64,
        j_blue: f64,
    ) -> Self {
        let lattice_size = 10;

        assert!(lattice_size * lattice_size >= no_blue + no_red);

        // FIXME: incorporate m_t closest neighbours
        assert_eq!(m_red, 8);
        assert_eq!(m_blue, 8);

        assert!(0.1 <= j_red && j_red <= 0.9);
        assert!(0.1 <= j_blue && j_blue <= 0.9);

        let j_red: usize = (j_red * m_red as f64) as usize;
        let j_blue: usize = (j_blue * m_blue as f64) as usize;

        //        assert!(j_red <= m_red as f64);
        //        assert!(j_blue <= m_blue as f64);

        assert!(m_red <= lattice_size);
        assert!(m_blue <= lattice_size);

        //        let mut lattice = Array2::from_elem((lattice_size, lattice_size), Mark::None);
        let mut lattice = Array2::from_elem((lattice_size, lattice_size), None);

        use rand::prelude::*;

        //                let mut agents = Vec::with_capacity(no_red + no_blue);

        let marks = once(Mark::Red)
            .cycle()
            .take(no_red)
            .chain(once(Mark::Blue).cycle().take(no_blue));
        // Place the marks randomly on the grid
        for (((x, y), cell), mark) in lattice
            .indexed_iter_mut()
            .choose_multiple(&mut thread_rng(), no_red + no_blue)
            .into_iter()
            .zip(marks)
        {
            //TODO: is it possible to place agents directly in the array and have the list of agents
            // be a list of references to those agents?
            *cell = Some(Agent {
                position: (x as isize, y as isize),
                mark: mark,
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
            //            agents,
        }
    }

    /// TODO: Add range of cells where it is considered neighbours
    fn closest_neighbours(&self, position: (isize, isize)) -> Vec<Option<Agent>> {
        use itertools::iproduct;
        use ndarray::s;
        let n = self.lattice.dim().0 as isize;
        //FIXME: this is all wrongly configured
        let radius = 1;
        //        let radius = self.m_blued;
        //        let radius = match neighbourhood {
        //            Neighbourhood::Radius(a) => a,
        //            _ => todo!(),
        //        };

        //        if radius >= lattice.len() as u32 {
        //            panic!("neighbourhood radius is too large")
        //        }

        //        lattice
        //            .indexed_iter()
        //            .map(|((idx, idy), _)| {
        let idx = position.0;
        let idy = position.1;

        let intervalsx;
        let intervalsy;
        let leftx = idx as isize - radius as isize;
        let rightx = idx as isize + radius as isize + 1;
        let lefty = idy as isize - radius as isize;
        let righty = idy as isize + radius as isize + 1;

        if leftx < 0 {
            intervalsx = vec![0..rightx as usize, (n + leftx) as usize..n as usize];
        } else if rightx >= n {
            intervalsx = vec![0..(rightx % n) as usize, leftx as usize..n as usize];
        } else {
            intervalsx = vec![leftx as usize..rightx as usize];
        }
        if lefty < 0 {
            intervalsy = vec![0..righty as usize, (n + lefty) as usize..n as usize];
        } else if righty >= n {
            intervalsy = vec![0..(righty % n) as usize, lefty as usize..n as usize];
        } else {
            intervalsy = vec![lefty as usize..righty as usize];
        }

        // APPROACH 1
        iproduct!(intervalsx.into_iter(), intervalsy.into_iter())
            .map(|(x, y)| {
                self.lattice
                    .slice(s![x, y])
                    .into_iter()
                    .cloned()
                    .collect_vec()
            })
            .flatten()
            .collect_vec()
    }

    fn run(&mut self) {
        let mut no_iterations = 0;

        let mark_count = |model: Self| {
            model
                .mark_lattice()
                .iter()
                .map(|x| match x {
                    Mark::None => 0,
                    _ => 1,
                })
                .sum::<usize>()
        };

        dbg!(mark_count(self.clone()));
        //        while self
        //            .lattice
        //            .iter()
        //            .filter_map(|x| match x {
        //                None => None,
        //                Some(a) if a.moving => Some(true),
        //                _ => Some(false),
        //            })
        //            .clone()
        //            .any(identity)
        //        {
        for _iter in 0..MAX_ITERATIONS {
            self.update_moving_agent();
            no_iterations += 1;

            if no_iterations % 25 == 0 {
                println!("Iteration: {:<5}", no_iterations);
                dbg!(mark_count(self.clone()));
            }

            if no_iterations == MAX_ITERATIONS {
                break;
            }
        }
        dbg!(mark_count(self.clone()));
    }

    fn update_moving_agent(&mut self) {
        //        let present_lattice = self.lattice.clone();
        use ndarray::s;
        let mut rc_lattice = Rc::new(RefCell::new(self.lattice.clone()));
        let moving_agent: &mut Agent = *rc_lattice
            .borrow_mut()
            .iter_mut()
            .filter_map(|cell| match cell {
                Some(ref mut agent) if agent.moving => Some(agent),
                _ => None,
            })
            .next()
            .expect("no moving agents available");

        let empty_position = Rc::get_mut(&mut rc_lattice)
            .unwrap()
            .indexed_iter()
            .filter_map(|(pos, x)| match x {
                None => Some(pos),
                Some(_) => None,
            })
            .choose(&mut thread_rng())
            .expect("no empty cells available");

        //dbg!(&moving_agent);
        //dbg!(empty_position);

        let same_type_neighbours: isize = self
            .closest_neighbours(moving_agent.clone().position)
            .into_iter()
            .map(|x| match x {
                None => 0,
                Some(neighbour_agent) if neighbour_agent.mark == moving_agent.mark => 1,
                Some(_) => 0,
            })
            .sum();
        let same_type_neighbours = same_type_neighbours - 1; // subtract origin
                                                             //dbg!(same_type_neighbours);

        let previous_location = (
            moving_agent.position.0 as usize,
            moving_agent.position.1 as usize,
        );
        moving_agent.position = (empty_position.0 as isize, empty_position.1 as isize);

        dbg!(previous_location, empty_position);
        dbg!(&self.lattice.get(previous_location).unwrap());
        dbg!(&self.lattice.get(empty_position).unwrap());

        assert!(self.lattice.get(previous_location).unwrap().is_some());
        assert!(self.lattice.get(empty_position).unwrap().is_none());

        match moving_agent.mark {
            Mark::None => unreachable!("agent is not assigned type"),
            Mark::Blue => {
                if same_type_neighbours > self.j_blue as isize {
                    //                    print!("Moved. ");
                    *self
                        .lattice
                        .get_mut(empty_position)
                        .expect("new position was not empty") = Some(*moving_agent);
                    *self
                        .lattice
                        .get_mut(previous_location)
                        .expect("previous location was empty already") = None;
                } else if same_type_neighbours == self.j_blue as isize {
                    // settle individual
                    // print!("Settled. ");
                    moving_agent.moving = false;
                }
            }
            Mark::Red => {
                if same_type_neighbours > self.j_red as isize {
                    *self.lattice.get_mut(empty_position).unwrap() = Some(*moving_agent);
                    *self
                        .lattice
                        .get_mut(previous_location)
                        .expect("previous location was empty already") = None;
                } else if same_type_neighbours == self.j_red as isize {
                    // settle individual
                    // print!("Settled. ");
                    moving_agent.moving = false;
                }
            }
        }
        //print!("\n")
    }

    /// Similar neighbor index
    fn segregation_index(&self) -> f64 {
        // for all individuals of a certain type, find the number of their neighbors that are of
        // the same type, and average over this.

        self.lattice
            .iter()
            .filter(|x: &&Option<Agent>| x.is_some())
            .clone()
            .map(
                |some_agent| {
                    let agent = some_agent.clone().unwrap();
                    self.closest_neighbours(agent.position)
                        .iter()
                        .map(|x| match x {
                            None => 0.,
                            Some(ref a) => {
                                if a.mark == agent.mark {
                                    1.
                                } else {
                                    0.
                                }
                            }
                        })
                        //.map(|x| if *x == agent.mark { 1. } else { 0. })
                        //.map(|x| if agent.mark == x.mark { 1. } else { 0. })
                        .sum::<f64>()
                        / self.m_red as f64
                }, //FIXME: no way to select m based on Mark
            )
            .sum::<f64>()
            / self.no_agents() as f64
    }

    //    fn agents(&mut self) -> Vec<&mut Agent> {
    //        self.lattice.iter_mut().flat_map(|x| x).collect()
    //    }

    //    fn moving_agents(&mut self) -> impl Iterator<Item = &'static Agent> {
    //        self.lattice.iter_mut().flat_map(|x| x).filter(|x| x.moving)
    //    }

    fn mark_lattice(&self) -> Array2<Mark> {
        self.lattice.mapv(|x| match x {
            None => Mark::None,
            Some(a) => a.mark,
        })
    }
}

#[test]
fn baseline_model() {
    //    let mut baseline_model = Model::new(250, 250, 8, 8, 0.5, 0.5);
    let mut baseline_model = Model::new(25, 25, 8, 8, 0.5, 0.5);

    //    println!("{:?}", baseline_model.lattice);
    println!("{:}", baseline_model.mark_lattice());
    println!("{:?}", baseline_model.segregation_index());

    println!("Running model until all agents have settled:");
    baseline_model.run();
    println!("{:}", baseline_model.mark_lattice());
    println!("{:?}", baseline_model.segregation_index());
}

#[test]
fn example_runs() {
    let mut sketch_model = Model::new(50, 25, 8, 8, 0.5, 0.5);
    use ndarray::prelude::array;

    //    println!("{:?}", sketch_model.lattice);
    println!("{:?}", sketch_model.lattice.dim());
    //    println!("{:?}", sketch_model.agents());
    println!("{:?}\n", sketch_model.lattice);
    println!("{:}\n", sketch_model.mark_lattice());
    println!("{:}\n", sketch_model.segregation_index());

    println!(
        "Mark count before updates: {}",
        sketch_model
            .mark_lattice()
            .iter()
            .map(|x| match x {
                Mark::None => 0,
                _ => 1,
            })
            .sum::<usize>()
    );

    //FIXME: update agents
    sketch_model.update_moving_agent();
    sketch_model.update_moving_agent();
    sketch_model.update_moving_agent();
    sketch_model.update_moving_agent();
    sketch_model.update_moving_agent();
    sketch_model.update_moving_agent();
    sketch_model.update_moving_agent();
    sketch_model.update_moving_agent();
    sketch_model.update_moving_agent();
    sketch_model.update_moving_agent();
    sketch_model.update_moving_agent();
    sketch_model.update_moving_agent();
    sketch_model.update_moving_agent();
    sketch_model.update_moving_agent();
    sketch_model.update_moving_agent();
    sketch_model.update_moving_agent();
    sketch_model.update_moving_agent();
    sketch_model.update_moving_agent();
    sketch_model.update_moving_agent();
    sketch_model.update_moving_agent();
    sketch_model.update_moving_agent();
    sketch_model.update_moving_agent();
    sketch_model.update_moving_agent();
    sketch_model.update_moving_agent();
    sketch_model.update_moving_agent();
    sketch_model.update_moving_agent();
    sketch_model.update_moving_agent();
    sketch_model.update_moving_agent();
    sketch_model.update_moving_agent();
    sketch_model.update_moving_agent();
    sketch_model.update_moving_agent();
    //    sketch_model.update_moving_agent();
    //    sketch_model.update_moving_agent();
    //    sketch_model.update_moving_agent();
    //    sketch_model.update_moving_agent();
    //    sketch_model.update_moving_agent();
    //    sketch_model.update_moving_agent();
    //    sketch_model.update_moving_agent();
    //    sketch_model.update_moving_agent();
    //    sketch_model.update_moving_agent();
    //    sketch_model.update_moving_agent();
    //    sketch_model.update_moving_agent();
    //    sketch_model.update_moving_agent();
    //    sketch_model.update_moving_agent();
    //    sketch_model.update_moving_agent();
    //    sketch_model.update_moving_agent();
    //    sketch_model.update_moving_agent();
    //    sketch_model.update_moving_agent();
    //    sketch_model.update_moving_agent();
    //    sketch_model.update_moving_agent();
    //    sketch_model.update_moving_agent();
    //    sketch_model.update_moving_agent();
    //    sketch_model.update_moving_agent();
    //sketch_model.update_moving_agents();

    //    println!("{:?}", sketch_model.agents());
    println!("{:}", sketch_model.mark_lattice());
    println!("{:}", sketch_model.segregation_index());

    println!(
        "Mark count after updates: {}",
        sketch_model
            .mark_lattice()
            .iter()
            .map(|x| match x {
                Mark::None => 0,
                _ => 1,
            })
            .sum::<usize>()
    );

    for (pos, cell) in sketch_model.lattice.indexed_iter() {
        if let Some(agent) = cell {
            if pos.0 as isize != agent.position.0 || pos.1 as isize != agent.position.1 {
                println!("{:>2?} === {:>2?}", pos, agent.position);
            }
        }
    }
}

#[test]
fn example() {
    //    let sketch_model = Model::new(20, 5, 8, 8, 0.1, 0.1);
    let sketch_model = Model::new(250, 250, 8, 8, 0.1, 0.1);
    println!("{:?}", sketch_model.lattice);
    println!("{:?}", sketch_model.lattice.dim());

    let all_neighbourhoods = sketch_model
        .lattice
        .indexed_iter()
        .map(|((x, y), _)| {
            sketch_model
                .closest_neighbours((x as isize, y as isize))
                .len()
        })
        .collect_vec();
    println!(
        "No. of neighbours {:?}\nSet of lengths: {:?}",
        all_neighbourhoods,
        all_neighbourhoods.iter().collect::<HashSet<_>>()
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
    let neighbourhood = sketch_model.closest_neighbours((0, 4));
    println!("Closest indices of (0, 4): {:?}\n", neighbourhood);

    println!("Segregation index: {:?}.", sketch_model.segregation_index());
}

/// Returns all the neighbours with periodic boundary condition, including the
/// origin.
fn find_all_periodic_boundary_neighbours_2d<T: Clone>(
    lattice: Array2<T>,
    neighbourhood: Neighbourhood,
) -> Vec<Vec<T>> {
    use itertools::iproduct;
    use ndarray::s;
    let n = lattice.dim().0 as isize;
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
            let rightx = idx as isize + radius as isize + 1;
            let lefty = idy as isize - radius as isize;
            let righty = idy as isize + radius as isize + 1;

            if leftx < 0 {
                intervalsx = vec![0..rightx as usize, (n + leftx) as usize..n as usize];
            } else if rightx >= n {
                intervalsx = vec![0..(rightx % n) as usize, leftx as usize..n as usize];
            } else {
                intervalsx = vec![leftx as usize..rightx as usize];
            }
            if lefty < 0 {
                intervalsy = vec![0..righty as usize, (n + lefty) as usize..n as usize];
            } else if righty >= n {
                intervalsy = vec![0..(righty % n) as usize, lefty as usize..n as usize];
            } else {
                intervalsy = vec![lefty as usize..righty as usize];
            }

            // APPROACH 1
            iproduct!(intervalsx.into_iter(), intervalsy.into_iter())
                .map(|(x, y)| lattice.slice(s![x, y]).into_iter().cloned().collect_vec())
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
    println!(
        "{:?}",
        neighbourhood
            .iter()
            .map(|x| x.len())
            .collect::<HashSet<_>>()
    );
}

/// For all elements in `lattice`, returns themselves ~including~ excluding their neighbours.
/// I.e. one is a neighbour to thyself.
/// One could remove the element from its neighbour-slice.
/// Currently, we remove the origin from each neighbourhood.
fn find_all_periodic_neighbours<T: Clone>(
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
