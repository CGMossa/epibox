//! unimplemented
//!
//! Incomplete. No idea how this could be "best" implemented. Elements that are to be considered
//!
//! - Wind direction
//!   - Preset, randomly set once, randomly at each step, randomly during a fixed period..
//! - Speed -- affects tiles
//! - Does wind direction affect diagonals?
//!
use crate::simple_forest_fire::Forrest;

#[test]
fn wind_example() {
    //    let forest_example = Forrest::new(25, 0.5);
    let forest_example = Forrest::new(100, 0.5);

    println!("{}", forest_example.cells());
    println!("{}", forest_example.no_clusters());
    println!("{:?}", forest_example.cluster_sizes());
}
