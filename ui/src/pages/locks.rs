use crate::app::SwitchProps;
use crate::client::Client;
use crate::proto::*;
use crate::utils::new_hero;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

pub enum Msg {
    Init(Result<Vec<ListResponse_Lock>, String>),
    Update((String, bool)),
    Remove(String),
    Reload,
    Create(String),
}

pub struct Locks {
    client: Client,
    locks: Vec<ListResponse_Lock>,
    error: Option<anyhow::Error>,
}

impl Component for Locks {
    type Message = Msg;
    type Properties = SwitchProps;

    fn create(ctx: &Context<Self>) -> Self {
        let client = ctx.props().client.clone().unwrap();
        let locks = Self {
            client,
            locks: vec![],
            error: None,
        };
        locks.init(ctx);
        locks
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Init(resp) => {
                match resp {
                    Ok(locks) => self.locks = locks,
                    Err(err) => self.error = Some(anyhow::Error::msg(err)),
                };
                true
            }
            Msg::Reload => {
                self.init(ctx);
                true
            }
            Msg::Update((name, state)) => {
                let client = self.client.clone();
                let link = ctx.link().clone();
                spawn_local(async move {
                    let _resp = match state {
                        false => client.unlock(name.as_str()).await,
                        true => client.lock(name.as_str()).await,
                    };
                    link.send_message(Msg::Reload);
                });
                false
            }
            Msg::Remove(name) => {
                let link = ctx.link().clone();
                let client = self.client.clone();
                spawn_local(async move {
                    let _resp = client.remove(name.as_str()).await;
                    link.send_message(Msg::Reload);
                });
                false
            }
            Msg::Create(name) => {
                let link = ctx.link().clone();
                let client = self.client.clone();
                spawn_local(async move {
                    let _resp = client.create(name.as_str()).await;
                    link.send_message(Msg::Reload);
                });
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <>
                { new_hero("Locks", "Overview over all locks", "is-info") }
                <div class="container is-fluid pt-3">
                    <div class="columns">
                        <div class="column is-one-fifth">
                            <input
                                id="create_name_input"
                                class="input is-info"
                                type="text"
                                placeholder="Lock name"
                            />
                        </div>
                        <div class="column">
                            <button
                                class="button is-info"
                                onclick={
                                    link.callback(|_| {
                                        let window = web_sys::window().unwrap();
                                        let document = window.document().unwrap();
                                        let input = document.get_element_by_id("create_name_input").unwrap();
                                        let input: web_sys::HtmlInputElement = input
                                            .dyn_into::<web_sys::HtmlInputElement>()
                                            .map_err(|_| ())
                                            .unwrap();
                                        let name = input.value();
                                        input.set_value("");
                                        Msg::Create(name)
                                    })
                                }
                            >
                                {"Create"}
                            </button>
                        </div>
                    </div>
                </div>
                <table class="table is-fullwidth is-hoverable">
                    <thead>
                        <tr>
                            <th>{"Name"}</th>
                            <th>{"State"}</th>
                            <th>{"Action"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {
                            self.locks.iter().map(|lock| {
                                let t_name = lock.name.clone().to_string();
                                let r_name = t_name.clone();
                                let state = lock.state;
                                let toggle = link.callback(move |_| Msg::Update((t_name.clone(), !state)));
                                let remove = link.callback(move |_| Msg::Remove(r_name.clone()));
                                html!{
                                <tr>
                                    <th>{ lock.name.clone() }</th>
                                    <th>
                                    {
                                        match lock.state {
                                            false => html!{
                                                <span class="tag is-success is-light">{"Unlocked"}</span>
                                            },
                                            true => html!{
                                                <span class="tag is-danger is-light">{"Locked"}</span>
                                            }
                                        }
                                    }
                                    </th>
                                    <th>
                                        <button class="button is-warning mr-3" onclick={toggle}>
                                            {match lock.state { false => "Lock", true => "Unlock"}}
                                        </button>
                                        <button class="button is-danger" onclick={remove}>
                                            {"Remove"}
                                        </button>
                                    </th>
                                </tr>
                                }}).collect::<Html>()
                        }
                    </tbody>
                </table>
                {
                    if self.locks.is_empty() {
                        html!{
                            <div class="container">
                                <div class="notification is-info is-light">
                                    {"Looks empty to me!"}
                                </div>
                            </div>
                        }
                    } else {
                        html!{}
                    }
                }
            </>
        }
    }
}

impl Locks {
    fn init(&self, ctx: &Context<Self>) {
        let client = self.client.clone();
        let link = ctx.link().clone();
        spawn_local(async move {
            let resp = client.clone().list().await.map_err(|err| err.to_string());
            link.send_message(Msg::Init(resp.clone()));
        })
    }
}
