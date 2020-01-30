//! Source: [Assignment 5](http://prac.im.pwr.wroc.pl/~szwabin/assets/abm/labs/l5.pdf)

use rand::distributions::Distribution;
use rand::seq::IteratorRandom;
use rand::thread_rng;
use std::fmt::{Display, Error, Formatter};
use std::iter::once;

/// Circular buffer
#[derive(Debug)]
struct Road {
    cells: Vec<Cell>,
}

#[derive(Copy, Clone, Debug)]
struct Cell {
    car: Option<Car>,
}

const MAX_VELOCITY: Velocity = 5;

type Velocity = usize;
#[derive(Copy, Clone, Debug)]
struct Car {
    velocity: Velocity,
}

impl Road {
    fn new(road_length: usize, cars: usize) -> Self {
        let car_vel_one = Car { velocity: 1 };

        let mut cells = vec![Cell { car: None }; road_length];

        for cell in cells.iter_mut().choose_multiple(&mut thread_rng(), cars) {
            cell.car = Some(car_vel_one);
        }

        Self { cells }
    }

    fn average_velocity(&self) {
        todo!()
    }
}

impl Display for Road {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for x in &self.cells {
            write!(
                f,
                "{}",
                match x.car {
                    None => format!("_"),
                    Some(car) => {
                        format!("{}", car.velocity)
                    }
                }
            )?;
        }

        Ok(())
    }
}

#[test]
fn random_placed_cars_on_road() {
    //    let road = Road::new(10, 5);
    println!("Roads of length 25 and 5 randomly placed cars:\n");
    (0..10).for_each(|_| {
        println!("{}\n", Road::new(25, 5));
    })
}

#[derive(Debug)]
struct Model {
    road: Road,
    timesteps: Vec<Road>,
    randomisation_probability: f64,
}

impl Model {
    fn update_acceleration(&mut self) {
        for cell in &mut self.road.cells {
            match cell.car {
                None => {}
                Some(ref mut car) => {
                    if car.velocity < MAX_VELOCITY {
                        car.velocity += 1;
                    }
                }
            }
        }
    }
    fn update_slowing_down(&mut self) {
        //FIXME: periodic boundary condition: idea? .cycle().take(2 x length). Test that it stops changing...
        let cars_iterator = self
            .road
            .cells
            .iter_mut()
            .enumerate()
            .filter(|x| x.1.car.is_some());
        let mut cars = cars_iterator.peekable();
        while let Some((position, cell)) = cars.next() {
            let current_car = cell.car.as_mut().unwrap();
            match cars.peek() {
                None => {}
                Some((next_position, _next_cell)) => {
                    let distance = next_position - position - 1;
                    if distance < current_car.velocity {
                        current_car.velocity = distance;
                    }
                }
            }
        }
    }
    fn update_randomisation(&mut self) {
        //FIXME: move this to the `new`-function to propogate result
        let sampling_dist = rand_distr::Bernoulli::new(self.randomisation_probability)
            .expect("probability parameter is invalid");
        let sampler = || sampling_dist.sample(&mut thread_rng());

        for cell in self.road.cells.iter_mut() {
            match cell.car {
                None => {}
                Some(ref mut car) => {
                    if car.velocity >= 1 && sampler() {
                        //println!("Reduction!");
                        car.velocity -= 1;
                    }
                }
            }
        }
    }
    fn update_motion(&mut self) {
        let mut new_cells = vec![Cell { car: None }; self.road.cells.len()];

        for (pos, x) in self.road.cells.iter().enumerate() {
            match &x.car {
                None => {}
                Some(car) => {
                    new_cells.insert((pos + car.velocity) % self.road.cells.len(), *x);
                }
            }
        }
        self.road.cells = new_cells;
    }

    fn new(road_length: usize, cars: usize, randomisation_probability: f64) -> Self {
        assert!(cars < road_length);

        Self {
            road: Road::new(road_length, cars),
            timesteps: vec![],
            randomisation_probability,
        }
    }
}

#[test]
fn test_update() {
    let mut simple_model = Model {
        road: Road {
            cells: vec![
                Cell { car: None },
                Cell {
                    car: Some(Car { velocity: 4 }),
                },
                Cell { car: None },
                Cell { car: None },
                Cell { car: None },
                Cell {
                    car: Some(Car { velocity: 2 }),
                },
            ],
        },
        timesteps: vec![],
        randomisation_probability: 0.5,
    };

    println!("{:#?}", simple_model);
    simple_model.update_acceleration();
    simple_model.update_slowing_down();
    simple_model.update_randomisation();
    simple_model.update_motion();
    println!("{:#?}", simple_model);
}
