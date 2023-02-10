use yew::prelude::*;

// enum Msg {
//     AddOne,
// }

pub struct Test {
    value: i64,
}

impl Component for Test {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        // match msg {
        //     Msg::AddOne => {
        //         self.value += 1;
        //         // the value has changed so we need to
        //         // re-render for it to appear on the page
        //         true
        //     }
        // }
        false
    }

    fn change(&mut self, dispatch: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{ "TEST" }</h1>
            </div>
        }
    }
}