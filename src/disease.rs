use std::fmt::{Display, Error, Formatter};

//type Count = u64;
type Count = f64;
type Rate = f64;

#[derive(Default)]
pub struct Population {
    disease_states: DiseaseStates,
    infection_rate: Rate,
    recovery_rate: Rate,
    immunity_decay_rate: Rate,
}

#[derive(Default)]
struct DiseaseCompartments<T: Default> {
    susceptible: T,
    exposed: T,
    infectious: T,
    removed: T,
    recovered: T,
}

type DiseaseStates = DiseaseCompartments<Count>;
type DiseaseRates = DiseaseCompartments<Rate>;

impl DiseaseStates {
    pub fn update_disease(&mut self, disease_rate: DiseaseRates) {
        let DiseaseRates {
            susceptible,
            exposed,
            infectious,
            removed,
            recovered,
        } = disease_rate;
        self.susceptible = (self.susceptible as Rate + susceptible) as Count;
        self.exposed += exposed as Rate;
        self.infectious += infectious as Rate;
        self.removed += removed as Rate;
        self.recovered += recovered as Rate;
    }

    pub fn total(&self) -> Count {
        self.susceptible + self.exposed + self.infectious + self.removed + self.recovered
    }
}

impl Population {
    fn disease_rate(&self) -> DiseaseRates {
        let newly_susceptible = self.immunity_decay_rate * self.disease_states.recovered;
        let newly_infected_rate = self.infection_rate
            * ((self.disease_states.susceptible * self.disease_states.infectious) as Rate);
        let recovery_and_removed = self.recovery_rate * (self.disease_states.infectious as Rate);
        DiseaseRates {
            susceptible: -newly_infected_rate + newly_susceptible,
            exposed: 0.0,
            infectious: newly_infected_rate - recovery_and_removed,
            removed: recovery_and_removed - newly_susceptible,
            recovered: recovery_and_removed - newly_susceptible,
        }
    }

    /// Reproduction number according to SIR-model. Should be taken with a grain of salt.
    fn reproduction_number(&self) -> Rate {
        (self.infection_rate * (self.disease_states.susceptible as Rate)) / self.recovery_rate
    }

    fn update_disease_state(&mut self) {
        self.disease_states.update_disease(self.disease_rate());
    }
}

impl Display for Population {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.disease_states)?;
        write!(f, "{:>10.6}", self.reproduction_number())?;
        Ok(())
    }
}

impl Display for DiseaseStates {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{:>10.6} {:>10.6} {:>10.6}",
            self.susceptible, self.infectious, self.removed
        )?;

        Ok(())
    }
}
/// SIR Model
///
/// Creates a [`Population`] that simulates the dynamics in a SIR model.
///
pub fn create_sir_population(
    initial_susceptible_population: Count,
    initial_infected: Count,
    infection_rate: Rate,
    recovery_rate: Rate,
) -> Population {
    // FIXME: how do you ensure that a call to update_disease does a SIR update?
    Population {
        disease_states: DiseaseCompartments {
            susceptible: initial_susceptible_population,
            infectious: initial_infected,
            ..Default::default()
        },
        infection_rate,
        recovery_rate,
        immunity_decay_rate: 0.0,
    }
}

#[test]
/// Source: https://mpra.ub.uni-muenchen.de/68939/1/MPRA_paper_68939.pdf
fn numerical_example_2() {
    let mut population = create_sir_population(50., 1., 0.02, 0.5);
    println!(
        "{:<10} {:10} {:10} {:10} {:10}",
        "time", "Susceptible", "Infectious", "Removed", "Reproduction"
    );
    for time in 0..14 {
        print!("{:<5}", time);
        println!("{}", population);
        population.update_disease_state();
    }
}

fn create_sirs_population(
    initial_susceptible_population: Count,
    initial_infected: Count,
    infection_rate: Rate,
    recovery_rate: Rate,
    immunity_decay_rate: Rate,
) -> Population {
    Population {
        disease_states: DiseaseStates {
            susceptible: initial_susceptible_population,
            infectious: initial_infected,
            ..Default::default()
        },
        infection_rate,
        recovery_rate,
        immunity_decay_rate,
    }
}

#[test]
fn numerical_example_4() {
    let mut sirs_pop = create_sirs_population(50., 1., 0.02, 0.5, 0.05);

    //    let simulation = Simulation::new(sirs_pop);
    println!(
        "{:<10} {:10} {:10} {:10} {:10}",
        "time", "Susceptible", "Infectious", "Removed", "Reproduction"
    );
    for time in 0..14 {
        print!("{:<5}", time);
        println!("{}", sirs_pop);
        sirs_pop.update_disease_state();
    }
}

//use enum_iterator::IntoEnumIterator;
//use std::collections::HashSet;
//use std::hash::Hash;

