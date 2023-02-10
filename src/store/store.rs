use std::rc::Rc;
use yewdux::prelude::*;

#[derive(Clone)]
pub struct State {
    pub count: u32,
}

impl Default for State {
    fn default() -> State {
        State {
            count: 0,
        }
    }
}

pub enum CounterStoreMsg {
    Update,
}

pub enum CounterInput {
    Increment
}

pub enum CounterOutput {
    Doubled(u32)
}

pub struct CounterStore {
    state: Rc<State>,
    link: StoreLink<Self>,
}

impl Store for CounterStore {
    type Model = State;
    type Message = CounterStoreMsg;
    type Input = CounterInput;
    type Output = CounterOutput;

    fn new(link: StoreLink<Self>) -> Self {
        Self {
            link,
            state: Rc::new(State { count: 0 }),
        }
    }

    fn state(&mut self) -> &mut Rc<Self::Model> {
        &mut self.state
    }

    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) -> Changed {
        let state = Rc::make_mut(&mut self.state);
        match msg {
            CounterInput::Increment => {
                state.count += 1;
                // Response with current count doubled.
                self.link.respond(who, CounterOutput::Doubled(state.count * 2));
            }
        }

        true
    }


    // HAVE NOT FIGURED OUT HOW FN UPDATE WORKS
    fn update(&mut self, msg: Self::Message) -> Changed {
        match msg {
            CounterStoreMsg::Update => {
                let state = State {
                    count: 100,
                };
                self.state = Rc::new(state);
            }
        }
        true
    }
}