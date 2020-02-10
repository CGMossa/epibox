type Numeric = f64;
struct Model {
    objective: fn(Numeric) -> Numeric,
    posterior: fn(Numeric) -> Numeric,
    likelihood: fn(Numeric) -> Numeric,
    prior: fn(Numeric) -> Numeric,
    surrogate_function: fn(Numeric) -> Numeric, // response-surface

    max_iterations: u32,
}

impl Model {
    fn run(&self) {
        for _t in (0..self.max_iterations) {

            // find the x_t that is the result of argmax_x{u(x|D[0.._t]}
            //Sample the objective function around x_t
            // Amend x_t and y_t to the gaussian process and update the gaussian process
        }
    }
}

fn gaussian_kernel_numeric(u: Numeric, v: Numeric) -> Numeric {
    ((u - v).powi(2) * (-0.5)).exp()
}

fn gaussian_kernel_vec(u: ndarray::Array1<Numeric>, v: ndarray::Array1<Numeric>) -> Numeric {
    let mut diff: ndarray::Array1<_> = (u - v);
    diff.par_mapv_inplace(|x| x.powi(2));
    diff.sum()
}

fn create_gaussian_kernel(x: ndarray::Array2<Numeric>) -> ndarray::Array2<Numeric> {
    x.stride
}
