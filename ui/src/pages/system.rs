use crate::app::SwitchProps;
use crate::utils::new_hero;
use tracing::info;
use yew::prelude::*;
use yew_router::prelude::*;

pub enum Msg {}

pub struct System {}

impl Component for System {
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
                { new_hero("System", "All about your cluster", "is-primary") }
                { format!("{:#?}", ctx.props().client.clone().unwrap().inner()) }
            </>
        }
    }
}
