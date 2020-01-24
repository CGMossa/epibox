use ndarray::Array2;
use ndarray_rand::RandomExt;
use rayon::prelude::*;
use std::fmt::{Display, Error, Formatter};

///! Source: [Assignment 1](http://prac.im.pwr.wroc.pl/~szwabin/assets/abm/labs/l1.pdf)

#[derive(Debug, Clone)]
struct Universe {
    cells: ndarray::Array2<State>,
    vegetation_probability: f64,
    size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum State {
    Empty,
    Tree,
    Burning,
}

impl Universe {
    fn new(size: usize, probability: f64) -> Self {
        let sampler = ndarray_rand::rand_distr::Bernoulli::new(probability)
            .expect("given probability argument is not valid");

        let cells = Array2::random((size, size), sampler);
        let cells = cells.mapv(|x| if x { State::Tree } else { State::Empty });
        //        cells.par_mapv_inplace(|x| State::from(x));

        Self {
            cells,
            vegetation_probability: probability,
            size,
        }
    }
    fn no_fire(&self) -> bool {
        let mut no_fire = true;
        self.cells.visit(|x| {
            if let State::Burning = x {
                no_fire = false;
                return;
            }
        });
        no_fire
    }
    fn update(&mut self) {
        // it is only necessary to count the neighbours of cells with trees in them.
        let mut new_cells = self.cells.mapv(|cell| {
            if let State::Burning = cell {
                State::Empty
            } else {
                cell
            }
        });

        for ((r, c), cell) in new_cells.indexed_iter_mut() {
            if let State::Tree = cell {
                let r_start = if r == 0 { r } else { r - 1 };
                for r_neigh in r_start..r + 2 {
                    let c_start = if c == 0 { c } else { c - 1 };
                    for c_neigh in c_start..r + 2 {
                        if (r_neigh, c_neigh) == (r, c) {
                            continue;
                        }
                        if let Some(State::Burning) = self.cells.get((r_neigh, c_neigh)) {
                            *cell = State::Burning;
                        }
                    }
                }
            }
        }

        self.cells = new_cells;
    }
}

impl Default for State {
    fn default() -> Self {
        Self::Empty
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}",
            match self {
                State::Empty => {
                    " "
                }
                State::Tree => {
                    "T"
                }
                State::Burning => {
                    "!"
                }
            }
        )?;
        Ok(())
    }
}

/// For each realisation of a grid with a given tree density (given in probability) there is a
/// percolation threshold, i.e. a probability that the fire will cross from one size to another.
/// Initially, only trees are set on fire, from the left most column and if a fire reaches the
/// right most column, then it is counted.
///
/// # Note
/// This procedure is currently using rayon. It is 8% slower when running only with `max_iter=1`, and
/// thus it is recommended to make a version that only does single-runs if needed elsewhere.
pub fn percolation_threshold(grid_size: usize, tree_density: f64, max_iter: usize) -> f64 {
    //    let mut fire_pass_throughs = 0usize;

    //    for _repetition in 0..max_iter {
    (0..max_iter)
        .collect::<Vec<_>>()
        .par_iter()
        .map(|_| {
            let mut run = Universe::new(grid_size, tree_density);
            //        println!("Initial grid: \n {}", run.cells);

            run.cells.column_mut(0).mapv_inplace(|x| {
                if let State::Tree = x {
                    State::Burning
                } else {
                    x
                }
            });
            //        println!("Left-side trees burning: \n {}", run.cells);

            loop {
                run.update();
                let flag_rightside_burning = run
                    .cells
                    .column(run.cells.ncols() - 1) // get rightmost column
                    .iter()
                    .any(|x| x == &State::Burning);
                if flag_rightside_burning {
                    //                println!("Right-side trees burning: \n {}", run.cells);
                    //                    fire_pass_throughs += 1;
                    return 1;
                    //                    break;
                }
                if run.no_fire() {
                    //                println!("Fire stopped: \n {}", run.cells);
                    break;
                }
            }
            0
        })
        .sum::<u64>() as f64
        / max_iter as f64
    //    fire_pass_throughs as f64 / max_iter as f64
}

