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

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <>
                 { new_hero("Home", "Everything in one place", "is-light") }
            </>
        }
    }
}
