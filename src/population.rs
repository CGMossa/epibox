use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};

type Count = f64;
type Rate = f64;

#[derive(Default)]
struct Population {
    count: HashMap<DiseaseCompartment, Count>,
    transitions: HashMap<DiseaseTransition, Box<dyn Fn(Count, Count) -> Rate>>,
    terms: HashMap<DiseaseCompartment, Box<dyn Fn(Count) -> Rate>>,
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
    using: Option<Vec<DiseaseCompartment>>,
    //    dynamic: fn(Count, Count) -> Rate,
}

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
enum DiseaseCompartment {
    Susceptible,
    Exposed, //also Latent
    Infected,
    Recovered,
    Removed,
}

impl Population {
    fn new() -> Self {
        Default::default()
    }
    fn add_compartment(mut self, compartment: DiseaseCompartment, count: Count) -> Self {
        self.count.insert(compartment, count);
        self
    }
    fn add_transition(
        mut self,
        from: DiseaseCompartment,
        to: DiseaseCompartment,
        dynamic: Box<dyn Fn(Count, Count) -> Rate>,
    ) -> Self {
        self.transitions.insert(
            DiseaseTransition {
                from,
                to,
                using: None,
            },
            Box::new(dynamic),
        );
        self
    }

    /// Useful in the case where the closure is not a function of of LEFT -> RIGHT, but other disease
    /// compartments.
    fn update_state_transition(
        mut self,
        from: DiseaseCompartment,
        to: DiseaseCompartment,
        using: &[DiseaseCompartment],
        dynamic: Box<dyn Fn(Vec<Count>) -> Rate>,
    ) -> Self {
        //FIXME: refactor, as it currently does everything
        //        todo!()

        let state: Vec<_> = using
            .iter()
            .map(|x| *self.count.get(x).expect("Disease state does not exist"))
            .collect();
        let mut next_counts = self.count.clone();

        let from_count = *self.count.get(&from).unwrap_or(&Default::default());
        let to_count = *self.count.get(&to).unwrap_or(&Default::default());
        let from_diff = dynamic(state);
        //        let from_diff = boxed_transition(from_count, to_count);
        //let to_diff = -from_diff;

        *next_counts.entry(from).or_default() -= from_diff;
        *next_counts.entry(to).or_default() += from_diff;

        self.count = next_counts;

        todo!()
    }
    /// A better name would be to call this `scalar_term`, as one cannot inject a different
    fn add_term(
        mut self,
        compartment: DiseaseCompartment,
        dynamic: Box<dyn Fn(Count) -> Rate>,
    ) -> Self {
        self.terms.insert(compartment, Box::new(dynamic));
        self
    }

    fn update_disease_states(&mut self) {
        let mut next_counts = self.count.clone();
        // Update transitions
        for (transition, boxed_transition) in self.transitions.iter() {
            let from_count = *self
                .count
                .get(&transition.from)
                .unwrap_or(&Default::default());
            let to_count = *self
                .count
                .get(&transition.to)
                .unwrap_or(&Default::default());
            let from_diff = boxed_transition(from_count, to_count);
            //let to_diff = -from_diff;

            *next_counts.entry(transition.from).or_default() -= from_diff;
            *next_counts.entry(transition.to).or_default() += from_diff;
        }

        // Update individual terms
        for (compartment, dynamic) in self.terms.iter() {
            *next_counts.entry(*compartment).or_default() +=
                dynamic(*self.count.get(&compartment).unwrap_or(&Default::default()))
        }

        self.count = next_counts;
    }

    fn total_population(&self) -> Count {
        self.count
            .iter()
            .fold(Default::default(), |acc, x| match x {
                (&DiseaseCompartment::Removed, _) => acc,
                (_, &count) => acc + count,
            })
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
            Box::new(move |sus, inf| sus * inf * infection_rate),
        )
        .add_transition(
            Infected,
            Recovered,
            Box::new(move |inf, _recover| recovery_rate * inf),
        );

    println!("time {:4} => {:}", 0, sir_population);
    for time in 1..31 {
        sir_population.update_disease_states();
        println!("time {:4} => {:.6}", time, sir_population);
    }
}

#[test]
fn building_a_sirs_model() {
    use DiseaseCompartment::*;

    let infection_rate = 0.02;
    let recovery_rate = 0.5;
    let immunity_decay_rate = 0.05;

    let mut sirs_population = Population::new()
        .add_compartment(Susceptible, 50.)
        .add_compartment(Infected, 1.)
        .add_compartment(Recovered, 0.)
        .add_transition(
            Susceptible,
            Infected,
            Box::new(move |susceptible, infected| infection_rate * susceptible * infected),
        )
        .add_transition(
            Infected,
            Recovered,
            Box::new(move |infected, _recovered| recovery_rate * infected),
        )
        .add_transition(
            Recovered,
            Susceptible,
            Box::new(move |recovered, _susceptible| immunity_decay_rate * recovered),
        );

    println!("time {:4} => {:}", 0, sirs_population);
    for time in 1..51 {
        sirs_population.update_disease_states();
        println!("time {:4} => {:.6}", time, sirs_population);
    }
}

