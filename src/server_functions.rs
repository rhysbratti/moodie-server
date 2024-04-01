/*
use std::sync::Arc;

use crate::leptos_server::*;

#[cfg(feature = "ssr")]
use lazy_static::lazy_static;

#[cfg(feature = "ssr")]
use crate::{tmdb::Tmdb, *};

#[cfg(feature = "ssr")]
lazy_static! {
    static ref TMDB: Arc<Tmdb> = Tmdb::shared_instance();
}

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
pub async fn fetch_decades() -> Result<Vec<DecadeInfo>, ServerFnError> {
    let decades = vec![
        Decade::Classic.info(),
        Decade::Fifties.info(),
        Decade::Sixties.info(),
        Decade::Seventies.info(),
        Decade::Eighties.info(),
        Decade::Nineties.info(),
        Decade::TwoThousands.info(),
        Decade::TwentyTens.info(),
        Decade::Recent.info(),
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

#[server(StartSession, "/api", "GetJson")]
pub async fn start_session() -> Result<String, ServerFnError> {
    println!("Got request to start session");
    match redis_helper::start_recommendation_session().await {
        Err(err) => Err(ServerFnError::new(format!(
            "Error creating session ID: {}",
            err
        ))),
        Ok(session_id) => Ok(session_id),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_criteria() -> RecommendationCriteria {
        RecommendationCriteria {
            genres: Some(vec![Genre {
                id: 234,
                name: "test".to_string(),
            }]),
            watch_providers: Some(vec![WatchProvider {
                provider_id: 434,
                provider_name: "foo".to_string(),
                logo_path: "/".to_string(),
            }]),
            runtime: Some(Runtime::Average),
            decade: Some(Decade::Recent),
            feedback: None,
        }
    }

    #[test]
    fn test_update_feedback_criteria() {
        let mut criteria = get_criteria();

        let criteria_feedback = Feedback {
            like: Some(vec![222, 444, 666]),
            dislike: Some(vec![111, 333, 555]),
        };

        let new_feedback = Feedback {
            like: Some(vec![888, 1010, 1212]),
            dislike: Some(vec![777, 999, 1111]),
        };

        criteria.feedback = Some(criteria_feedback);

        let updated_criteria = update_feedback(criteria, new_feedback);

        assert!(updated_criteria.feedback.is_some());

        let updated_feedback = updated_criteria.feedback.unwrap();

        assert_eq!(
            updated_feedback.like.unwrap(),
            vec![222, 444, 666, 888, 1010, 1212]
        );

        assert_eq!(
            updated_feedback.dislike.unwrap(),
            vec![111, 333, 555, 777, 999, 1111]
        );
    }

    #[test]
    fn test_update_feedback_criteria_empty_likes() {
        let mut criteria = get_criteria();

        let criteria_feedback = Feedback {
            like: None,
            dislike: Some(vec![111, 333, 555]),
        };

        let new_feedback = Feedback {
            like: Some(vec![888, 1010, 1212]),
            dislike: Some(vec![777, 999, 1111]),
        };

        criteria.feedback = Some(criteria_feedback);

        let updated_criteria = update_feedback(criteria, new_feedback);

        assert!(updated_criteria.feedback.is_some());

        let updated_feedback = updated_criteria.feedback.unwrap();

        assert_eq!(updated_feedback.like.unwrap(), vec![888, 1010, 1212]);

        assert_eq!(
            updated_feedback.dislike.unwrap(),
            vec![111, 333, 555, 777, 999, 1111]
        );
    }

    #[test]
    fn test_update_feedback_criteria_empty_dislikes() {
        let mut criteria = get_criteria();

        let criteria_feedback = Feedback {
            like: Some(vec![222, 444, 666]),
            dislike: None,
        };

        let new_feedback = Feedback {
            like: Some(vec![888, 1010, 1212]),
            dislike: Some(vec![777, 999, 1111]),
        };

        criteria.feedback = Some(criteria_feedback);

        let updated_criteria = update_feedback(criteria, new_feedback);

        assert!(updated_criteria.feedback.is_some());

        let updated_feedback = updated_criteria.feedback.unwrap();

        assert_eq!(
            updated_feedback.like.unwrap(),
            vec![222, 444, 666, 888, 1010, 1212]
        );

        assert_eq!(updated_feedback.dislike.unwrap(), vec![777, 999, 1111]);
    }

    #[test]
    fn test_update_feedback_empty_likes() {
        let mut criteria = get_criteria();

        let criteria_feedback = Feedback {
            like: Some(vec![222, 444, 666]),
            dislike: Some(vec![111, 333, 555]),
        };

        let new_feedback = Feedback {
            like: None,
            dislike: Some(vec![777, 999, 1111]),
        };

        criteria.feedback = Some(criteria_feedback);

        let updated_criteria = update_feedback(criteria, new_feedback);

        assert!(updated_criteria.feedback.is_some());

        let updated_feedback = updated_criteria.feedback.unwrap();

        assert_eq!(updated_feedback.like.unwrap(), vec![222, 444, 666]);

        assert_eq!(
            updated_feedback.dislike.unwrap(),
            vec![111, 333, 555, 777, 999, 1111]
        );
    }

    #[test]
    fn test_update_feedback_empty_dislikes() {
        let mut criteria = get_criteria();

        let criteria_feedback = Feedback {
            like: Some(vec![222, 444, 666]),
            dislike: Some(vec![111, 333, 555]),
        };

        let new_feedback = Feedback {
            like: Some(vec![888, 1010, 1212]),
            dislike: None,
        };

        criteria.feedback = Some(criteria_feedback);

        let updated_criteria = update_feedback(criteria, new_feedback);

        assert!(updated_criteria.feedback.is_some());

        let updated_feedback = updated_criteria.feedback.unwrap();

        assert_eq!(
            updated_feedback.like.unwrap(),
            vec![222, 444, 666, 888, 1010, 1212]
        );

        assert_eq!(updated_feedback.dislike.unwrap(), vec![111, 333, 555]);
    }

    #[test]
    fn test_update_feedback_empty_criteria() {
        let mut criteria = get_criteria();

        let criteria_feedback = Feedback {
            like: None,
            dislike: None,
        };

        let new_feedback = Feedback {
            like: Some(vec![888, 1010, 1212]),
            dislike: Some(vec![777, 999, 1111]),
        };

        criteria.feedback = Some(criteria_feedback);

        let updated_criteria = update_feedback(criteria, new_feedback);

        assert!(updated_criteria.feedback.is_some());

        let updated_feedback = updated_criteria.feedback.unwrap();

        assert_eq!(updated_feedback.like.unwrap(), vec![888, 1010, 1212]);

        assert_eq!(updated_feedback.dislike.unwrap(), vec![777, 999, 1111]);
    }
}
*/
