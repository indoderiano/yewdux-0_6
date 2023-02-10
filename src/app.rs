use yew::{
    prelude::*,
    services::{
        ConsoleService,
    },
};
use yewdux::prelude::*;
use std::rc::Rc;
use crate::store::test::Test;
use crate::store::store::{
    State,
    CounterStore,
    CounterStoreMsg,
    CounterInput,
    CounterOutput,
};
use crate::components::child::Child;

pub enum Msg {
    State(Rc<State>),
    Output(CounterOutput),
    Update,
}

pub struct App {
    link: ComponentLink<Self>,
    state: Rc<State>,
    dispatch: Dispatch<BasicStore<State>>,
    dispatch_send: Dispatch<CounterStore>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let dispatch = Dispatch::bridge_state(link.callback(Msg::State));

        // How to send
        let dispatch_send = {
            let on_state = link.callback(Msg::State);
            let on_output = link.callback(Msg::Output);

            Dispatch::bridge(on_state, on_output)
        };
        // Magically increment counter by one for this example.
        // dispatch_send.send(CounterInput::Increment);


        Self {
            link,
            state: Default::default(),
            dispatch,
            dispatch_send,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::State(state) => {
                ConsoleService::info("msg state in app");
                ConsoleService::info(&format!("state count now is {:?}", state.count));
                self.state = state;
                true
            }
            Msg::Output(msg) => match msg {
                CounterOutput::Doubled(n) => {
                    ConsoleService::info("msg counter output doubled");
                    // println!("Count doubled would be: {}", n);
                    true
                }
            },
            Msg::Update => {
                self.dispatch_send.send(CounterInput::Increment);
                true
            }
        }
    }

    fn change(&mut self, dispatch: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let count = self.state.count;
        let incr = self.dispatch.reduce_callback(|s| {
            s.count = 3
        });
        let update = self.link.callback(|_| {
            ConsoleService::info("this button will trigger event Output in this component");
            Msg::Update
        });
        html! {
            <div>
                <h1>{ "YEWDUX STORE" }</h1>
                <Test/>
                <h1>{ count }</h1>
                <button onclick=incr>{"3"}</button>
                <button onclick=update>{ "update" }</button>

                // <Child/>
            </div>
        }
    }
}