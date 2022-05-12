use crate::app::SwitchProps;
use crate::utils::new_hero;
use yew::prelude::*;

pub enum Msg {}

pub struct Home {}

impl Component for Home {
    type Message = Msg;
    type Properties = SwitchProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        html! {
            <>
                 { new_hero("Home", "Everything in one place", "is-light") }
                 <div class="tile mx-2 mt-2 is-ancestor is-vertical">
                    <div class="tile is-parent">
                        <div class="tile is-child notification is-success">
                            <h1 class="title">{"Welcome"}</h1>
                            {"to "}<strong>{"Kustodio"}</strong>
                            {". The distributed lock-manager."}
                        </div>
                    </div>
                    <div class="tile">
                        <div class="tile is-parent">
                            <div class="tile mr-2 is-child is-6 notification is-info">
                                <p class="title">{"Discover your locks"}</p>
                                <p class="subtile">{"At the current state of time in the system. "}
                                <br/>
                                <a href="/locks">{"here"}</a></p>
                            </div>
                            <div class="is-vertical ml-2 tile is-6 notification is-primary">
                                <p class="title">{"Check the system"}</p>
                                <p class="subtile">{"and it's cluster state. "}
                                <a href="/system">{"here"}</a></p>
                            </div>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}
