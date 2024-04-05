#![allow(unused_imports, dead_code, unused_variables)]
use std::sync::Arc;

use leptos::{html::s, server_fn::redirect, svg::view, *};

use leptos_meta::*;
use leptos_router::*;
use log::{info, Level};
use tracing::error;

use crate::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    //let (session_id, set_session_id) = create_signal(String::from(""));

    //provide_context(session_id);

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
                        <Route path="/decades/:session_id" view=DecadePage ssr=SsrMode::OutOfOrder/>
                        <Route path="/runtime/:session_id" view=RuntimePage/>
                        <Route path="/genres/:session_id" view=GenrePage ssr=SsrMode::OutOfOrder/>
                        <Route path="/movies/:session_id" view=MoviePage ssr=SsrMode::OutOfOrder/>
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

    let pending = start_session.pending();
    let session_value = start_session.value();
    let version = start_session.version();

    if version.get() == 0 {
        start_session.dispatch(StartSession {});
    }

    view! {
        <div
            style:position="absolute"
            style:left="45%"
            style:top="30%"
            style:transform="translate(-20%, -25%)"
        >
            {move || {
                {
                    if pending() {
                        view! { <div class="loader"></div> }.into_view()
                    } else {
                        let session_id = session_value();
                        view! {
                            <A
                                href=match session_value() {
                                    Some(session_result) => {
                                        match session_result {
                                            Ok(session_id) => format!("/providers/{}", session_id),
                                            Err(_) => String::from("/"),
                                        }
                                    }
                                    None => String::from("/"),
                                }

                                class="btn btn-primary"
                            >
                                "Get Started"
                            </A>
                        }
                            .into_view()
                    }
                }
            }}

        </div>
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

#[component]
pub fn TestComponent() -> impl IntoView {
    view! { <p>"Testing"</p> }
}

#[component]
pub fn GenrePage() -> impl IntoView {
    let genres = create_resource(|| (), |_| async move { fetch_genres().await });
    let params = use_params_map();
    let session_id = move || {
        params
            .with(|params| params.get("session_id").cloned())
            .expect("Oh noooo")
    };
    let (selected_data, set_select_data) = create_signal(Vec::<i32>::new());
    // <GridPage resource=genres />
    view! {
        <div
            style:position="absolute"
            style:left="30%"
            style:top="30%"
            style:transform="translate(-20%, -25%)"
        >
            <h1>{session_id}</h1>
            <GridPage
                resource=genres
                selected_data=selected_data
                set_selected_data=set_select_data
            />

        </div>
    }
}

#[component]
pub fn DecadePage() -> impl IntoView {
    let decades = create_resource(|| (), |_| async move { fetch_decades().await });
    let params = use_params_map();
    let session_id = move || {
        params
            .with(|params| params.get("session_id").cloned())
            .expect("Oh noooo")
    };
    let (selected_data, set_select_data) = create_signal(Vec::<i32>::new());
    view! {
        <div
            style:position="absolute"
            style:left="30%"
            style:top="30%"
            style:transform="translate(-20%, -25%)"
        >
            <h1>{session_id}</h1>
            <GridPage
                resource=decades
                selected_data=selected_data
                set_selected_data=set_select_data
            />
            <A href=format!("/genres/{}", session_id()) class="btn btn-primary">
                "To Genres"
            </A>
        </div>
    }
}

#[component]
pub fn ProviderPage() -> impl IntoView {
    let watch_providers = create_resource(
        || (),
        |_| async move { fetch_simple_watch_providers().await },
    );
    let post_providers = create_server_action::<PostProviders>();
    let params = use_params_map();
    let session_id = move || {
        params
            .with(|params| params.get("session_id").cloned())
            .expect("Oh noooo")
    };
    let (selected_data, set_select_data) = create_signal(Vec::<i32>::new());
    view! {
        <div
            style:position="absolute"
            style:left="30%"
            style:top="30%"
            style:transform="translate(-20%, -25%)"
        >
            <h1>{session_id}</h1>
            <GridPage
                resource=watch_providers
                selected_data=selected_data
                set_selected_data=set_select_data
            />
            <A
                href=format!("/runtime/{}", session_id())
                class="btn btn-primary"
                on:click=move |_| {
                    post_providers
                        .dispatch(PostProviders {
                            session_id: session_id(),
                            providers: selected_data.get(),
                        });
                }
            >
                "To Runtime"
            </A>
        </div>
    }
}

#[component]
pub fn MoviePage() -> impl IntoView {
    //let recommendations = create_resource(|| (), |_| async move { get_movies().await });
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
            <h1>{move || println!("{:#?}", loading())}</h1>
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
    //let session_id = use_context::<ReadSignal<String>>().expect("No session located");
    let params = use_params_map();
    let session_id = move || {
        params
            .with(|params| params.get("session_id").cloned())
            .expect("Oh noooo")
    };
    view! {
        <div
            style:position="absolute"
            style:left="40%"
            style:top="30%"
            style:transform="translate(-20%, -25%)"
        >
            <h1>{session_id}</h1>
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
            <A href=format!("/decades/{}", session_id()) class="btn btn-primary">
                "To Decades"
            </A>
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
fn Card<T: CardData + Clone + 'static>(
    #[prop(into)] card_data: Vec<T>,
    selected_data: ReadSignal<Vec<i32>>,
    set_select_data: WriteSignal<Vec<i32>>,
) -> impl IntoView {
    //let (selected_data, set_select_data) = create_signal(Vec::<i32>::new());
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
                                    view! { data.get_body() }.into_view()
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
