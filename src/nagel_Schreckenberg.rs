//! Source: [Assignment 5](http://prac.im.pwr.wroc.pl/~szwabin/assets/abm/labs/l5.pdf)

use rand::distributions::Distribution;
use rand::seq::IteratorRandom;
use rand::thread_rng;
use std::fmt::{Display, Error, Formatter};

type CarId = usize;
#[derive(Debug)]
struct Road {
    cars: Vec<Car>,
    road_length: usize,
}

type Position = usize;
type Velocity = usize;
#[derive(Debug, Clone)]
struct Car {
    position: Position,
    velocity: Velocity,
}

impl Car {
    fn new(position: usize, velocity: usize) -> Self {
        Self { position, velocity }
    }
}

impl Road {
    fn new(road_length: usize, cars: usize) -> Self {
        let mut random_positions = (0..road_length).choose_multiple(&mut thread_rng(), cars);
        random_positions.sort();
        let cars = random_positions
            .iter()
            .map(|&position| Car {
                position,
                velocity: 1,
            })
            .collect::<Vec<Car>>();
        Self { cars, road_length }
    }

    fn road(&self) -> Vec<Option<CarId>> {
        let mut road = vec![None; self.road_length];
        for (id, x) in self.cars.iter().enumerate() {
            road[x.position] = Some(id)
        }
        road
    }

    fn next_car(&self, current_car: CarId) -> Option<&Car> {
        if self.cars.len() <= 1 {
            panic!("there are no next car")
        }
        self.cars.iter().cycle().nth(current_car + 1)
    }

    fn average_velocity(&self) -> f64 {
        self.cars.iter().map(|x| x.velocity as f64).sum::<f64>() / self.cars.len() as f64
    }
}

#[test]
fn testing_circular_next_car() {
    let a_road = Road {
        cars: vec![Car::new(0, 1), Car::new(2, 3), Car::new(4, 1)],
        road_length: 8,
    };

    println!("{:?}, where expecting Car(2, 3).", a_road.next_car(0));
    println!("{:?}, where expecting Car(4, 1).", a_road.next_car(1));
    println!("{:?}, where expecting Car(0, 1),", a_road.next_car(2));
}

impl Display for Road {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for x in self.road() {
            write!(
                f,
                "{}",
                match x {
                    None => '_'.to_string(),
                    Some(car_id) => {
                        format!(
                            "{}",
                            self.cars
                                .get::<usize>(car_id)
                                .expect("a car is missing")
                                .velocity
                        )
                    }
                }
            )?;
        }

        Ok(())
    }
}

impl From<&str> for Road {
    fn from(s: &str) -> Self {
        let mut cars = Vec::new();
        let mut road_length = s.trim().len();
        s.trim()
            .char_indices()
            .for_each(|(position, cell)| match cell {
                '_' => {}
                velocity_char @ '0'..='9' => {
                    let velocity = velocity_char.to_digit(10).unwrap() as usize;
                    cars.push(Car { position, velocity });
                }
                _ => panic!("invalid string provided for road"),
            });

        Self { cars, road_length }
    }
}

#[test]
fn testing_road_conversion() {
    println!("Creating roads from string-slices:");
    let roads = "_1________1____1_______1_1____
____1______1______11_____1____
__1_1_________1_____1___1_____
_____1__1_________1____1___1__
1____1________1__1____1_______
___1____1__1_________1____1___
__1______1______1__1__1_______
____1_1___1______1_______1____
____1_______1_____1___1_1_____
____1______1__1___1______1____"
        .split("\n")
        .map(|x: &str| Road::from(x))
        .inspect(|x| println!("{:}", x))
        .collect::<Vec<_>>();
}

#[test]
fn random_placed_cars_on_road() {
    //    let road = Road::new(10, 5);
    println!("Roads of length 25 and 5 randomly placed cars:\n");
    (0..10).for_each(|_| {
        println!("{}\n", Road::new(25, 5));
    })
}

fn maximum_velocity(density: f64) -> usize {
    (density.powi(-1) - 1.).round() as usize
}

fn density(maximum_velocity: usize) -> f64 {
    1. / (maximum_velocity as f64 + 1.)
}

