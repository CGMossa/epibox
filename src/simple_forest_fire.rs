use ndarray::Array2;
use ndarray_rand::RandomExt;
use rayon::prelude::*;
use std::fmt::{Display, Error, Formatter};

///! Source: [Assignment 1](http://prac.im.pwr.wroc.pl/~szwabin/assets/abm/labs/l1.pdf)

//struct SquareLattice<CellType> {
//    x: PhantomData<CellType>,
//}

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
            }
        });
        no_fire
    }
    fn update(&mut self) {
        // it is only neccessary to count the neighbours of cells with trees in them.
        let mut new_cells = self.cells.mapv(|cell| {
            if let State::Burning = cell {
                State::Empty
            } else {
                cell
            }
        });

        for ((r, c), cell) in new_cells.indexed_iter_mut() {
            if let State::Tree = cell {
                let r_start = if r == 0 { 0 } else { r - 1 };
                for r_neigh in r_start..r + 2 {
                    let c_start = if c == 0 { 0 } else { c - 1 };
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
pub fn percolation_threshold(grid_size: usize, tree_density: f64, max_iter: usize) -> f64 {
    let mut fire_pass_throughs = 0usize;

    //    for _repetition in 0..max_iter {
    (0..max_iter)
        .collect::<Vec<_>>()
        .par_iter()
        .map(move |_| {
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
    let L = 10;
    let simple_grid = ndarray::Array2::<State>::default((L, L));

    let mut simple_universe = Universe::new(10, 0.5);
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
fn examples() {
    //    let L = 20;
    //    let L = 50;
    //    let L = 100;

    // run this simulation for multiple p's.
}

mod hoshen_kopelman {
    use crate::simple_forest_fire::Universe;
    use ndarray::{Array2, ArrayBase};

    struct Clustering {
        occupied: Array2<bool>,
        labels: Vec<i32>,
        label: Array2<i32>,
    }

    impl From<Universe> for Clustering {
        fn from(u: Universe) -> Self {
            Self {
                occupied: u.cells.mapv(|x| match x {
                    super::State::Tree => true,
                    super::State::Burning => false,
                    super::State::Empty => false,
                }),
                labels: vec![],
                label: Array2::<i32>::zeros(u.cells.dim()),
            }
        }
    }

    impl Clustering {
        fn new(occupied: Array2<bool>) -> Self {
            todo!()
        }
        fn raster_scan(&self, n_rows: usize, n_columns: usize) {
            let mut largest_label = 0;
            let label = &self.label;
            let occupied = &self.occupied;

            for x in 0..n_columns {
                for y in 0..n_rows {
                    if !occupied.uget((x, y)) {
                        continue;
                    }
                    let left = *occupied.uget((x - 1, y));
                    let above = *occupied.uget((x, y - 1));
                    if (left == false) & (above == false) {
                        // Neither a label above nor to the left
                        largest_label += 1;
                        *label.uget_mut((x, y)) = largest_label;
                    } else if (left != false) & (above == false) {
                        *label.uget_mut((x, y)) = self.find(left);
                    } else if (left == false) & (above != false) {
                        *label.uget_mut((x, y)) = self.find(above);
                    } else {
                        self.union(left, above);
                        *label.uget_mut((x, y)) = self.find(left);
                    }
                }
            }
        }

        fn union(&self, x: i32, y: i32) {
            self.labels[self.find(x)] = self.find(y);
        }

        fn find(&self, mut x: i32) {
            let labels = &self.labels;
            let mut y = x;
            while labels[y] != y {
                y = labels[y];
            }
            while labels[x] != x {
                let z = labels[x];
                labels[x] = y;
                x = z;
            }
            return y;
        }
    }
}

fn hoshen_kopelman() {}
