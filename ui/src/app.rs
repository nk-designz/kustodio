use crate::pages::*;
use tracing::info;
use yew::prelude::*;
use yew_router::prelude::*;

pub enum Msg {}

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

fn switch(routes: &Route) -> Html {
    info!("Switching to route {:?}", routes);
    match routes {
        Route::Home => html! { <Home/> },
        Route::NotFound => html! { <NotFound/> },
        Route::Locks => html! { <Locks/> },
        Route::System => html! { <System/> },
    }
}

pub struct App {
    server_configuration: Option<String>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            server_configuration: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
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
                <BrowserRouter>
                    <Switch<Route> render={Switch::render(switch)} />
                </BrowserRouter>
            </>
        }
    }
}
