use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
    use crate::*;
    use lazy_static::lazy_static;
    use redis::{Commands, Connection};
    use uuid::Uuid;
    use crate::RecommendationCriteria;

    //const CONNECTION_STRING: &str = "redis://localhost:6379";

    lazy_static! {
        static ref CONNECTION_STRING: String = match std::env::var("REDIS_CONNECTION_STRING") {
            Ok(con_string) => {
                println!("Env connection string found...");
                con_string
            }
            Err(e) => {
                println!("Connection string not set. Defaulting...");
                "redis://localhost:6379".to_string()
            }
        };
    }

    #[cfg(feature = "ssr")]
    pub async fn criteria_from_cache(
        session_id: &String,
    ) -> Result<RecommendationCriteria, redis::RedisError> {
        match get_connection() {
            Ok(mut con) => {
                let redis_result: String = con.get(session_id).expect("Error reading from redis");

                let criteria: RecommendationCriteria =
                    serde_json::from_str(&redis_result).expect("Error parsing result");

                Ok(criteria)
            }
            Err(err) => Err(err),
        }
    }

    #[cfg(feature = "ssr")]
    pub async fn criteria_to_cache(
        session_id: &String,
        criteria: RecommendationCriteria,
    ) -> Result<bool, redis::RedisError> {
        match get_connection() {
            Ok(mut con) => {
                let json_string =
                    serde_json::to_string(&criteria).expect("Unable to parse criteria");
                Ok(con
                    .set(session_id, json_string)
                    .expect("Error writing to redis cache"))
            }
            Err(err) => Err(err),
        }
    }

    #[cfg(feature= "ssr")]
    pub async fn clear_session_feedback(session_id: &String) -> Result<(), redis::RedisError> {
        println!("Clearing feedback data for {}", &session_id);
        match get_connection() {
            Ok(con) => {
                let existing_criteria = criteria_from_cache(&session_id).await;
                match existing_criteria {
                    Ok(mut criteria) => {
                        criteria.feedback = None;
                        let response = criteria_to_cache(&session_id, criteria).await;
                        match response {
                            Ok(_) => Ok(()),
                            Err(err) => Err(err)
                        }
                    },
                    Err(err) => Err(err)
                }
            },
            Err(err) => Err(err)
        }
    }

    #[cfg(feature = "ssr")]
    pub async fn start_recommendation_session() -> Result<String, redis::RedisError> {
        match get_connection() {
            Ok(mut con) => {
                let session_id = Uuid::new_v4().to_string();

                let criteria = RecommendationCriteria {
                    genres: None,
                    watch_providers: None,
                    runtime: None,
                    decade: None,
                    feedback: None,
                };

                let json_string =
                    serde_json::to_string(&criteria).expect("Unable to parse criteria");

                let _: () = con.set(&session_id, json_string).unwrap();

                Ok(session_id)
            }
            Err(err) => Err(err),
        }
    }

    #[cfg(feature = "ssr")]
    fn get_connection() -> Result<Connection, redis::RedisError> {
        match redis::Client::open(CONNECTION_STRING.as_str()) {
            Ok(client) => client.get_connection(),
            Err(err) => Err(err),
        }
    }

    #[cfg(feature = "ssr")]
    pub async fn end_session(session_id: String) {
        let mut con = get_connection().expect("Error connecting to redis");

        let _: () = con.del(session_id).unwrap();
    }
    #[cfg(test)]
    mod local_redis {
        use std::vec;

        use super::*;
        use redis::ConnectionLike;

        #[tokio::test]
        async fn redis_connection() {
            let mut con = get_connection().unwrap();
            assert!(con.check_connection());
        }

        #[tokio::test]
        async fn redis_criteria_roundtrip() {
            let session_id = start_recommendation_session().await;

            assert!(session_id.is_ok());

            let session_id = session_id.unwrap();

            let criteria_start = RecommendationCriteria {
                genres: Some(vec![1]),
                watch_providers: Some(vec![1]),
                runtime: Some(Runtime::MovieNight),
                decade: Some(Decade::Eighties),
                feedback: None,
            };

            let to_cache_result = criteria_to_cache(&session_id, criteria_start.clone()).await;

            assert!(to_cache_result.is_ok());

            let from_cache_result = criteria_from_cache(&session_id).await;

            assert!(from_cache_result.is_ok());

            assert_eq!(from_cache_result.unwrap(), criteria_start);

            end_session(session_id).await;
        }

        #[tokio::test]
        async fn redis_start_session() {
            let empty_criteria_string =
            "{\"genres\":null,\"watch_providers\":null,\"runtime\":null,\"decade\":null,\"feedback\":null}";
            let response = start_recommendation_session().await;

            assert!(response.is_ok());

            let session_id = response.unwrap();

            assert!(!session_id.is_empty());

            let mut con = get_connection().unwrap();
            assert!(con.check_connection());

            let empty_criteria: String = con.get(session_id).expect("Error fetching from redis");

            assert_eq!(empty_criteria, empty_criteria_string);
        }
    }
}
}
