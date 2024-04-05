#![allow(unused_imports, dead_code, unused_variables)]
use std::{f64::consts::E, sync::Arc};

use leptos::{html::s, server_fn::redirect, svg::view, *};
use leptos_meta::*;
use leptos_router::*;
use log::{info, Level};
use serde::de;
use tracing::error;
use web_sys::js_sys::global;

use crate::*;

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
pub fn ProviderPage() -> impl IntoView {
    let watch_providers = create_resource(
        || (),
        |_| async move { fetch_simple_watch_providers().await },
    );
    let post_providers = create_server_action::<PostProviders>();
    let pending = post_providers.pending();
    let params = use_params_map();
    let session_id = move || {
        params
            .with(|params| params.get("session_id").cloned())
            .expect("Oh noooo")
    };
    let (selected_data, set_select_data) = create_signal(Vec::<i32>::new());

    let mut global_state = expect_context::<GlobalState>();
    view! {
        <div
            style:position="absolute"
            style:left="30%"
            style:top="30%"
            style:transform="translate(-20%, -25%)"
        >
            <GridPage
                resource=watch_providers
                selected_data=selected_data
                set_selected_data=set_select_data
            />
            {move || if !global_state.data_loading.get() {
                view! {
                    <A
                        href=format!("/runtime/{}", session_id())
                        class="btn btn-primary"
                        on:click=move |_| {
                            provide_context(global_state.data_loading = pending);
                            post_providers
                                .dispatch(PostProviders {
                                    session_id: session_id(),
                                    providers: selected_data.get(),
                                });
                        }
                    >

                        "To Runtime"
                    </A>
                }.into_view()
            }else{
                view! {}.into_view()
            }}

        </div>
    }
}

#[component]
pub fn RuntimePage() -> impl IntoView {
    let (runtime, set_runtime) = create_signal(1);
    let post_runtime = create_server_action::<PostRuntime>();
    let pending = post_runtime.pending();
    let params = use_params_map();
    let session_id = move || {
        params
            .with(|params| params.get("session_id").cloned())
            .expect("Oh noooo")
    };
    let mut global_state = expect_context::<GlobalState>();
    view! {
        <div
            style:position="absolute"
            style:left="40%"
            style:top="30%"
            style:transform="translate(-20%, -25%)"
        >
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
            {move || if !global_state.data_loading.get() {
                view! {
                    <A
                        href=format!("/decade/{}", session_id())
                        class="btn btn-primary"
                        on:click=move |_| {
                            provide_context(global_state.data_loading = pending);
                            let selected_runtime = match runtime.get() {
                                1 => Runtime::Quick,
                                2 => Runtime::Average,
                                3 => Runtime::MovieNight,
                                4 => Runtime::MartinScorsese,
                                _ => Runtime::Average,
                            };
                            post_runtime
                                .dispatch(PostRuntime {
                                    session_id: session_id(),
                                    runtime: selected_runtime,
                                });
                        }
                    >

                        "Next"
                    </A>
                }.into_view()
            }else{
                view! {}.into_view()
            }}

        </div>
    }
}

#[component]
pub fn DecadePage() -> impl IntoView {
    let (decade, set_decade) = create_signal(1);
    let post_decade = create_server_action::<PostDecade>();
    let pending = post_decade.pending();
    let params = use_params_map();
    let session_id = move || {
        params
            .with(|params| params.get("session_id").cloned())
            .expect("Oh noooo")
    };
    let mut global_state = expect_context::<GlobalState>();
    view! {
        <div
            style:position="absolute"
            style:left="40%"
            style:top="30%"
            style:transform="translate(-20%, -25%)"
        >
            <div style:display="flex" style:alignItems="center" style:justifyContent="center">
                <div style:width="800px">
                    <input
                        type="range"
                        min=1
                        max=9
                        step=1
                        bind:value=decade
                        style:width="90%"
                        on:input=move |e| {
                            match event_target_value(&e).parse() {
                                Ok(target_value) => set_decade(target_value),
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
                        <span style:width="11%">Classics</span>
                        <span style:width="11%">50s</span>
                        <span style:width="11%">60s</span>
                        <span style:width="11%">70s</span>
                        <span style:width="11%">80s</span>
                        <span style:width="11%">90s</span>
                        <span style:width="11%">2000s</span>
                        <span style:width="11%">2010s</span>
                        <span style:width="11%">Recent</span>
                    </div>
                </div>
            </div>
            {move || if !global_state.data_loading.get(){
                view! {
                    <A
                        href=format!("/genres/{}", session_id())
                        class="btn btn-primary"
                        on:click=move |_| {
                            provide_context(global_state.data_loading = pending);
                            let selected_decade = match decade.get() {
                                1 => Decade::Classic,
                                2 => Decade::Fifties,
                                3 => Decade::Sixties,
                                4 => Decade::Seventies,
                                5 => Decade::Eighties,
                                6 => Decade::Nineties,
                                7 => Decade::TwoThousands,
                                8 => Decade::TwentyTens,
                                _ => Decade::Recent,
                            };
                            post_decade
                                .dispatch(PostDecade {
                                    session_id: session_id(),
                                    decade: selected_decade,
                                });
                        }
                    >

                        "Next"
                    </A>
                }.into_view()
            }else{
                view! {}.into_view()
            }}

        </div>
    }
}

#[component]
pub fn GenrePage() -> impl IntoView {
    let genres = create_resource(|| (), |_| async move { fetch_genres().await });
    let post_genres = create_server_action::<PostGenres>();
    let pending = post_genres.pending();
    let params = use_params_map();
    let session_id = move || {
        params
            .with(|params| params.get("session_id").cloned())
            .expect("Oh noooo")
    };
    let (selected_data, set_select_data) = create_signal(Vec::<i32>::new());
    let mut global_state = expect_context::<GlobalState>();
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
            {move || if !global_state.data_loading.get(){
                view! {
                    <A
                        href=format!("/recommend/{}", session_id())
                        class="btn btn-primary"
                        on:click=move |_| {
                            provide_context(global_state.data_loading = pending);
                            post_genres
                                .dispatch(PostGenres {
                                    session_id: session_id(),
                                    genres: selected_data.get(),
                                });
                        }
                    >

                        "Get Recommendations"
                    </A>
                }.into_view()
            }else{
                view! {}.into_view()
            }}


        </div>
    }
}

#[component]
pub fn RecommendationPage() -> impl IntoView {
    let params = use_params_map();
    let session_id = move || {
        params
            .with(|params| params.get("session_id").cloned())
            .expect("Oh noooo")
    };
    let (selected_data, set_select_data) = create_signal(Vec::<i32>::new());
    let global_state = expect_context::<GlobalState>();
    view! {
        <div
            style:position="absolute"
            style:left="30%"
            style:top="30%"
            style:transform="translate(-20%, -25%)"
        >
            {move || if global_state.data_loading.get() {
                view!{
                <div class="loader" />
                }.into_view()
            }else{
                    let recommendations = create_resource(
                        || (),
                        move |_| async move { fetch_recommendations(session_id()).await },
                    );
                    view! {<GridPage
                        resource=recommendations
                        selected_data=selected_data
                        set_selected_data=set_select_data
                    />
                }.into_view()
            }}
        </div>
    }
}
