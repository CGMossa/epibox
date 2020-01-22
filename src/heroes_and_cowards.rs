//! Source: [Lecture 4](http://prac.im.pwr.wroc.pl/~szwabin/assets/abm/lec/3.pdf)

struct Universe {
    agents: Vec<Agent>,
}

impl Universe {
    fn new() {
        todo!()
        // Initialize:
        //  Create NUMBER agents
        //  Move each agent to a random location
        //  If “hero” personality chosen, each agent turns blue
        //  If “coward” personality chosen, each agent turns red
        //  If “mixed” personality chosen, color each agent red or blue at random
        //  Each agent picks one other agent as friend
        //  Each agent picks one other agent as enemy
        //  Start the clock
    }

    fn update() {
        todo!()
        // At each tick:
        //  Each blue agent moves a step towards a location between his
        //  friend and its enemy
        //  Each red agent moves a step towards a location that puts his friend
        //      between him and his enemy
    }

    // From lecture:
    //    universe – a plane
    //    ● agents – people being in one of two states: brave or cowardly
    //    ● initial state:
    //    ● all brave
    //    ● all cowards
    //    ● mixed population
    //    ● random positions
    //    ● rules:
    //    ● if brave, move toward the midpoint of your friend and enemy
    //    ● if a coward, put your friend between you and your enemy
    //    ● time evolution – in every tick check the state of the agent and act
    //    accordingly
}

struct Agent {
    state: State,
}

enum State {
    Brave,
    Cowardly,
}

enum Perceptions {
    Friend,
    Enemy,
}
