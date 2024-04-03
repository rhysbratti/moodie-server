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

#[cfg(feature = "ssr")]
use lazy_static::lazy_static;

#[cfg(feature = "ssr")]
use crate::{tmdb::Tmdb, *};

#[cfg(feature = "ssr")]
lazy_static! {
    static ref TMDB: Arc<Tmdb> = Tmdb::shared_instance();
}

/* Server functions */

#[server(FetchRuntimes, "/api", "GetJson")]
pub async fn fetch_runtimes() -> Result<Vec<RuntimeInfo>, ServerFnError> {
    let runtimes = vec![
        Runtime::Quick.info(),
        Runtime::Average.info(),
        Runtime::MovieNight.info(),
        Runtime::MartinScorsese.info(),
    ];

    Ok(runtimes)
}

#[server(FetchDecades, "/api", "GetJson")]
pub async fn fetch_decades() -> Result<Vec<Decade>, ServerFnError> {
    let decades = vec![
        Decade::Classic,
        Decade::Fifties,
        Decade::Sixties,
        Decade::Seventies,
        Decade::Eighties,
        Decade::Nineties,
        Decade::TwoThousands,
        Decade::TwentyTens,
        Decade::Recent,
    ];

    Ok(decades)
}

#[server(FetchWatchProviders, "/api", "GetJson")]
pub async fn fetch_simple_watch_providers() -> Result<Vec<WatchProvider>, ServerFnError> {
    let tmdb: Arc<Tmdb> = Arc::clone(&TMDB);
    let providers = tmdb.get_providers_list();
    let supported_providers = vec![
        "Netflix",
        "Hulu",
        "Apple TV",
        "Peacock",
        "Amazon Prime Video",
        "Max",
        "Disney Plus",
        "Tubi",
        "Crunchyroll",
        "Paramount Plus",
    ];
    println!("Getting watch providers");
    match providers.await {
        Err(err) => Err(ServerFnError::new(format!(
            "Error fetching watch providers: {}",
            err
        ))),
        Ok(providers) => {
            let mut provider_output: Vec<WatchProvider> = providers
                .results
                .into_iter()
                .filter(|p| supported_providers.contains(&p.provider_name.as_str()))
                .collect();

            for provider in &mut provider_output {
                provider.logo_path = str::replace(provider.logo_path.as_str(), "jpg", "svg");
            }

            Ok(provider_output)
        }
    }
}

#[server(PostDecades, "/api")]
pub async fn post_decades(session_id: String, decade: DecadeResponse) -> Result<(), ServerFnError> {
    let id = session_id.clone();

    let decade = Decade::from_string(&decade.decade);

    match redis_helper::criteria_from_cache(&session_id).await {
        Err(err) => Err(ServerFnError::new(format!(
            "Error reading criteria from cache: {}",
            err
        ))),
        Ok(mut criteria) => {
            criteria.decade = Some(decade);

            match redis_helper::criteria_to_cache(&session_id, criteria).await {
                Ok(redis_response) => {
                    let response = format!("Posted decade for {}", id);

                    println!("{}", &response);

                    Ok(())
                }
                Err(err) => Err(ServerFnError::new(format!(
                    "Error writing decades to cache: {}",
                    err
                ))),
            }
        }
    }
}

#[server(PostProviders, "/api")]
pub async fn post_providers(
    session_id: String,
    providers: Vec<WatchProvider>,
) -> Result<(), ServerFnError> {
    let id = session_id.clone();

    match redis_helper::criteria_from_cache(&session_id).await {
        Err(err) => Err(ServerFnError::new(format!(
            "Error reading criteria from cache: {}",
            err
        ))),
        Ok(mut criteria) => {
            criteria.watch_providers = Some(providers);

            match redis_helper::criteria_to_cache(&session_id, criteria).await {
                Ok(redis_response) => {
                    let response = format!("Posted providers for {}", id);

                    println!("{}", &response);

                    Ok(())
                }
                Err(err) => Err(ServerFnError::new(format!(
                    "Error writing providers to cache: {}",
                    err
                ))),
            }
        }
    }
}

#[server(PostGenres, "/api")]
pub async fn post_genres(session_id: String, genres: Vec<Genre>) -> Result<(), ServerFnError> {
    let id = session_id.clone();

    match redis_helper::criteria_from_cache(&session_id).await {
        Err(err) => Err(ServerFnError::new(format!(
            "Error reading criteria from cache: {}",
            err
        ))),
        Ok(mut criteria) => {
            criteria.genres = Some(genres);

            match redis_helper::criteria_to_cache(&session_id, criteria).await {
                Ok(redis_response) => {
                    let response = format!("Posted genres for{}", id);

                    println!("{}", &response);

                    Ok(())
                }
                Err(err) => Err(ServerFnError::new(format!(
                    "Error writing genres to cache: {}",
                    err
                ))),
            }
        }
    }
}

#[server(PostRuntime, "/api")]
pub async fn post_runtime(
    session_id: String,
    runtime: RuntimeResponse,
) -> Result<(), ServerFnError> {
    let id = session_id.clone();
    println!("Received a runtime: {:#?}", runtime);

    match redis_helper::criteria_from_cache(&session_id).await {
        Err(err) => Err(ServerFnError::new(format!(
            "Error reading criteria from cache: {}",
            err
        ))),
        Ok(mut criteria) => {
            criteria.runtime = Some(runtime.runtime);

            match redis_helper::criteria_to_cache(&session_id, criteria).await {
                Ok(redis_response) => {
                    let response = format!("Posted runtime for {}", &id);

                    Ok(())
                }
                Err(err) => Err(ServerFnError::new(format!(
                    "Error writing runtime to cache: {}",
                    err
                ))),
            }
        }
    }
}

