use ndarray::Array2;
use ndarray_rand::RandomExt;
use rayon::prelude::*;
use std::fmt::{Display, Error, Formatter};

///! Source: [Assignment 1](http://prac.im.pwr.wroc.pl/~szwabin/assets/abm/labs/l1.pdf)

#[derive(Debug, Clone)]
pub struct Forrest {
    cells: ndarray::Array2<TreeState>,
    vegetation_probability: f64,
    size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TreeState {
    None,
    Tree,
    Burning,
}

impl Forrest {
    pub(crate) fn new(size: usize, probability: f64) -> Self {
        let sampler = ndarray_rand::rand_distr::Bernoulli::new(probability)
            .expect("given probability argument is not valid");

        let cells = Array2::random((size, size), sampler);
        let cells = cells.mapv(|x| if x { TreeState::Tree } else { TreeState::None });

        Self {
            cells,
            vegetation_probability: probability,
            size,
        }
    }

    pub fn cells(&self) -> &Array2<TreeState> {
        &self.cells
    }

    pub(crate) fn no_clusters(&self) -> usize {
        hoshen_kopelman::Raster::from(self.clone())
            .raster_scan()
            .no_clusters()
    }
    fn no_fire(&self) -> bool {
        let mut no_fire = true;
        self.cells.visit(|x| {
            if let TreeState::Burning = x {
                no_fire = false;
                return;
            }
        });
        no_fire
    }
    fn update(&mut self) {
        // it is only necessary to count the neighbours of cells with trees in them.
        let mut new_cells = self.cells.mapv(|cell| {
            if let TreeState::Burning = cell {
                TreeState::None
            } else {
                cell
            }
        });

        for ((r, c), cell) in new_cells.indexed_iter_mut() {
            if let TreeState::Tree = cell {
                let r_start = if r == 0 { r } else { r - 1 };
                for r_neigh in r_start..r + 2 {
                    let c_start = if c == 0 { c } else { c - 1 };
                    for c_neigh in c_start..r + 2 {
                        if (r_neigh, c_neigh) == (r, c) {
                            continue;
                        }
                        if let Some(TreeState::Burning) = self.cells.get((r_neigh, c_neigh)) {
                            *cell = TreeState::Burning;
                        }
                    }
                }
            }
        }

        self.cells = new_cells;
    }
}

impl Default for TreeState {
    fn default() -> Self {
        TreeState::None
    }
}

impl Display for TreeState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}",
            match self {
                TreeState::None => {
                    " "
                }
                TreeState::Tree => {
                    "T"
                }
                TreeState::Burning => {
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
            let mut run = Forrest::new(grid_size, tree_density);
            //        println!("Initial grid: \n {}", run.cells);

            run.cells.column_mut(0).mapv_inplace(|x| {
                if let TreeState::Tree = x {
                    TreeState::Burning
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
                    .any(|x| x == &TreeState::Burning);
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

    let mut simple_universe = Forrest::new(grid_size, 0.5);
    simple_universe.cells.column_mut(0).fill(TreeState::Burning);

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
    use crate::simple_forest_fire::Forrest;
    use ndarray::Array2;

    struct Clustering {
        occupied: Array2<bool>,
        labels: Vec<usize>,
        label: Array2<usize>,
    }

    type Label = usize;
    pub(crate) struct Raster {
        occupied: Array2<bool>,
        label: Array2<Label>,
    }

    impl Raster {
        pub(crate) fn no_clusters(&self) -> usize {
            self.label.fold(0usize, |acc, x| acc.max(*x))
        }

        pub(crate) fn raster_scan(&mut self) -> &Self {
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

            self
        }
    }

    impl From<Forrest> for Raster {
        fn from(u: Forrest) -> Self {
            Self {
                occupied: u.cells.mapv(|x| match x {
                    super::TreeState::Tree => true,
                    super::TreeState::Burning => false,
                    super::TreeState::None => false,
                }),
                label: Array2::<usize>::zeros(u.cells.dim()),
            }
        }
    }

    #[test]
    fn hoshen_kopelman_examples() {
        let forrest = Forrest::new(10, 0.5);
        println!("Forrest: \n{:<2}", forrest.cells);
        let mut cluster_example = Raster::from(forrest.clone());

        cluster_example.raster_scan();
        println!("Clusters: \n{:<2}", cluster_example.label);
        println!("No. of clusters: {:}", cluster_example.no_clusters());
    }
}
