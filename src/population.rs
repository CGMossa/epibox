use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};

type Count = f64;
type Rate = f64;

///
/// This turned out to be a failed experiment. The expressions in the equation changes value, when
/// we are to calculate them, due to them being coupled.
///

#[derive(Default)]
struct Population {
    count: HashMap<DiseaseCompartment, Count>,
    //    transitions: Vec<DiseaseTransition>,
    transitions: HashMap<DiseaseTransition, Box<dyn Fn(Count, Count) -> Rate>>,
    transition_parameters: HashMap<(DiseaseCompartment, DiseaseCompartment), DiseaseParameter>,
}

#[derive(Debug)]
enum DiseaseParameter {
    Rate(Rate),
    Probability(f64),
    Count(u64),
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct DiseaseTransition {
    from: DiseaseCompartment,
    to: DiseaseCompartment,
    //    dynamic: fn(Count, Count) -> Rate,
}

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
enum DiseaseCompartment {
    Susceptible,
    Exposed, //also Latent
    Infected,
    Recovered,
    //    Removed,
}

impl Population {
    fn new() -> Self {
        Default::default()
    }
    fn add_compartment(mut self, new_compartment: DiseaseCompartment, count: Count) -> Self {
        self.count.insert(new_compartment, count);
        self
    }
    fn add_transition(
        mut self,
        from: DiseaseCompartment,
        to: DiseaseCompartment,
        dynamic: Box<dyn Fn(Count, Count) -> Rate>,
    ) -> Self {
        self.transitions
            .insert(DiseaseTransition { from, to }, Box::new(dynamic));
        self
    }

    fn update_disease_states(&mut self) {
        let mut next_counts = self.count.clone();
        for (transition, boxed_transition) in self.transitions.iter() {
            let from_count = *self.count.get(&transition.from).unwrap();
            let to_count = *self.count.get(&transition.to).unwrap();
            let from_diff = boxed_transition(from_count, to_count);
            //            let to_diff = -from_diff;

            //            dbg!((&transition.from, &transition.to));
            //            dbg!((from_count, to_count), (from_diff, -from_diff));

            *next_counts.entry(transition.from).or_default() = from_count + from_diff;
            *next_counts.entry(transition.to).or_default() = to_count - from_diff;
        }
        self.count = next_counts;
    }
}

impl Display for Population {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:.6?}", self.count)?;

        Ok(())
    }
}

#[test]
fn building_a_sir_population() {
    use DiseaseCompartment::*;

    let infection_rate = 0.02;
    let recovery_rate = 0.5;

    let sir_population = Population::new()
        .add_compartment(Susceptible, 50.)
        .add_compartment(Infected, 1.)
        .add_compartment(Recovered, 0.);
    let mut sir_population = sir_population
        .add_transition(
            Susceptible,
            Infected,
            Box::new(move |sus, inf| -sus * inf * infection_rate),
        )
        .add_transition(
            Infected,
            Recovered,
            Box::new(move |inf, _recover| -recovery_rate * inf),
        );

    println!("{}", sir_population);
    for time in 0..5 {
        sir_population.update_disease_states();
        println!("time {} => {:.6}", time, sir_population);
    }
}
