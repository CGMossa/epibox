use rand::prelude::Distribution;
use rand::thread_rng;
use std::cmp::Ordering::Equal;
use std::f64::consts::PI as pi;

#[test]
#[ignore]
/// Source: [Tutorial](https://machinelearningmastery.com/what-is-bayesian-optimization/)
fn example() {
    type Numeric = f64;
    fn objective(x: Numeric) -> Numeric {
        x.powi(2) * (5. * pi * x).sin().powi(6)
    }

    fn create_objective_with_noise(noise: Numeric) -> impl Fn(Numeric) -> Numeric {
        let noise_distr = rand_distr::Normal::new(0., noise).unwrap();

        move |x| objective(x) + noise_distr.sample(&mut thread_rng())
    }

    let X = ndarray::Array1::range(0., 1., 0.01);
    println!("{:.3}", X);

    let y = X.mapv(create_objective_with_noise(0.));
    println!("y = {:.3}", y);
    let ynoise = X.mapv(create_objective_with_noise(0.1));
    println!("ynoise = {:.4}", y);

    let (ix, yix) = y
        .indexed_iter()
        .max_by(|(_, x), (_, y)| x.partial_cmp(y).unwrap())
        .unwrap();
    let xix = X[ix];

    println!("Optima: x = {:.3}, y = {:.3}", xix, yix);
}
