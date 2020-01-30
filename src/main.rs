//use epibox::simple_forest_fire::percolation_threshold;
//use itertools::iproduct;

fn main() {
    //    for grid_size in &[20, 50, 100] {
    //        println!("Grid size {}", *grid_size);
    //
    //        let tree_densities = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
    //        let max_iterations = vec![1, 10, 20, 50, 100, 200, 250];
    //        let prob_thres_array =
    //            iproduct!(&tree_densities, &max_iterations).map(|(&tree_density, &max_iter)| {
    //                percolation_threshold(*grid_size, tree_density, max_iter)
    //            });
    //        println!(
    //            "{:.5}",
    //            ndarray::Array2::<f64>::from_shape_vec(
    //                (tree_densities.len(), max_iterations.len()),
    //                prob_thres_array.collect()
    //            )
    //            .unwrap()
    //        );
    //    }
}
