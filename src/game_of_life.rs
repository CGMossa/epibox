//! Source: [Assignment 3](http://prac.im.pwr.wroc.pl/~szwabin/assets/abm/labs/l3.pdf)
struct Grid2D;

type Grid2DIdx = i32;

struct Universe {
    grid2d: Grid2D,
}

enum State {
    Alive,
    Dead,
}

enum Transitions {
    Underpopulation,
    Overpopulation,
    Reproduction,
}

impl Universe {
    /// Source: [Lecture 3](http://prac.im.pwr.wroc.pl/~szwabin/assets/abm/lec/3.pdf)
    fn create_block(&self, x_start: Grid2DIdx, y_start: Grid2DIdx) {
        for x in x_start..x_start + 2 {
            for y in y_start..y_start + 2 {
                self.set_state(x, y, State::Alive)
            }
        }
    }

    fn set_state(x: Grid2DIdx, y: Grid2DIdx, state: State) {
        todo!()
    }
}

mod patterns {

    mod still_life {

        fn create_beehive() {
            todo!()
        }

        fn create_loaf() {
            todo!()
        }

        fn create_boat() {
            todo!()
        }

        fn create_tube() {
            todo!()
        }
    }

    mod oscillators {
        /// Period 2
        fn create_blinker() {
            todo!()
        }
        /// Period 2
        fn create_toad() {
            todo!()
        }
        /// Period 2
        fn create_beacon() {
            todo!()
        }
        /// Period 3
        fn create_pulsar() {
            todo!()
        }
        /// Period 15
        fn create_pentadecathlon() {
            todo!()
        }
    }

    mod glider {
        fn create_glider() {
            todo!()
        }
        fn create_spaceship() {
            todo!()
        }
    }

    mod gun {
        fn create_gun() {
            todo!()
        }
    }
}
