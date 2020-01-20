use std::fmt::{Display, Error, Formatter};

type Count = f64;
type Rate = f64;

#[derive(Clone)]
struct DiseaseState {
    susceptible: Count,
    infected: Count,
    recovered: Count,
}

#[derive(Clone)]
struct PopulationState {
    time: u64,
    state: DiseaseState,
}

struct Population {
    population: Count,
}

struct SteadyStateSIRModelParameters {
    m: Rate,
    alpha: Rate,
    beta: Rate,
    delta: Rate,
}

struct SteadyStateSIRModel {
    parameters: SteadyStateSIRModelParameters,
    initial_population: Population,
    states: Vec<PopulationState>,
}

impl SteadyStateSIRModel {
    fn set_disease_parameters(mut self, disease_parameters: SteadyStateSIRModelParameters) -> Self {
        self.parameters = disease_parameters;
        self
    }

    fn new(susceptible: Count, infected: Count) -> Self {
        Self {
            parameters: SteadyStateSIRModelParameters {
                m: 0.0,
                alpha: 0.0,
                beta: 0.0,
                delta: 0.0,
            },
            initial_population: Population {
                population: susceptible + infected,
            },
            states: vec![PopulationState {
                time: 0,
                state: DiseaseState {
                    susceptible,
                    infected,
                    recovered: 0.0,
                },
            }],
        }
    }

    fn update(&mut self, timesteps: u64) {
        let SteadyStateSIRModelParameters {
            m,
            alpha,
            beta,
            delta,
        } = self.parameters;
        #[allow(non_snake_case)]
        let N = self.initial_population.population;
        let PopulationState {
            time,
            state:
                DiseaseState {
                    mut susceptible,
                    mut infected,
                    mut recovered,
                },
        } = self
            .states
            .last()
            .cloned()
            .expect("failed to initialise population");

        for time_increment in 1..=timesteps {
            let diff_susceptible = m * N - m * susceptible - alpha * susceptible * infected;
            let diff_infected = alpha * susceptible * infected - (m + delta + beta) * infected;
            let diff_recovered = beta * infected - m * recovered;

            susceptible += diff_susceptible;
            infected += diff_infected;
            recovered += diff_recovered;

            self.states.push(PopulationState {
                time: time + time_increment,
                state: DiseaseState {
                    susceptible,
                    infected,
                    recovered,
                },
            })
        }
    }
}

impl Display for Population {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Population size = {}", self.population)?;
        Ok(())
    }
}

impl Display for PopulationState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "time = {:>3} ", self.time)?;
        write!(f, "{}", self.state)?;
        Ok(())
    }
}

impl Display for DiseaseState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "Sus. = {:<10.5} Inf. = {:<10.5} Rec. = {:<10.5}",
            self.susceptible, self.infected, self.recovered
        )?;
        Ok(())
    }
}

impl Display for SteadyStateSIRModelParameters {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "alpha = {}, beta = {}, delta = {}, m = {}",
            self.alpha, self.beta, self.delta, self.m
        )?;

        Ok(())
    }
}

impl Display for SteadyStateSIRModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "{}", self.parameters)?;
        writeln!(f, "{}", self.initial_population)?;
        //        write!(f, "{}", self.states)?;
        for x in &self.states {
            writeln!(f, "{}", x)?;
        }
        Ok(())
    }
}

/// Source: [](https://mpra.ub.uni-muenchen.de/68939/1/MPRA_paper_68939.pdf)
#[test]
fn numerical_example_7_of_sir_model_2() {
    let mut model =
        SteadyStateSIRModel::new(50., 1.).set_disease_parameters(SteadyStateSIRModelParameters {
            m: 0.0001,
            alpha: 0.02,
            beta: 0.5,
            delta: 0.1,
        });

    model.update(13);

    println!("{}", model);
}
