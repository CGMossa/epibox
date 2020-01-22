//! A. J. Lotka and V. Volterra
//!  
//!
//!
//!
//! Source [lectures](http://prac.im.pwr.wroc.pl/~szwabin/assets/abm/lec/2.pdf)

type Numeric = f64;

struct Population {
    x: Numeric,
    y: Numeric,
}

/// `x` is prey
/// `y` is predators
/// `t` is time
/// `alpha`, `beta`, `gamma`, and `delta` are species specific parameters.
struct Parameters {
    alpha: Numeric,
    beta: Numeric,
    gamma: Numeric,
    delta: Numeric,
}

struct Model {
    population: Population,
    parameters: Parameters,
}

impl Model {
    fn update(&mut self) {
        let Parameters {
            alpha,
            beta,
            gamma,
            delta,
        } = self.parameters;
        let Population { x, y } = self.population;

        self.population.x += alpha * x - beta * x * y;
        self.population.y += delta * x * y - gamma * y;
    }
}
