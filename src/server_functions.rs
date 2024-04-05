use leptos::{server_fn::redirect, svg::view, *};
use serde::de;
use std::sync::Arc;

use crate::*;
use leptos_meta::*;
use leptos_router::*;

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

#[server(PostDecade, "/api")]
pub async fn post_decades(session_id: String, decade: Decade) -> Result<(), ServerFnError> {
    let id = session_id.clone();

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
pub async fn post_providers(session_id: String, providers: Vec<i32>) -> Result<(), ServerFnError> {
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
pub async fn post_genres(session_id: String, genres: Vec<i32>) -> Result<(), ServerFnError> {
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
pub async fn post_runtime(session_id: String, runtime: Runtime) -> Result<(), ServerFnError> {
    let id = session_id.clone();
    println!("Received a runtime: {:#?}", runtime);

    match redis_helper::criteria_from_cache(&session_id).await {
        Err(err) => Err(ServerFnError::new(format!(
            "Error reading criteria from cache: {}",
            err
        ))),
        Ok(mut criteria) => {
            criteria.runtime = Some(runtime);

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
pub async fn start_session() -> Result<String, ServerFnError> {
    use actix_web::{cookie::Cookie, http::header, http::header::HeaderValue};
    use leptos_actix::redirect;
    use leptos_actix::ResponseOptions;
    println!("Got request to start session");
    // pull ResponseOptions from context
    let response = expect_context::<leptos_actix::ResponseOptions>();

    let existing_cookie = get_session().await;

    match existing_cookie {
        Ok(existing_session_id) => Ok(existing_session_id),
        Err(_) => match redis_helper::start_recommendation_session().await {
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
                Ok(session_id)
            }
        },
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
            Ok(cookie.to_string().replace("SESSION_ID=", ""))
        }
        None => {
            println!("No cookie found :/");
            Err(ServerFnError::ServerError(
                "No cookie named SESSION_ID exists".to_string(),
            ))
        }
    }
}

#[server(TestOutput, "/api")]
pub async fn test_output() -> Result<(), ServerFnError> {
    println!("Test resource has been requested");
    Ok(())
}
