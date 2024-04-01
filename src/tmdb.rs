use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(feature = "ssr")] {
        use std::{fs, sync::Arc};

    #[allow(unused_imports)]
    use reqwest::{
        header::{ACCEPT, AUTHORIZATION, USER_AGENT},
        Response,
    };
    use serde::{Deserialize, Serialize};

    use crate::*;
    pub struct AsyncRecommendation {
        pub movie: Movie,
        //pub providers: Vec<WatchProvider>,
        pub async_providers: tokio::task::JoinHandle<GetWatchProvidersResponse>,
    }
    /* Struct for interacting with TMDB API */
    #[derive(Clone)]
    pub struct Tmdb {
        base_url: String,
        api_key: String,
    }

    /* Methods for TMDB API endpoints */
    impl Tmdb {
        /* Constructor for building Tmdb object */
        pub fn new() -> Self {
            let api_key: String = fs::read_to_string("config/api.key")
                .expect("Unable to read API Key!")
                .trim()
                .to_string();
            let base_url: String = String::from("https://api.themoviedb.org/3");
            Self { api_key, base_url }
        }

        pub fn mock(api_key: String, base_url: String) -> Self {
            Self { api_key, base_url }
        }

        pub fn mock_shared_instance(api_key: String, base_url: String) -> Arc<Self> {
            Arc::new(Self::mock(api_key, base_url))
        }

        /* For building shared instance */
        pub fn shared_instance() -> Arc<Self> {
            Arc::new(Self::new())
        }

        /* Private function to make TMDB API call */
        async fn make_tmdb_request(&self, url: &String) -> Result<Response, reqwest::Error> {
            let client = reqwest::Client::new();
            client
                .get(format!("{}/{}", self.base_url, url))
                .header(AUTHORIZATION, format!("Bearer {0}", self.api_key))
                .header(ACCEPT, "application/json")
                .header(USER_AGENT, "rust web-api demo")
                .send()
                .await
        }

        /* Searches for movie by title - helpful for retrieving movie IDs */
        pub async fn search_by_title(
            &self,
            movie_title: &String,
        ) -> Result<SearchByTitleResponse, Box<dyn std::error::Error>> {
            let url = format!("{}/search/movie?query={}", self.base_url, movie_title);

            let search_response = self.make_tmdb_request(&url).await?;

            let movie_results = search_response.json::<SearchByTitleResponse>().await?;

            Ok(movie_results)
        }

        pub async fn get_keywords_for_id(
            &self,
            movie_id: &i64,
        ) -> Result<KeywordResponse, Box<dyn std::error::Error>> {
            let url = format!("movie/{}/keywords", movie_id);

            let keyword_response = self.make_tmdb_request(&url).await?;

            let keyword_results = keyword_response.json::<KeywordResponse>().await?;

            Ok(keyword_results)
        }

        /* Gets watch providers by movie ID */
        /* Watch providers are given by country, and by type: */
        /* For this application we are mostly interested in "flatrate" */
        pub async fn get_watch_providers_by_id(
            &self,
            movie_id: &String,
        ) -> Result<GetWatchProvidersResponse, Box<dyn std::error::Error>> {
            let url = format!("movie/{}/watch/providers", movie_id);

            let provider_response = self.make_tmdb_request(&url).await?;

            // TODO: Improve error handling for things not available on streaming services
            let providers = provider_response
                .json::<GetWatchProvidersResponse>()
                .await?;
            Ok(providers)
        }

        pub async fn get_genre_list(
            &self,
        ) -> Result<GetGenresResponse, Box<dyn std::error::Error>> {
            let url = "genre/movie/list?language=en".to_string();

            let genre_response = self.make_tmdb_request(&url).await?;

            let genres = genre_response.json::<GetGenresResponse>().await?;

            Ok(genres)
        }

        pub async fn get_providers_list(
            &self,
        ) -> Result<GetProvidersResponse, Box<dyn std::error::Error>> {
            let url = "watch/providers/movie?language=en-US&watch_region=US".to_string();

            let providers_response = self.make_tmdb_request(&url).await?;

            let providers = providers_response
                .json::<GetProvidersResponse>()
                .await
                .expect("Error parsing JSON");

            Ok(providers)
        }

        pub async fn get_recommendations(
            &self,
            genres: Vec<Genre>,
            watch_providers: Vec<WatchProvider>,
            runtime: Runtime,
            decade: Decade,
            feedback: Option<Feedback>,
        ) -> Result<GetRecommendationsResponse, Box<dyn std::error::Error>> {
            let genre_ids: String = genres
                .iter()
                .map(|g| g.id.to_string())
                .collect::<Vec<_>>()
                .join(",");

            let provider_ids: String = watch_providers
                .iter()
                .map(|p| p.provider_id.to_string())
                .collect::<Vec<_>>()
                .join("|");

            let start_date = decade.year_range().0;

            let end_date = decade.year_range().1;

            let mut url = format!(
                "discover/movie?include_adult=false&include_video=false&language=en-US&page=1&{}&{}&{}&{}&sort_by=popularity.desc&watch_region=US&{}&with_watch_monetization_types=flatrate&{}",
                format!("primary_release_date.gte={}-01-01", start_date),
                format!("primary_release_date.lte={}-12-31", end_date),
                format!("with_runtime.gte={}", runtime.runtime().0),
                format!("with_runtime.lte={}",runtime.runtime().1),
                format!("with_genres={}", genre_ids),
                format!("with_watch_providers={}",provider_ids)
            );

            match feedback {
                Some(feedback) => {
                    match feedback.like {
                        Some(keywords) => url.push_str(&format!(
                            "&with_keywords={}",
                            keywords
                                .iter()
                                .map(|k| k.to_string())
                                .collect::<Vec<_>>()
                                .join("|")
                        )),
                        None => println!("Nothing"),
                    };

                    match feedback.dislike {
                        Some(keywords) => url.push_str(&format!(
                            "&without_keywords={}",
                            keywords
                                .iter()
                                .map(|k| k.to_string())
                                .collect::<Vec<_>>()
                                .join("|")
                        )),
                        None => println!("Nothing"),
                    };
                }
                None => {}
            }

            println!("{}", &url);

            let recommendation_response = self.make_tmdb_request(&url).await?;

            let recommendations = recommendation_response
                .json::<GetRecommendationsResponse>()
                .await
                .expect("Error parsing JSON");

            Ok(recommendations)
        }
    }
    #[allow(dead_code)]
    #[cfg(test)]
    mod tests {
        use super::*;
        use httpmock::prelude::*;
        use lazy_static::lazy_static;

        lazy_static! {
            static ref MOCK_TMDB_VALID: MockServer = MockServer::start();
            static ref MOCK_TMDB_INVALID: MockServer = MockServer::start();
        }

        #[allow(dead_code)]
        fn get_json_from_file(file_name: &str) -> String {
            fs::read_to_string(format!("src/test/{}.json", file_name)).expect("Error parsing file")
        }

        #[tokio::test]
        #[should_panic]
        async fn test_keywords_invalid() {
            let movie_id = 12345;
            let api_key = String::from("supersecret");

            let tmdb = Tmdb {
                base_url: MOCK_TMDB_INVALID.base_url(),
                api_key: api_key.clone(),
            };

            let keyword_mock = MOCK_TMDB_INVALID.mock(|when, then| {
                when.method(GET)
                    .path(format!("/movie/{}/keywords", movie_id))
                    .header("Authorization", format!("Bearer {}", &api_key));
                then.status(500);
            });

            let response = tmdb.get_keywords_for_id(&movie_id).await;

            keyword_mock.assert();

            assert!(response.is_err());

            response.unwrap();
        }

        #[tokio::test]
        async fn test_keywords() {
            let movie_id = 438631;
            let api_key = String::from("supersecret");

            let tmdb = Tmdb {
                base_url: MOCK_TMDB_VALID.base_url(),
                api_key: api_key.clone(),
            };

            let keywords_response = get_json_from_file("keywords_response");

            let keyword_mock = MOCK_TMDB_VALID.mock(|when, then| {
                when.method(GET)
                    .path(format!("/movie/{}/keywords", movie_id))
                    .header("Authorization", format!("Bearer {}", &api_key));
                then.status(200).body(keywords_response);
            });

            let response = tmdb.get_keywords_for_id(&movie_id).await;

            keyword_mock.assert();

            assert!(response.is_ok());

            let response = response.unwrap();

            assert_eq!(response.id, movie_id);

            assert!(!response.keywords.is_empty());
        }

        #[tokio::test]
        #[should_panic]
        async fn test_watch_providers_invalid() {
            let movie_id = String::from("456");
            let api_key = String::from("supersecret");
            let tmdb = Tmdb {
                base_url: MOCK_TMDB_INVALID.base_url(),
                api_key: api_key.clone(),
            };

            let provider_mock = MOCK_TMDB_INVALID.mock(|when, then| {
                when.method(GET)
                    .path(format!("/movie/{}/watch/providers", movie_id))
                    .header("Authorization", format!("Bearer {}", &api_key));
                then.status(500);
            });

            let response = tmdb.get_watch_providers_by_id(&movie_id).await;

            provider_mock.assert();

            assert!(response.is_err());

            response.unwrap();
        }

        #[tokio::test]
        async fn test_watch_providers() {
            let movie_id = String::from("123");
            let api_key = String::from("supersecret");
            let tmdb = Tmdb {
                base_url: MOCK_TMDB_VALID.base_url(),
                api_key: api_key.clone(),
            };

            let watch_provider_response = get_json_from_file("watch_provider_response");

            let provider_mock = MOCK_TMDB_VALID.mock(|when, then| {
                when.method(GET)
                    .path(format!("/movie/{}/watch/providers", movie_id))
                    .header("Authorization", format!("Bearer {}", &api_key));
                then.status(200).body(watch_provider_response);
            });

            let watch_provider = WatchProvider {
                logo_path: "/bxBlRPEPpMVDc4jMhSrTf2339DW.jpg".to_string(),
                provider_id: 15,
                provider_name: "Hulu".to_string(),
            };

            let response = tmdb.get_watch_providers_by_id(&movie_id).await;

            provider_mock.assert();

            assert!(response.is_ok());

            let response = response.unwrap().results.us.flatrate[0].clone();

            assert_eq!(response, watch_provider);
        }

        #[tokio::test]
        #[should_panic]
        async fn test_genres_invalid() {
            let api_key = String::from("supersecret");
            let tmdb = Tmdb {
                base_url: MOCK_TMDB_INVALID.base_url(),
                api_key: api_key.clone(),
            };

            let genre_mock = MOCK_TMDB_INVALID.mock(|when, then| {
                when.method(GET)
                    .path("/genre/movie/list")
                    .query_param("language", "en")
                    .header("Authorization", format!("Bearer {}", &api_key));
                then.status(500);
            });

            genre_mock.assert();

            let response = tmdb.get_genre_list().await;

            assert!(response.is_err());

            response.unwrap();
        }

        #[tokio::test]
        async fn test_genres() {
            let api_key = String::from("supersecret");
            let tmdb = Tmdb {
                base_url: MOCK_TMDB_VALID.base_url(),
                api_key: api_key.clone(),
            };

            let genre_response = get_json_from_file("genres_response");

            let genre_mock = MOCK_TMDB_VALID.mock(|when, then| {
                when.method(GET)
                    .path("/genre/movie/list")
                    .query_param("language", "en")
                    .header("Authorization", format!("Bearer {}", &api_key));
                then.status(200).body(genre_response);
            });

            let test_genre = Genre {
                id: 28,
                name: "Action".to_string(),
            };

            let response = tmdb.get_genre_list().await;

            genre_mock.assert();

            assert!(response.is_ok());

            let response = response.unwrap();

            assert!(!response.genres.is_empty());

            assert!(response.genres.iter().any(|g| g == &test_genre));
        }

        #[tokio::test]
        #[should_panic]
        async fn test_provider_list_invalid() {
            let api_key = String::from("supersecret");
            let tmdb = Tmdb {
                base_url: MOCK_TMDB_INVALID.base_url(),
                api_key: api_key.clone(),
            };

            let provider_mock = MOCK_TMDB_INVALID.mock(|when, then| {
                when.method(GET)
                    .path("/watch/providers/movie")
                    .query_param("language", "en-US")
                    .query_param("watch_region", "US")
                    .header("Authorization", format!("Bearer {}", &api_key));
                then.status(500);
            });

            provider_mock.assert();

            let response = tmdb.get_providers_list().await;

            assert!(response.is_err());

            response.unwrap();
        }

        #[tokio::test]
        async fn test_providers_list() {
            let api_key = String::from("supersecret");
            let tmdb = Tmdb {
                base_url: MOCK_TMDB_VALID.base_url(),
                api_key: api_key.clone(),
            };

            let providers_response = get_json_from_file("watch_providers_list_response");

            let provider_mock = MOCK_TMDB_VALID.mock(|when, then| {
                when.method(GET)
                    .path("/watch/providers/movie")
                    .query_param("language", "en-US")
                    .query_param("watch_region", "US")
                    .header("Authorization", format!("Bearer {}", &api_key));
                then.status(200).body(providers_response);
            });

            let test_provider = WatchProvider {
                logo_path: "/pbpMk2JmcoNnQwx5JGpXngfoWtp.jpg".to_string(),
                provider_name: "Netflix".to_string(),
                provider_id: 8,
            };

            let response = tmdb.get_providers_list().await;

            provider_mock.assert();

            assert!(response.is_ok());

            let response = response.unwrap();

            assert!(!response.results.is_empty());

            assert!(response.results.iter().any(|p| p == &test_provider));
        }

        #[tokio::test]
        async fn test_get_recommendations() {
            let genres = vec![
                Genre {
                    id: 28,
                    name: "Action".to_string(),
                },
                Genre {
                    id: 12,
                    name: "Adventure".to_string(),
                },
            ];
            let watch_providers = vec![
                WatchProvider {
                    logo_path: "/pbpMk2JmcoNnQwx5JGpXngfoWtp.jpg".to_string(),
                    provider_name: "Netflix".to_string(),
                    provider_id: 8,
                },
                WatchProvider {
                    logo_path: "/7YPdUs60C9qQQQfOFCgxpnF07D9.jpg".to_string(),
                    provider_name: "Disney Plus".to_string(),
                    provider_id: 337,
                },
            ];

            let runtime = Runtime::Average;

            let decade = Decade::TwentyTens;

            let feedback = Feedback {
                like: None,
                dislike: None,
            };

            let api_key = String::from("supersecret");
            let tmdb = Tmdb {
                base_url: MOCK_TMDB_VALID.base_url(),
                api_key: api_key.clone(),
            };

            let rec_response = get_json_from_file("recommendations_response");

            let rec_mock = MOCK_TMDB_VALID.mock(|when, then| {
                when.method(GET)
                    .path("/discover/movie")
                    .query_param("include_adult", "false")
                    .query_param("include_video", "false")
                    .query_param("language", "en-US")
                    .query_param("page", "1")
                    .query_param("primary_release_date.gte", "2010-01-01")
                    .query_param("primary_release_date.lte", "2019-12-31")
                    .query_param("with_runtime.gte", "90")
                    .query_param("with_runtime.lte", "120")
                    .query_param("sort_by", "popularity.desc")
                    .query_param("watch_region", "US")
                    .query_param("with_genres", "28,12")
                    .query_param("with_watch_monetization_types", "flatrate")
                    .query_param("with_watch_providers", "8|337")
                    .header("Authorization", format!("Bearer {}", &api_key));
                then.status(200).body(rec_response);
            });

            let movie = Movie{
            id: 293660,
            overview: "The origin story of former Special Forces operative turned mercenary Wade Wilson, who, after being subjected to a rogue experiment that leaves him with accelerated healing powers, adopts the alter ego Deadpool. Armed with his new abilities and a dark, twisted sense of humor, Deadpool hunts down the man who nearly destroyed his life.".to_string(),
            poster_path: Some("/fSRb7vyIP8rQpL0I47P3qUsEKX3.jpg".to_string()),
            release_date: "2016-02-09".to_string(),
            title: "Deadpool".to_string(),
        };

            let response = tmdb
                .get_recommendations(genres, watch_providers, runtime, decade, Some(feedback))
                .await;

            rec_mock.assert();

            assert!(response.is_ok());

            let response = response.unwrap();

            assert!(!response.results.is_empty());

            assert!(response.results.iter().any(|m| m == &movie));
        }
    }
    }
}

/* ======================================================================================================================== */
/* ====================================================== UNIT TESTS ====================================================== */
/* ======================================================================================================================== */
