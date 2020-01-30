//! [](http://www.csc.kth.se/utbildning/kandidatexjobb/datateknik/2011/rapport/erneholm_carl-oscar_K11044.pdf)

use ndarray::{Array1, Array2};

type Numeric = f64;

struct Boids {
    location: Array2<Numeric>,
    velocity: Array1<Numeric>,
    course: Array1<Numeric>,
}

impl Boids {
    fn update_location(&self, delta: Numeric) {
        self.location.map
    }
}

struct Simulation {
    delta_time: Numeric,
}
