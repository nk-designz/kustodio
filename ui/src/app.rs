use crate::{client::Client, pages::*};
use anyhow::Error;
use tracing::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

pub enum Msg {
    ClientInit(Result<Client, String>),
}

#[derive(Clone, Routable, PartialEq, Debug)]
enum Route {
    #[at("/")]
    Home,
    #[at("/locks")]
    Locks,
    #[at("/system")]
    System,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(PartialEq, Clone)]
pub struct SwitchProps {
    client: Client,
}

fn switch(_props: SwitchProps) -> impl Fn(&Route) -> Html {
    move |route| {
        info!("Switching to route {:?}", route);
        match route {
            Route::Home => html! { <Home  /> },
            Route::NotFound => html! { <NotFound /> },
            Route::Locks => html! { <Locks /> },
            Route::System => html! { <System /> },
        }
    }
}

#[derive(Default)]
pub struct App {
    client: Option<Client>,
    error: Option<Error>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        spawn_local(async move {
            info!("Creating client");
            let client = match Client::auto().await {
                Ok(client) => Ok(client.clone()),
                Err(err) => Err(err.to_string()),
            };
            link.callback(move |_: Msg| Msg::ClientInit(client.to_owned()));
        });
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ClientInit(client_result) => match client_result {
                Ok(client) => {
                    self.client = Some(client);
                    info!("Client initialized: {:?}", self.client);
                    true
                }
                Err(err) => {
                    info!("Error in creating client");
                    self.error = Some(Error::msg(err));
                    true
                }
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let _link = ctx.link();

        html! {
            <>
                <nav class="navbar is-black" role="navigation" aria-label="main navigation">
                    <div class="navbar-brand">
                        <a class="navbar-item" href="https://github.com/nk-designz/kustodio">
                            <h1><b>{"Kustodio"}</b></h1>
                        </a>
                    </div>
                    <div id="navbar-menu" class="navbar-menu">
                        <div class="navbar-start">
                            <a class="navbar-item" href="/">{"Home"}</a>
                            <a class="navbar-item" href="/locks">{"Locks"}</a>
                            <a class="navbar-item" href="/system">{"System"}</a>
                        </div>
                    </div>
                </nav>
                { if self.error.is_some() {
                    let props = SwitchProps {
                        client: self.client.clone().unwrap(),
                    };
                    html!{
                        <BrowserRouter>
                            <Switch<Route> render={Switch::render(switch(props))} />
                        </BrowserRouter>
                    }
                } else {
                    html!{
                        <>{"Error"}</>
                    }
                }
                }
            </>
        }
    }
}