struct Model {
    road: Road,
    timesteps: Vec<Road>,
    max_velocity: Velocity,
    density: f64,
    max_iterations: usize,
    randomisation_probability: f64,
    randomisation_sampler: Box<dyn Fn() -> bool>,
}

impl Model {
    fn update_acceleration(&mut self) {
        for car in &mut self.road.cars {
            if car.velocity < self.max_velocity {
                car.velocity += 1;
            }
        }
    }
    fn update_slowing_down(&mut self) {
        //FIXME: periodic boundary condition: idea? .cycle().take(2 x length). Test that it stops changing...
        let no_cars = self.road.cars.len();
        for car_id in 0..no_cars {
            let car = &self.road.cars[car_id];
            let next_car = &self.road.cars[(car_id + 1) % no_cars];
            let distance = if next_car.position > car.position {
                next_car.position - car.position - 1
            } else {
                (next_car.position + self.road.road_length) - car.position - 1
            };
            if distance < car.velocity {
                self.road.cars[car_id].velocity = distance;
            }
        }
    }
    fn update_randomisation(&mut self) {
        for car in &mut self.road.cars {
            if car.velocity >= 1 && (self.randomisation_sampler)() {
                car.velocity -= 1;
            }
        }
    }
    fn update_motion(&mut self) {
        for x in &mut self.road.cars {
            x.position += x.velocity;
            x.position %= self.road.road_length;
        }
    }

    fn update(&mut self) {
        self.update_acceleration();
        self.update_slowing_down();
        self.update_randomisation();
        self.update_motion();
    }

    fn amend_randomisation_sampler(mut self, randomisation_probability: f64) -> Self {
        //FIXME: move this to the `new`-function to propogate result
        let sampling_dist = rand_distr::Bernoulli::new(randomisation_probability)
            .expect("probability parameter is invalid");
        let sampler = move || sampling_dist.sample(&mut thread_rng());
        self.randomisation_sampler = Box::new(sampler);
        self
    }

    fn new(
        road_length: usize,
        cars: usize,
        randomisation_probability: f64,
        road_dimension: RoadDimension,
        max_iterations: usize,
    ) -> Self {
        let density: f64;
        let max_velocity;

        match road_dimension {
            RoadDimension::Density(den) => {
                density = den;
                max_velocity = maximum_velocity(density);
            }
            RoadDimension::MaximalVelocity(vel) => {
                max_velocity = vel;
                density = self::density(max_velocity);
            }
        }

        //FIXME: move this to the `new`-function to propogate result
        Self {
            road: Road::new(road_length, cars),
            timesteps: vec![],
            max_velocity,
            density,
            max_iterations,
            randomisation_probability,
            randomisation_sampler: Box::new(|| false),
        }
        .amend_randomisation_sampler(randomisation_probability)
    }

    fn run(mut self, _no_saved_iterations: usize) -> Self {
        for _iteration in 0..self.max_iterations {
            self.timesteps.push(Road {
                cars: self.road.cars.clone(),
                road_length: self.road.road_length,
            });
            self.update();
        }
        self
    }
}

enum RoadDimension {
    Density(f64),
    MaximalVelocity(usize),
}

#[test]
fn test_trajectory() {
    let max_iterations = 60;
    let mut simple_model = Model::new(150, 75, 0.5, RoadDimension::MaximalVelocity(13), 100);
    println!("Max. velocity: {:?}", simple_model.max_velocity);
    println!("Density: {:?}", simple_model.density);
    for iteration in 0..max_iterations {
        println!("{:<4}: {:}", iteration, simple_model.road);
        simple_model.update();
    }
}

#[test]
fn test_update() {
    let mut simple_model = Model::new(100, 50, 0.5, RoadDimension::Density(0.6), 100);
    println!("0: {}", simple_model.road);
    simple_model.update_acceleration();
    println!("1: {}", simple_model.road);
    simple_model.update_slowing_down();
    println!("1: {}", simple_model.road);
    simple_model.update_randomisation();
    println!("1: {}", simple_model.road);
    simple_model.update_motion();
    println!("1: {}", simple_model.road);
}
