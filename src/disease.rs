use std::fmt::{Display, Error, Formatter};

//type Count = u64;
type Count = f64;
type Rate = f64;

struct Population {
    disease_compartments: DiseaseStates,
    infection_rate: Rate,
    recovery_rate: Rate,
}

struct DiseaseCompartments<T> {
    susceptible: T,
    exposed: T,
    infectious: T,
    removed: T,
    recovered: T,
}

type DiseaseStates = DiseaseCompartments<Count>;
type DiseaseRates = DiseaseCompartments<Rate>;

impl DiseaseStates {
    fn update_disease(&mut self, disease_rate: DiseaseRates) {
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

    fn total(&self) -> Count {
        self.susceptible + self.exposed + self.infectious + self.removed + self.recovered
    }
}

impl Population {
    fn disease_rate(&self) -> DiseaseRates {
        let newly_infected_rate = self.infection_rate
            * ((self.disease_compartments.susceptible * self.disease_compartments.infectious)
                as f64);
        let recovery_and_removed =
            self.recovery_rate * (self.disease_compartments.infectious as f64);
        DiseaseRates {
            susceptible: -newly_infected_rate,
            exposed: 0.0,
            infectious: newly_infected_rate - recovery_and_removed,
            removed: recovery_and_removed,
            recovered: recovery_and_removed,
        }
    }

    fn reproduction_number(&self) -> Rate {
        (self.infection_rate * (self.disease_compartments.susceptible as Rate)) / self.recovery_rate
    }

    fn update_disease_state(&mut self) {
        self.disease_compartments
            .update_disease(self.disease_rate());
    }
}

impl Display for Population {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.disease_compartments)?;
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

#[test]
/// Source: https://mpra.ub.uni-muenchen.de/68939/1/MPRA_paper_68939.pdf
fn numerical_example_2() {
    let mut population = Population {
        disease_compartments: DiseaseStates {
            susceptible: 50.into(),
            exposed: 0.into(),
            //            infectious: 0,
            infectious: 1.into(),
            removed: 0.into(),
            recovered: 0.into(),
        },
        infection_rate: 0.02,
        recovery_rate: 0.5,
    };

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