#[cfg(feature = "ssr")]
fn update_feedback(
    mut criteria: RecommendationCriteria,
    mut feedback: Feedback,
) -> RecommendationCriteria {
    if let Some(mut criteria_feedback) = criteria.feedback.take() {
        if let Some(mut likes) = criteria_feedback.like.take() {
            if let Some(feedback_likes) = feedback.like.take() {
                likes.extend(feedback_likes);
                criteria_feedback.like = Some(likes);
            } else {
                criteria_feedback.like = Some(likes);
            }
        } else {
            criteria_feedback.like = feedback.like;
        }
        if let Some(mut dislikes) = criteria_feedback.dislike.take() {
            if let Some(feedback_dislikes) = feedback.dislike.take() {
                dislikes.extend(feedback_dislikes);
                criteria_feedback.dislike = Some(dislikes);
            } else {
                criteria_feedback.dislike = Some(dislikes);
            }
        } else {
            criteria_feedback.dislike = feedback.dislike;
        }
        criteria.feedback = Some(criteria_feedback);
    } else {
        criteria.feedback = Some(feedback);
    }

    criteria
}

#[server(PostFeedback, "/api")]
pub async fn post_feedback(session_id: String, feedback: Feedback) -> Result<(), ServerFnError> {
    let tmdb = Arc::clone(&TMDB);

    match redis_helper::criteria_from_cache(&session_id).await {
        Err(err) => Err(ServerFnError::new(format!(
            "Error reading criteria from cache: {}",
            err
        ))),
        Ok(criteria) => {
            let (upvotes, downvotes) = tmdb_helper::process_feedback(
                tmdb,
                feedback.like.unwrap(),
                feedback.dislike.unwrap(),
            )
            .await;

            let feedback = Feedback {
                like: match upvotes.is_empty() {
                    true => None,
                    false => Some(upvotes),
                },
                dislike: match downvotes.is_empty() {
                    true => None,
                    false => Some(downvotes),
                },
            };

            let criteria = update_feedback(criteria, feedback);

            println!("Posting feedback");

            match redis_helper::criteria_to_cache(&session_id, criteria).await {
                Err(err) => Err(ServerFnError::new(format!(
                    "Error writing feedback to cache: {}",
                    err
                ))),
                Ok(redis_response) => Ok(()),
            }
        }
    }
}

#[server(FetchSessionCriteria, "/api", "GetJson")]
pub async fn fetch_session_criteria(
    session_id: String,
) -> Result<RecommendationCriteria, ServerFnError> {
    let criteria = redis_helper::criteria_from_cache(&session_id)
        .await
        .expect("Uh oh");

    Ok(criteria)
}

#[server(FetchRecommendations, "/api", "GetJson")]
pub async fn fetch_recommendations(
    session_id: String,
) -> Result<Vec<MovieRecommendation>, ServerFnError> {
    let tmdb = Arc::clone(&TMDB);

    match tmdb_helper::get_recommendations_for_session(tmdb, session_id).await {
        Err(err) => Err(ServerFnError::new(format!(
            "Error fetching recommendations: {}",
            err
        ))),
        Ok(recs) => {
            let mut movie_recommendations: Vec<MovieRecommendation> = vec![];

            for rec in recs {
                let providers: Vec<WatchProvider> = rec
                    .async_providers
                    .await
                    .expect(format!("Error fetching watch providers for {}", rec.movie.id).as_str())
                    .results
                    .us
                    .flatrate;
                movie_recommendations.push(MovieRecommendation {
                    movie: rec.movie,
                    providers,
                })
            }

            Ok(movie_recommendations)
        }
    }
}

#[server(FetchGenres, "/api", "GetJson")]
pub async fn fetch_genres() -> Result<Vec<Genre>, ServerFnError> {
    let tmdb = Arc::clone(&TMDB);

    let genre_list = tmdb.get_genre_list().await;

    match tmdb.get_genre_list().await {
        Ok(list) => Ok(list.genres),
        Err(err) => Err(ServerFnError::new(format!(
            "Error fetching genres: {}",
            err
        ))),
    }
}

#[server(StartSession, "/api")]
pub async fn start_session() -> Result<(), ServerFnError> {
    use actix_web::{cookie::Cookie, http::header, http::header::HeaderValue};
    use leptos_actix::redirect;
    use leptos_actix::ResponseOptions;
    println!("Got request to start session");
    // pull ResponseOptions from context
    let response = expect_context::<leptos_actix::ResponseOptions>();

    match redis_helper::start_recommendation_session().await {
        Err(err) => Err(ServerFnError::new(format!(
            "Error creating session ID: {}",
            err
        ))),
        Ok(session_id) => {
            println!("Session: {}", &session_id);
            response.append_header(
                header::SET_COOKIE,
                HeaderValue::from_str(&format!(
                    "SESSION_ID={session_id};\
                     Path=/"
                ))
                .expect("to create header value"),
            );
            redirect("/providers");
            Ok(())
        }
    }
}

#[server(GetSession, "/api")]
pub async fn get_session() -> Result<String, ServerFnError> {
    use actix_web::HttpRequest;
    use actix_web::{cookie::Cookie, http::header, http::header::HeaderValue};
    use leptos_actix::ResponseOptions;
    println!("Got a request for session data");
    // pull ResponseOptions from context
    let response = expect_context::<HttpRequest>();

    match response.cookie("SESSION_ID") {
        Some(cookie) => {
            println!("Found a cookie: {}", &cookie);
            Ok(cookie.to_string())
        }
        None => {
            println!("No cookie found :/");
            Err(ServerFnError::ServerError(
                "No cookie named SESSION_ID exists".to_string(),
            ))
        }
    }
}