#[test]
fn building_sis_model() {
    use DiseaseCompartment::*;

    let infection_rate = 0.02;
    let recovery_rate = 0.5;

    let mut sis_pop = Population::new()
        .add_compartment(Susceptible, 50.)
        .add_compartment(Infected, 1.)
        .add_transition(
            Susceptible,
            Infected,
            Box::new(move |sus, inf| infection_rate * sus * inf),
        )
        .add_transition(
            Infected,
            Susceptible,
            Box::new(move |inf, _sus| recovery_rate * inf),
        );

    println!("time {:4} => {:}", 0, sis_pop);
    for time in 1..14 {
        sis_pop.update_disease_states();
        println!("time {:4} => {:.6}", time, sis_pop);
    }
}

/// Source: [Survey paper](https://mpra.ub.uni-muenchen.de/68939/1/MPRA_paper_68939.pdf)
#[test]
fn numerical_example_6() {
    use DiseaseCompartment::*;

    let infection_rate = 0.02;
    let recovery_rate = 0.5;

    let mut sis_population = Population::new()
        .add_compartment(Susceptible, 10.)
        .add_compartment(Infected, 1.)
        .add_transition(
            Susceptible,
            Infected,
            Box::new(move |sus, inf| infection_rate * sus * inf),
        )
        .add_transition(
            Infected,
            Susceptible,
            Box::new(move |inf, _sus| recovery_rate * inf),
        );
    println!("time {:4} => {:}", 0, sis_population);
    for time in 1..38 {
        sis_population.update_disease_states();
        println!("time {:4} => {:.6}", time, sis_population);
    }
}

/// Based on Sir model(2) a reference to Tassier (2013) is thrown in there
#[test]
fn building_steady_sir_model() {
    use DiseaseCompartment::*;

    let m = 0.0001;
    let alpha = 0.02;
    let beta = 0.5;
    let delta = 0.1;
    #[allow(non_snake_case)]
    let N = 51.;

    let mut steady_sir_population = Population::new()
        .add_compartment(Susceptible, 50.)
        .add_compartment(Infected, 1.)
        .add_term(Susceptible, Box::new(move |_| m * N))
        .add_transition(Susceptible, Removed, Box::new(move |sus, _rem| m * sus))
        .add_transition(
            Susceptible,
            Infected,
            Box::new(move |sus, inf| alpha * sus * inf),
        )
        .add_transition(Infected, Removed, Box::new(move |inf, _rem| m * inf))
        .add_transition(Infected, Removed, Box::new(move |inf, _rem| beta * inf))
        .add_transition(Infected, Removed, Box::new(move |inf, _rem| delta * inf))
        .add_transition(Infected, Recovered, Box::new(move |inf, _rec| beta * inf))
        .add_transition(Recovered, Removed, Box::new(move |rec, _rem| m * rec));

    println!("time {:4} => {:}", 0, steady_sir_population);
    for time in 1..26 {
        steady_sir_population.update_disease_states();
        println!("time {:4} => {:.6}", time, steady_sir_population);
    }
}

#[test]
fn building_steady_sir_with_hunting_model() {
    use DiseaseCompartment::*;

    let m = 0.0001;
    let alpha = 0.02;
    let beta = 0.5;
    let delta = 0.1;
    let h = 0.1;
    #[allow(non_snake_case)]
    let N = 51.;

    let mut sir_with_hunting = Population::new()
        .add_compartment(Susceptible, 50.)
        .add_compartment(Infected, 1.)
        .add_term(Susceptible, Box::new(move |_| m * N))
        .add_transition(
            Susceptible,
            Removed,
            Box::new(move |sus, _rem| (m + h) * sus),
        )
        .add_transition(
            Susceptible,
            Infected,
            Box::new(move |sus, inf| alpha * sus * inf),
        )
        .add_transition(
            Infected,
            Removed,
            Box::new(move |inf, _rem| (m + delta + h) * inf),
        )
        .add_transition(Infected, Recovered, Box::new(move |inf, _rec| beta * inf))
        .add_transition(Recovered, Removed, Box::new(move |rec, _rem| (m + h) * rec));

    println!("time {:4} => {:}", 0, sir_with_hunting);
    for time in 1..26 {
        sir_with_hunting.update_disease_states();
        println!("time {:4} => {:.6}", time, sir_with_hunting);
    }
}

#[test]
fn building_sei_model() {
    //    let r;
    //    let k;
    //    let N;
    //    let alpha;
    //    let d1;
    //    let delta;
    //    let sigma;

    use DiseaseCompartment::*;

    // Configuration
    let b1 = 0.25;
    let b2 = 0.001;
    let d1 = 0.05;
    let d2 = 0.001;
    let r = 0.2;
    let N = 51.;
    let K = 100.;
    let alpha = 0.02;
    let delta = 0.1;
    let sigma = 0.1;
    let k1 = r * (K - N) / K;
    let k2 = r * N / K;

    //    assert_eq!(k1 + k2, 1.);

    let sei_population = Population::new()
        .add_compartment(Susceptible, 50.)
        .add_compartment(Infected, 1.)
        .add_term(Susceptible, Box::new(move |sus| k1 * sus))
//        .add_transition(Susceptible, Exposed, Box::new(move |sus, exp|))
    ;
}