//#[derive(Eq, PartialEq, Hash)]
//enum Compartment<T> {
//    Susceptible(T),
//    Exposed(T),
//    Infectious(T),
//    Removed(T),
//    Recovered(T),
//}
//
//enum TransitionParameter {
//    Count(u64),
//    Rate(f64),
//    Probability(f64),
//}
//
//enum CompartmentName {
//    Susceptible,
//    Exposed,
//    Infectious,
//    Removed,
//    Recovered,
//}
//
//struct Transition {
//    from: Compartment<u64>,
//    to: Compartment<u64>,
//    parameter: TransitionParameter,
//}
//
//struct DiseasePopulation {
//    //    compartments: Vec<Compartment<Count>>,
//    compartments: HashSet<Compartment<u64>>,
//    transitions: Vec<Transition>,
//}
//
//impl DiseasePopulation {
//    fn new() -> Self {
//        Self {
//            compartments: Default::default(),
//            transitions: Default::default(),
//        }
//    }
//    fn add_compartment(mut self, new_compartment: Compartment<u64>) -> Self {
//        self.compartments.insert(new_compartment);
//        self
//    }
//    fn add_transition(mut self, transition: Transition) -> Self {
//        todo!()
//        //        self.transitions.insert(
//        //            |sus, inf| - alpha * sus * inf
//        //        )
//    }
//    fn update_transition(mut self) -> Self {
//        for x in self.transitions {}
//        todo!()
//    }
//}
//
//#[test]
//fn example_of_building_populations() {
//    let sir_model = DiseasePopulation::new()
//        .add_compartment(Compartment::Susceptible(0))
//        .add_compartment(Compartment::Infectious(0))
//        .add_compartment(Compartment::Recovered(0));
//    //    let sir_model = sir_model.add_transition()
//
//    assert!(false);
//}

//pub fn create_sis_population(
//    initial_susceptibles: Count,
//    initial_infected: Count,
//    infection_rate: Rate,
//    recovery_rate: Rate,
//) -> Population {
//    Population {
//        disease_states: DiseaseStates {
//            susceptible: initial_susceptibles,
//            infectious: initial_infected,
//            ..Default::default()
//        },
//        infection_rate,
//        recovery_rate,
//        ..Default::default()
//    }
//}

//#[test]
//fn numerical_example_5() {
//    let mut sis_pop = create_sis_population(50., 1., 0.02, 0.5);
//
//    println!(
//        "{:<10} {:10} {:10} {:10} {:10}",
//        "time", "Susceptible", "Infectious", "Removed", "Reproduction"
//    );
//    for time in 0..14 {
//        print!("{:<5}", time);
//        println!("{}", sis_pop);
//        sis_pop.update_disease_state();
//    }
//}

//
//struct DiseaseDerivative {
//    susceptible: Rate,
//    exposed: Rate,
//    infectious: Rate,
//    removed: Rate,
//    recovered: Rate,
//}
//
//impl From<DiseasePopulations> for DiseaseDerivative {
//    fn from(population: DiseasePopulations) -> Self {
//        Self {
//            susceptible: 0.0,
//            exposed: 0.0,
//            infectious: 0.0,
//            removed: 0.0,
//            recovered: 0.0,
//        }
//    }
//}
//

//enum Compartment {
//    Susceptible(u64),
//    Exposed(u64),
//    Infectious(u64),
//    Removed(u64),
//    Recovered(u64),
//}

//enum TransitionParameter {
//    Count(u64),
//    Rate(f64),
//    Probability(f64),
//}

//struct Transition {
//    from: Compartment,
//    to: Compartment,
//    parameter: TransitionParameter,
//}

//impl Transition {
//    fn update(&self) {
//        match self.parameter {
//            TransitionParameter::Count(_) => todo!(),
//            TransitionParameter::Probability(_) => todo!(),
//            TransitionParameter::Rate(beta) => match self.from {
//                Compartment::Susceptible(_) => match self.to {
//                    Compartment::Susceptible(_) => {}
//                    Compartment::Exposed(_) => {}
//                    Compartment::Infectious(_) => {}
//                    Compartment::Removed(_) => {}
//                    Compartment::Recovered(_) => {}
//                },
//                Compartment::Exposed(_) => match self.to {
//                    Compartment::Susceptible(_) => {}
//                    Compartment::Exposed(_) => {}
//                    Compartment::Infectious(_) => {}
//                    Compartment::Removed(_) => {}
//                    Compartment::Recovered(_) => {}
//                },
//                Compartment::Infectious(_) => match self.to {
//                    Compartment::Susceptible(_) => {}
//                    Compartment::Exposed(_) => {}
//                    Compartment::Infectious(_) => {}
//                    Compartment::Removed(_) => {}
//                    Compartment::Recovered(_) => {}
//                },
//                Compartment::Removed(_) => {}
//                Compartment::Recovered(_) => match self.to {
//                    Compartment::Susceptible(_) => {}
//                    Compartment::Exposed(_) => {}
//                    Compartment::Infectious(_) => {}
//                    Compartment::Removed(_) => {}
//                    Compartment::Recovered(_) => {}
//                },
//            },
//        }
//    }
//}