#[test]
fn percolation_one_at_a_time() {
    let grid_size = 10;
    let tree_density = 0.5;
    let max_iter = 100;

    let perco_thres_estimate = percolation_threshold(grid_size, tree_density, max_iter);
    println!(
        "L = {}; Prob. = {}; N = {} => {}",
        grid_size, tree_density, max_iter, perco_thres_estimate
    );
}

#[test]
fn percolation_bunch() {
    for grid_size in vec![20, 50, 100] {
        for tree_density in vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9] {
            //            for max_iter in vec![1, 10, 20, 50, 100, 250] {
            for max_iter in vec![1, 10, 20] {
                //TODO: improve this, by just adding to the already gathered simulations
                let perco_thres_estimate = percolation_threshold(grid_size, tree_density, max_iter);
                println!(
                    "L = {}; Prob. = {}; N = {} => {}",
                    grid_size, tree_density, max_iter, perco_thres_estimate
                );
            }
        }
    }
}

#[test]
fn checking_out_ndarray_for_lattice() {
    let grid_size = 10;
    //    let simple_grid = ndarray::Array2::<State>::default((grid_size, grid_size));

    let mut simple_universe = Universe::new(grid_size, 0.5);
    simple_universe.cells.column_mut(0).fill(State::Burning);

    loop {
        println!("{}", simple_universe.cells);
        simple_universe.update();

        if simple_universe.no_fire() {
            println!("{}", simple_universe.cells);
            break;
        }
    }

    //    println!("Time = {}", 0);
    //    println!("{}", simple_universe.cells);
    //    println!("Time = {}", 1);
    //    simple_universe.update();
    //    println!("{}", simple_universe.cells);
    //    println!("Time = {}", 2);
    //    simple_universe.update();
    //    println!("{}", simple_universe.cells);
}

#[test]
#[ignore]
fn examples() {
    //    let L = 20;
    //    let L = 50;
    //    let L = 100;

    // run this simulation for multiple p's.
}

mod hoshen_kopelman {
    use crate::simple_forest_fire::Universe;
    use ndarray::Array2;

    struct Clustering {
        occupied: Array2<bool>,
        labels: Vec<usize>,
        label: Array2<usize>,
    }

    type Label = usize;
    struct Raster {
        occupied: Array2<bool>,
        label: Array2<Label>,
    }

    impl Raster {
        fn raster_scan(&mut self) {
            let mut largest_label = 0usize;
            for ((x, y), occupied) in self.occupied.indexed_iter() {
                if !occupied {
                    continue;
                }
                if !self.occupied.get((x, y)).unwrap() {
                    continue;
                }
                let left = if y == 0 {
                    0usize
                } else {
                    *self.label.get((x, y - 1)).unwrap()
                };
                let above = if x == 0 {
                    0usize
                } else {
                    *self.label.get((x - 1, y)).unwrap()
                };

                match (left != 0, above != 0) {
                    (false, false) => {
                        largest_label += 1;
                        *self.label.get_mut((x, y)).unwrap() = largest_label;
                    }
                    (false, true) => {
                        *self.label.get_mut((x, y)).unwrap() = above;
                    }
                    (true, false) => {
                        *self.label.get_mut((x, y)).unwrap() = left;
                    }
                    (true, true) => {
                        let (lower, upper) = if left <= above {
                            (left, above)
                        } else {
                            (above, left)
                        };
                        if lower != upper {
                            self.label
                                .par_mapv_inplace(|x| if x == upper { lower } else { x });
                        }
                        *self.label.get_mut((x, y)).unwrap() = lower;
                    }
                }
            }
        }
    }

    impl From<Universe> for Raster {
        fn from(u: Universe) -> Self {
            Self {
                occupied: u.cells.mapv(|x| match x {
                    super::State::Tree => true,
                    super::State::Burning => false,
                    super::State::Empty => false,
                }),
                label: Array2::<usize>::zeros(u.cells.dim()),
            }
        }
    }

    #[test]
    fn hoshen_kopelman_examples() {
        let forrest = Universe::new(10, 0.3);
        println!("Forrest: \n{:<2}", forrest.cells);
        let mut cluster_example = Raster::from(forrest.clone());

        cluster_example.raster_scan();
        println!("Clusters: \n{:<2}", cluster_example.label);
    }
}
