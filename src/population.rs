use std::collections::HashMap;

type Count = u64;
type Rate = f64;

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

#[derive(Hash, PartialEq, Eq, Debug)]
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
        for (transition, boxed_transition) in self.transitions.iter() {
            let from_count = self.count.get(&transition.from);
            let to_count = self.count.get(&transition.to);
            let dynamic = boxed_transition.call((from_count, to_count));
        }
    }
}

#[test]
fn building_a_sir_population() {
    use DiseaseCompartment::*;

    let infection_rate = 0.02;
    let recovery_rate = 0.5;

    let sir_population = Population::new()
        .add_compartment(Susceptible, 0)
        .add_compartment(Infected, 0)
        .add_compartment(Recovered, 0);
    let sir_population = sir_population
        .add_transition(
            Susceptible,
            Infected,
            Box::new(move |sus, inf| -((sus * inf) as Rate) * infection_rate),
        )
        .add_transition(
            Infected,
            Recovered,
            Box::new(move |inf, recover| -recovery_rate * (inf as f64)),
        );

    sir_population.update_disease_states();
    //    println!("{:#?}", sir_population);
}
