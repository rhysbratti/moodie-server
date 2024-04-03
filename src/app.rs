#![allow(unused_imports, dead_code, unused_variables)]
use std::sync::Arc;

use leptos::{server_fn::redirect, svg::view, *};

use leptos_meta::*;
use leptos_router::*;
use log::{info, Level};
use tracing::error;

use crate::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
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
                        <Route path="/" view=HomePage />
                        <Route path="/providers" view=ProviderPage ssr=SsrMode::OutOfOrder/>
                        <Route path="/decades" view=DecadePage ssr=SsrMode::OutOfOrder/>
                        <Route path="/runtime" view=RuntimePage/>
                        <Route path="/movies" view=MoviePage ssr=SsrMode::OutOfOrder/>
                        <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
                    </Routes>
                </main>
            </Router>

        </body>
    }
}

#[component]
pub fn HomePage() -> impl IntoView {
    let start_session = create_server_action::<StartSession>();

    view! {
        <div
            style:position="absolute"
            style:left="45%"
            style:top="30%"
            style:transform="translate(-20%, -25%)"
        >
        <button class="btn btn-primary" on:click={
            move |_| {
                start_session.dispatch(StartSession{});
            }
        }>"Get Started"</button>

        </div>
    }
}

#[component]
pub fn GridPage<T: CardData + Clone + 'static>(
    resource: Resource<(), Result<Vec<T>, ServerFnError>>,
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
                                <Grid>
                                    <LoadingCards/>
                                </Grid>
                            }
                        }
                            .into_view()
                    }
                    Some(data) => {
                        {
                            view! {
                                <Grid>
                                    <Card card_data=data.expect("whoopsie")/>
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

#[component]
pub fn DecadePage() -> impl IntoView {
    let decades = create_resource(|| (), |_| async move { fetch_decades().await });
    //<GridPage resource=decades/>
    view! {
        <div
            style:position="absolute"
            style:left="30%"
            style:top="30%"
            style:transform="translate(-20%, -25%)"
        >
            <GridPage resource=decades />

        </div>
    }
}

#[component]
pub fn ProviderPage() -> impl IntoView {
    let watch_providers = create_resource(
        || (),
        |_| async move { fetch_simple_watch_providers().await },
    );
    let get_session = create_server_action::<GetSession>();
    get_session.dispatch(GetSession {});
    let session_val = get_session.value();
    let pending = get_session.pending();
    view! {
        <div
            style:position="absolute"
            style:left="30%"
            style:top="30%"
            style:transform="translate(-20%, -25%)"
        >
        <h1>{move || format!("{:#?}", pending())}</h1>
            {move || {if pending().into() {
                view! { <div class="loader"></div> }.into_view()
            } else {
                view! {
                    {match session_val() {
                        Some(session_id) => view! { <h1>{session_id}</h1> }.into_view(),
                        None => view! {}.into_view(),
                    }}
                }
                    .into_view()
            }}}
            <code>"Here yee: "{move || format!("{:#?}", session_val())}</code>
            <GridPage resource=watch_providers />

        </div>
    }
}

#[component]
pub fn MoviePage() -> impl IntoView {
    let recommendations = create_resource(|| (), |_| async move { get_movies().await });
    let session_resource =
        use_context::<Resource<(), Result<String, ServerFnError>>>().expect("session resource");
    let loading = session_resource.loading();
    view! {
        <div
            style:position="absolute"
            style:left="30%"
            style:top="30%"
            style:transform="translate(-20%, -25%)"
        >
            {if loading() {
                view! { <div class="loader"></div> }.into_view()
            } else {
                view! {
                    {match session_resource.get() {
                        Some(session_id) => view! { <h1>{session_id}</h1> }.into_view(),
                        None => view! {}.into_view(),
                    }}
                }
                    .into_view()
            }}

        </div>
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
fn RuntimePage() -> impl IntoView {
    let (runtime, set_runtime) = create_signal(1);
    let session_resource =
        use_context::<Resource<(), Result<String, ServerFnError>>>().expect("session resource");
    let loading = session_resource.loading();
    view! {
        <div
            style:position="absolute"
            style:left="40%"
            style:top="30%"
            style:transform="translate(-20%, -25%)"
        >
            {if loading() {
                view! { <div class="loader"></div> }.into_view()
            } else {
                view! {
                    {match session_resource.get() {
                        Some(session_id) => view! { <h1>{session_id}</h1> }.into_view(),
                        None => view! {}.into_view(),
                    }}
                }
                    .into_view()
            }}

            <div style:display="flex" style:alignItems="center" style:justifyContent="center">
                <div style:width="600px">
                    <input
                        type="range"
                        min=1
                        max=4
                        step=1
                        bind:value=runtime
                        style:width="90%"
                        on:input=move |e| {
                            match event_target_value(&e).parse() {
                                Ok(target_value) => set_runtime(target_value),
                                Err(err) => error!("{}", err),
                            }
                        }
                    />

                    <div
                        style:display="flex"
                        style:justifyContent="space-between"
                        style:position="absolute"
                        style:bottom="calc(100% + 10px)"
                        style:left="0"
                        style:right="0"
                        style:marginTop="10px"
                    >
                        <span style:width="25%">Quick</span>
                        <span style:width="25%">Average</span>
                        <span style:width="25%">Movie Night</span>
                        <span style:width="25%">Martin Scorsese</span>
                    </div>
                </div>
            </div>
            <h1>{runtime}</h1>
        </div>
    }
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
fn Card<T: CardData + Clone + 'static>(#[prop(into)] card_data: Vec<T>) -> impl IntoView {
    let (selected_data, set_select_data) = create_signal(Vec::<i32>::new());
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
                            <Show
                                when={
                                    let has_body = data.has_body();
                                    move || { has_body }
                                }

                                fallback=|| view! {}
                            >
                                <div class="card-body">{data.get_body()}</div>
                            </Show>

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
