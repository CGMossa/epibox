//! Source: [Assignment 6](http://prac.im.pwr.wroc.pl/~szwabin/assets/abm/labs/l6.pdf)

type Numeric = f64;
type NumericVector = f64;

struct Boid {
    position: NumericVector,
    velocity: NumericVector,
    mass: Numeric,
}

/// Also called a "field of view"
struct Neighbourhood {
    heading: NumericVector,
    angle: Numeric,
    distance: Numeric,
}

/// Approximation to [`Neighbourhood`].
struct Sphere {
    radius: Numeric,
    center: NumericVector,
}

enum FlockingRules {
    Separation,
    Cohesion,
    Alignment,
}

enum SteeringRules {
    ObstacleAvoidance,
    GoalSeeking,
}

// Alignment
// Cohesion
// Goal-seeking
// Separation
// Obstacle Avoidance

struct Simulation {
    n: usize,
    boids: Vec<Boid>,
    end_time: f64,
    delta_time: f64,
}

struct BoidsArray {
    force: ndarray::Array2<Numeric>,
    acceleration: ndarray::Array2<Numeric>,
    mass: ndarray::Array1<Numeric>,
    velocity: ndarray::Array2<Numeric>,
    position: ndarray::Array2<Numeric>,
    time: Numeric,
}

impl BoidsArray {
    fn update(&mut self, delta: f64) {
        self.acceleration = self.force / self.mass;
        self.velocity += delta * self.acceleration;
        self.position += delta * self.velocity;

        time += delta;
    }
}
