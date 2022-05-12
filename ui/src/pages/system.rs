use crate::app::SwitchProps;
use crate::client::Client;
use crate::proto::*;
use crate::utils::new_hero;
use tracing::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

pub enum Msg {
    Peers(Result<Vec<PeersResponse_Peer>, String>),
}

pub struct System {
    client: Client,
    peers: Vec<PeersResponse_Peer>,
    error: Option<anyhow::Error>,
}

impl Component for System {
    type Message = Msg;
    type Properties = SwitchProps;

    fn create(ctx: &Context<Self>) -> Self {
        let client = ctx.props().client.clone().unwrap();
        let system = Self {
            client,
            peers: vec![],
            error: None,
        };
        system.peers(ctx);
        system
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Peers(resp) => {
                match resp {
                    Ok(peers) => self.peers = peers,
                    Err(err) => self.error = Some(anyhow::Error::msg(err)),
                };
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let _link = ctx.link();
        let config = ctx.props().client.clone().unwrap().inner();
        html! {
            <>
                { new_hero("System", "All about your cluster", "is-primary") }
                <div class="columns">
                    <div class="column">
                        <section class="section">
                            <h2 class="title" >{"Configuration"}</h2>
                        </section>
                        <pre><code class="language-toml">
                        {
                            match toml::to_string(&config) {
                                Ok(v) => v,
                                Err(err) => err.to_string()
                            }
                        }
                        </code></pre>
                    </div>
                    <div class="column">
                        <section class="section">
                            <h2 class="title" >{"Peers"}</h2>
                        </section>
                        <table class="table is-fullwidth is-hoverable">
                            <thead>
                                <tr>
                                    <th>{"Address"}</th>
                                    <th>{"Status"}</th>
                                </tr>
                            </thead>
                            <tbody>
                            {
                                self.peers
                                    .iter()
                                    .map(
                                        |peer| {
                                            let peer = peer.clone();
                                            info!("Status: {}", peer.status);
                                            let status = match peer.status {
                                                0 => ("Error", "is-danger"),
                                                1 => ("Running", "is-success"),
                                                _ => ("Unknown", "is-light"),

                                            };
                                            html!{
                                            <tr>
                                                <th>
                                                    <a href={peer.api_address.to_string()}>
                                                        {peer.cluster_address.to_string()}
                                                    </a>
                                                </th>
                                                <th>
                                                    <span class={format!("tag {}", status.1)}>
                                                        {status.0}
                                                    </span>
                                                </th>
                                            </tr>
                                            }
                                        }
                                    )
                                    .collect::<Html>()
                            }
                            </tbody>
                        </table>
                    </div>
                </div>
            </>
        }
    }
}

impl System {
    fn peers(&self, ctx: &Context<Self>) {
        let client = self.client.clone();
        let link = ctx.link().clone();
        spawn_local(async move {
            let resp = client.clone().peers().await.map_err(|err| err.to_string());
            link.callback(move |_: ()| Msg::Peers(resp.clone()))
                .emit(());
        })
    }
}
