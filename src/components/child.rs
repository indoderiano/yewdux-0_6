use yew::{
    prelude::*,
    services::{
        ConsoleService,
    },
};
use yewdux::prelude::*;
use std::rc::Rc;
use crate::store::store::{
    State,
    CounterStore,
    CounterOutput,
    CounterInput
};

pub enum Msg {
    State(Rc<State>),
    Output(CounterOutput),
}

pub struct Child {
    state: Rc<State>,
    dispatch: Dispatch<BasicStore<State>>,
    dispatch_send: Dispatch<CounterStore>,
}

impl Component for Child {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let dispatch = Dispatch::bridge_state(link.callback(Msg::State));


        let dispatch_send = {
            let on_state = link.callback(Msg::State);
            let on_output = link.callback(Msg::Output);

            Dispatch::bridge(on_state, on_output)
        };
        // Magically increment counter by one for this example.
        // dispatch_send.send(CounterInput::Increment);

        Self {
            state: Default::default(),
            dispatch,
            dispatch_send,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::State(state) => {
                ConsoleService::info("msg state in child");
                self.state = state;
                true
            }
            Msg::Output(msg) => match msg {
                CounterOutput::Doubled(n) => {
                    ConsoleService::info("msg counter output doubled in component child");
                    // println!("Count doubled would be: {}", n);
                    true
                }
            },
        }
    }

    fn change(&mut self, dispatch: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let count = self.state.count;
        let incr = self.dispatch.reduce_callback(|s| {
            // ConsoleService::info(&format!("state = {:?}", s.count));
            ConsoleService::info("this button will not trigger event Output in this component");
            s.count += 1
        });
        html! {
            <div>
                <h1>{ "CHILD COMPONENT" }</h1>
                <h1>{ count }</h1>
                <button onclick=incr>{"+1"}</button>
            </div>
        }
    }
}