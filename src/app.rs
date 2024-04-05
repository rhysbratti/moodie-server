#![allow(unused_imports, dead_code, unused_variables)]
use std::sync::Arc;

use leptos::{html::s, server_fn::redirect, svg::view, *};
use leptos_meta::*;
use leptos_router::*;
use log::{info, Level};
use serde::de;
use tracing::error;

use crate::components::pages::*;
use crate::*;

#[derive(Copy, Clone, Debug)]
pub struct GlobalState {
    pub data_loading: ReadSignal<bool>,
}

impl GlobalState {
    pub fn new() -> Self {
        let (data_loading, _) = create_signal(false);
        Self { data_loading }
    }

    pub fn from(data_loading: ReadSignal<bool>) -> Self {
        Self { data_loading }
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    provide_context(GlobalState::new());

    view! {
        <Stylesheet id="leptos" href="/pkg/moodie_server.css"/>
        <Stylesheet
            id="boostrap"
            href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css"
        />

        <Title text="Moodie"/>

        <body data-bs-theme="dark">

            <Router>
                <nav></nav>
                <main>
                    <Routes>
                        <Route path="/" view=HomePage/>
                        <Route
                            path="/providers/:session_id"
                            view=ProviderPage
                            ssr=SsrMode::OutOfOrder
                        />
                        <Route path="/decade/:session_id" view=DecadePage ssr=SsrMode::OutOfOrder/>
                        <Route path="/runtime/:session_id" view=RuntimePage/>
                        <Route path="/genres/:session_id" view=GenrePage ssr=SsrMode::OutOfOrder/>
                        <Route
                            path="/recommend/:session_id"
                            view=RecommendationPage
                            ssr=SsrMode::OutOfOrder
                        />
                        <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
                    </Routes>
                </main>
            </Router>

        </body>
    }
}

#[component]
pub fn GridPage<T: CardData + Clone + 'static>(
    resource: Resource<(), Result<Vec<T>, ServerFnError>>,
    selected_data: ReadSignal<Vec<i32>>,
    set_selected_data: WriteSignal<Vec<i32>>,
) -> impl IntoView {
    let loading = resource.loading();
    view! {
        {move || {
            if loading() {
                view! {
                    <Grid>
                        <LoadingCards/>
                    </Grid>
                }
                    .into_view()
            } else {
                match resource.get() {
                    None => {
                        {
                            view! {
                                <h1>"There was an error loading the page"</h1>
                                <A href="/">"Home"</A>
                            }
                        }
                            .into_view()
                    }
                    Some(data) => {
                        {
                            view! {
                                <Grid>
                                    <Card
                                        card_data=data.expect("whoopsie")
                                        selected_data=selected_data
                                        set_select_data=set_selected_data
                                    />
                                </Grid>
                            }
                        }
                            .into_view()
                    }
                }
            }
        }}
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { <h1>"Not Found"</h1> }
}

#[component]
fn LoadingCards() -> impl IntoView {
    let cards: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    view! {
        {cards
            .into_iter()
            .map(|_| {
                view! {
                    <div class="col">
                        <div class="card h-100 text-bg-dark" style:width="13rem">
                            <img
                                src="https://raw.githubusercontent.com/rhysbratti/moodie_assets/master/question_mark.png"
                                class="card-img-top"
                                alt="Placeholder"
                                style:width="100%"
                                style:height="auto"
                                loading="lazy"
                            />
                            <div class="card-body">
                                <h5 class="card-title">
                                    <span class="placeholder col-8"></span>
                                </h5>
                            </div>
                            <ul class="list-group list-group-flush">
                                <li class="list-group-item list-group-item-dark">
                                    Release Date: <span className="placeholder col-6"></span>
                                </li>
                            </ul>
                        </div>
                    </div>
                }
            })
            .collect_view()}
    }
}

#[component]
fn Card<T: CardData + Clone + 'static>(
    #[prop(into)] card_data: Vec<T>,
    selected_data: ReadSignal<Vec<i32>>,
    set_select_data: WriteSignal<Vec<i32>>,
) -> impl IntoView {
    view! {
        {card_data
            .into_iter()
            .map(|data| {
                view! {
                    <div
                        key=data.get_id()
                        class="col"
                        on:click={
                            let selected_id = data.get_id();
                            move |_| {
                                set_select_data
                                    .update(|selected_data| {
                                        selected_data.add_or_remove(selected_id)
                                    });
                            }
                        }
                    >

                        <div
                            class="card h-100"
                            class=(
                                "text-bg-secondary",
                                {
                                    let current_id = data.get_id();
                                    move || selected_data().contains(&current_id)
                                },
                            )

                            style:width="13rem"
                        >
                            <img
                                id=data.get_id()
                                src=data.get_logo_path()
                                class="card-img-top"
                                style:width="100%"
                                style:margin="auto"
                                style:display="block"
                                style:height="auto"
                                loading="lazy"
                            />
                            <div class="card-header" style:height="auto">
                                <h5 class="card-title">{data.get_display()}</h5>
                            </div>
                            {move || {
                                if data.has_body() {
                                    view! { <div class="card-body">{data.get_body()}</div> }
                                        .into_view()
                                } else {
                                    view! {}.into_view()
                                }
                            }}

                        </div>
                    </div>
                }
            })
            .collect_view()}
    }
}

#[component]
pub fn Grid(children: Children) -> impl IntoView {
    view! {
        <div>
            <div class="row row-cols-1 row-cols-md-5 g-5">{children()}</div>
        </div>
    }
}
