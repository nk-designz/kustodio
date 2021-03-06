use crate::{client::Client, pages::*};
use anyhow::Error;
use stylist::{css, style, yew::Global};
use tracing::*;
use wasm_bindgen_futures::spawn_local;
use yew::{prelude::*, Properties};
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

#[derive(PartialEq, Clone, Properties)]
pub struct SwitchProps {
    #[prop_or_default]
    pub client: Option<Client>,
}

fn switch(props: SwitchProps) -> impl Fn(&Route) -> Html {
    move |route| {
        info!("Switching to route {:?}", route);
        match route {
            Route::Home => html! { <Home ..props.clone()  /> },
            Route::NotFound => html! { <NotFound /> },
            Route::Locks => html! { <Locks ..props.clone() /> },
            Route::System => html! { <System ..props.clone() /> },
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
            info!("Created client");
            link.callback(move |_: ()| Msg::ClientInit(client.to_owned()))
                .emit(());
        });
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ClientInit(client_result) => match client_result {
                Ok(client) => {
                    self.client = Some(client);
                    self.error = None;
                    info!("Client initialized: {:?}", self.client);
                    true
                }
                Err(err) => {
                    info!("Error in creating client: {}", err);
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
                <Global css={r#"
                    body {
                        min-height: 100vh;
                    }
                "#}></Global>
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

                { if self.error.is_none() && self.client.is_some() {
                    let props = SwitchProps {
                        client: self.client.clone(),
                    };

                    html!{
                        <BrowserRouter>
                            <Switch<Route> render={
                                Switch::render(switch(props))
                            }/>
                        </BrowserRouter>
                    }
                } else {
                    html!{
                        <div class={css!(r#"
                                @keyframes loading {
                                    0%   {height: 10vw; width: 10vw;}
                                    50% {height: 20vw; width: 20vw;}
                                    100% {height: 10vw; width: 10vw;}
                                }
                                border: 10px solid lightgrey;
                                background-color: white;
                                border-radius: 50%;
                                position: absolute;
                                left: 50%;
                                top: 50%;
                                transform: translate(-50%, -50%);
                                animation: loading 2s infinite;
                        "#)}></div>
                    }
                }
                }
                <footer class={format!("footer {}", style!(r#"
                    position: absolute;
                    width: 100vw;
                    bottom: 0;
                    left: 0;
                "#).unwrap().get_class_name())}>
                    <div class="content has-text-centered">
                        <p>
                            <strong>{"Kustodio"}</strong>{" by "}<a href="https://github.com/nk-designz">{"nk-designz"}</a>{"."}
                        </p>
                    </div>
                </footer>
            </>
        }
    }
}
