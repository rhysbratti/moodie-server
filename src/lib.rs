#![allow(unused_imports, dead_code, unused_variables)]
pub mod app;
mod redis_helper;
mod tmdb;
mod tmdb_helper;
use leptos::*;
use serde::{Deserialize, Serialize};
use std::{thread, time::Duration};

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    use leptos::*;

    console_error_panic_hook::set_once();

    mount_to_body(App);
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct RecommendationCriteria {
    pub genres: Option<Vec<Genre>>,
    pub watch_providers: Option<Vec<WatchProvider>>,
    pub runtime: Option<Runtime>,
    pub decade: Option<Decade>,
    pub feedback: Option<Feedback>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Feedback {
    pub like: Option<Vec<i64>>,
    pub dislike: Option<Vec<i64>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Keyword {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct KeywordResponse {
    pub id: i64,
    pub keywords: Vec<Keyword>,
}

/*
   Runtime options
*/
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub enum Runtime {
    Quick,
    Average,
    MovieNight,
    MartinScorsese,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RuntimeResponse {
    pub runtime: Runtime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RuntimeInfo {
    name: String,
    description: String,
}
impl Runtime {
    pub fn info(&self) -> RuntimeInfo {
        match self {
            Runtime::Quick => RuntimeInfo {
                name: String::from("Quick"),
                description: String::from(
                    "You're not looking for a commitment, but still want something awesome",
                ),
            },
            Runtime::Average => RuntimeInfo {
                name: String::from("Average"),
                description: String::from("You've got some time, lets make it count"),
            },
            Runtime::MovieNight => RuntimeInfo {
                name: String::from("Movie Night"),
                description: String::from(
                    "Grab your popcorn, lets find a movie with that 'wow' factor",
                ),
            },
            Runtime::MartinScorsese => RuntimeInfo {
                name: String::from("Martin Scorsese"),
                description: String::from(
                    "You refer to movies as 'films' and have a lot of time on your hands",
                ),
            },
        }
    }

    pub fn runtime(&self) -> (i32, i32) {
        match self {
            Runtime::Quick => (60, 90),
            Runtime::Average => (90, 120),
            Runtime::MovieNight => (120, 150),
            Runtime::MartinScorsese => (150, 500),
        }
    }

    pub fn from_string(runtime_string: &str) -> Self {
        match runtime_string {
            "Quick" => Runtime::Quick,
            "Average" => Runtime::Average,
            "MovieNight" => Runtime::MovieNight,
            "MartinScorsese" => Runtime::MartinScorsese,
            _ => Runtime::Average,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecadeInfo {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecadeResponse {
    pub decade: String,
}

impl Decade {
    pub fn info(&self) -> DecadeInfo {
        match self {
            Decade::Classic => DecadeInfo {
                name: String::from("Classics"),
            },
            Decade::Fifties => DecadeInfo {
                name: String::from("50s"),
            },
            Decade::Sixties => DecadeInfo {
                name: String::from("60s"),
            },
            Decade::Seventies => DecadeInfo {
                name: String::from("70s"),
            },
            Decade::Eighties => DecadeInfo {
                name: String::from("80s"),
            },
            Decade::Nineties => DecadeInfo {
                name: String::from("90s"),
            },
            Decade::TwoThousands => DecadeInfo {
                name: String::from("2000s"),
            },
            Decade::TwentyTens => DecadeInfo {
                name: String::from("2010s"),
            },
            Decade::Recent => DecadeInfo {
                name: String::from("Recent"),
            },
        }
    }

    pub fn from_string(decade_string: &str) -> Self {
        match decade_string {
            "Classics" => Decade::Classic,
            "50s" => Decade::Fifties,
            "60s" => Decade::Sixties,
            "70s" => Decade::Seventies,
            "80s" => Decade::Eighties,
            "90s" => Decade::Nineties,
            "2000s" => Decade::TwoThousands,
            "2010s" => Decade::TwentyTens,
            "Recent" => Decade::Recent,
            _ => Decade::Recent,
        }
    }
}

/*
    Decade enum for filtering by Decade
*/
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub enum Decade {
    Classic,
    Fifties,
    Sixties,
    Seventies,
    Eighties,
    Nineties,
    TwoThousands,
    TwentyTens,
    Recent,
}

impl Decade {
    // Map decade enum to a tuple date range. This is passed into the /discover endpoint to filter by release year
    pub fn year_range(&self) -> (String, String) {
        match self {
            Decade::Classic => (String::from("1900"), String::from("1949")),
            Decade::Fifties => (String::from("1950"), String::from("1959")),
            Decade::Sixties => (String::from("1960"), String::from("1969")),
            Decade::Seventies => (String::from("1970"), String::from("1979")),
            Decade::Eighties => (String::from("1980"), String::from("1989")),
            Decade::Nineties => (String::from("1990"), String::from("1999")),
            Decade::TwoThousands => (String::from("2000"), String::from("2009")),
            Decade::TwentyTens => (String::from("2010"), String::from("2019")),
            Decade::Recent => (String::from("2020"), String::from("2024")),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Movie {
    pub id: i64,
    pub overview: String,
    //popularity: f64,
    pub poster_path: Option<String>,
    pub release_date: String,
    pub title: String,
    //vote_average: f64,
    //vote_count: i64,
}

#[derive(Debug, Deserialize)]
pub struct SearchByTitleResponse {
    pub results: Vec<Movie>,
}

#[derive(Debug, Deserialize, Clone, Serialize, PartialEq, Eq)]
pub struct WatchProvider {
    pub logo_path: String,
    pub provider_id: i32,
    pub provider_name: String,
}

/* Represents a JSON object of a country/region - contains a list of movie providers broken down by type: */
/* flatrate - subscription based services like Netflix, HBO, etc. */
/* buy - services where movies can be bought like Vudu, Google Play Movies, etc */
/* rent - services where movies can be rented, like Vudu, Google Play Movies, etc */
#[derive(Debug, Deserialize)]
pub struct WatchProviderRegion {
    pub flatrate: Vec<WatchProvider>,
    //buy: Vec<WatchProvider>,
    //rent: Vec<WatchProvider>,
}

/* Represents a JSON object containing supported countries/regions */
#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct WatchProviderRegions {
    pub us: WatchProviderRegion,
}

#[derive(Debug, Deserialize)]
pub struct GetWatchProvidersResponse {
    pub results: WatchProviderRegions,
}

#[derive(Debug, Deserialize, Clone, Serialize, PartialEq, Eq)]
pub struct Genre {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct GetGenresResponse {
    pub genres: Vec<Genre>,
}

#[derive(Debug, Deserialize)]
pub struct GetProvidersResponse {
    pub results: Vec<WatchProvider>,
}

#[derive(Debug, Deserialize)]
pub struct GetRecommendationsResponse {
    pub results: Vec<Movie>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MovieRecommendation {
    pub movie: Movie,
    pub providers: Vec<WatchProvider>,
}

impl CardData for MovieRecommendation {
    fn get_id(&self) -> i32 {
        self.movie.id as i32
    }

    fn get_display(&self) -> String {
        self.movie.title.clone()
    }

    fn get_body(&self) -> impl IntoView {
        view! {
            {self
                .providers
                .clone()
                .into_iter()
                .map(|provider| {
                    view! {
                        <span class="badge rounded-pill text-bg-secondary">
                            {provider.provider_name}
                        </span>
                    }
                })
                .collect_view()}
        }
    }

    fn has_body(&self) -> bool {
        true
    }

    fn get_logo_path(&self) -> String {
        match &self.movie.poster_path{
            Some(path) => format!("https://image.tmdb.org/t/p/w500/{}", &path),
            None => "https://raw.githubusercontent.com/rhysbratti/moodie_assets/master/question_mark.png".to_string()
        }
    }
}

pub trait CardData {
    fn get_id(&self) -> i32;
    fn get_display(&self) -> String;
    fn get_body(&self) -> impl IntoView;
    fn get_logo_path(&self) -> String;
    fn has_body(&self) -> bool;
}

impl CardData for WatchProvider {
    fn get_id(&self) -> i32 {
        self.provider_id
    }
    fn get_display(&self) -> String {
        self.provider_name.clone()
    }

    fn get_body(&self) -> impl IntoView {
        view! {}
    }

    fn get_logo_path(&self) -> String {
        format!(
            "https://raw.githubusercontent.com/rhysbratti/moodie_assets/master/{}/logo.png",
            str::replace(&self.provider_name, " ", "_")
        )
    }

    fn has_body(&self) -> bool {
        false
    }
}

impl CardData for Decade {
    fn get_id(&self) -> i32 {
        match self {
            Decade::Classic => 1,
            Decade::Fifties => 2,
            Decade::Sixties => 3,
            Decade::Seventies => 4,
            Decade::Eighties => 5,
            Decade::Nineties => 6,
            Decade::TwoThousands => 7,
            Decade::TwentyTens => 8,
            Decade::Recent => 9,
        }
    }
    fn get_display(&self) -> String {
        match self {
            Decade::Classic => String::from("Classics"),
            Decade::Fifties => String::from("50s"),
            Decade::Sixties => String::from("60s"),
            Decade::Seventies => String::from("70s"),
            Decade::Eighties => String::from("80s"),
            Decade::Nineties => String::from("90s"),
            Decade::TwoThousands => String::from("2000s"),
            Decade::TwentyTens => String::from("2010s"),
            Decade::Recent => String::from("Recent"),
        }
    }

    fn get_body(&self) -> impl IntoView {
        view! {}
    }

    fn has_body(&self) -> bool {
        false
    }

    fn get_logo_path(&self) -> String {
        String::from("/")
    }
}

pub async fn get_decades() -> Vec<Decade> {
    vec![
        Decade::Classic,
        Decade::Fifties,
        Decade::Sixties,
        Decade::Seventies,
        Decade::Eighties,
        Decade::Nineties,
        Decade::TwoThousands,
        Decade::TwentyTens,
        Decade::Recent,
    ]
}

pub async fn get_movies() -> Vec<MovieRecommendation> {
    vec![
        MovieRecommendation {
            movie: Movie {
                id: 140300,
                overview: String::from("Example"),
                poster_path: Some(String::from("/oajNi4Su39WAByHI6EONu8G8HYn.jpg")),
                release_date: String::from("2016-01-23"),
                title: String::from("Kung Fu Panda 3"),
            },
            providers: get_watch_providers().await,
        },
        MovieRecommendation {
            movie: Movie {
                id: 383498,
                overview: String::from("Example"),
                poster_path: Some(String::from("/to0spRl1CMDvyUbOnbb4fTk3VAd.jpg")),
                release_date: String::from("2016-01-23"),
                title: String::from("Deadpool 2"),
            },
            providers: get_watch_providers().await,
        },
        MovieRecommendation {
            movie: Movie {
                id: 293660,
                overview: String::from("Example"),
                poster_path: Some(String::from("/fSRb7vyIP8rQpL0I47P3qUsEKX3.jpg")),
                release_date: String::from("2016-01-23"),
                title: String::from("Deadpool"),
            },
            providers: get_watch_providers().await,
        },
        MovieRecommendation {
            movie: Movie {
                id: 177572,
                overview: String::from("Example"),
                poster_path: Some(String::from("/2mxS4wUimwlLmI1xp6QW6NSU361.jpg")),
                release_date: String::from("2016-01-23"),
                title: String::from("Big Hero 6"),
            },
            providers: get_watch_providers().await,
        },
        MovieRecommendation {
            movie: Movie {
                id: 10138,
                overview: String::from("Example"),
                poster_path: Some(String::from("/6WBeq4fCfn7AN0o21W9qNcRF2l9.jpg")),
                release_date: String::from("2016-01-23"),
                title: String::from("Iron Man 2"),
            },
            providers: get_watch_providers().await,
        },
        MovieRecommendation {
            movie: Movie {
                id: 260513,
                overview: String::from("Example"),
                poster_path: Some(String::from("/9lFKBtaVIhP7E2Pk0IY1CwTKTMZ.jpg")),
                release_date: String::from("2016-01-23"),
                title: String::from("Incredibles 2"),
            },
            providers: get_watch_providers().await,
        },
        MovieRecommendation {
            movie: Movie {
                id: 246655,
                overview: String::from("Example"),
                poster_path: Some(String::from("/2mtQwJKVKQrZgTz49Dizb25eOQQ.jpg")),
                release_date: String::from("2016-01-23"),
                title: String::from("X-Men: Apocalypse"),
            },
            providers: get_watch_providers().await,
        },
        MovieRecommendation {
            movie: Movie {
                id: 141052,
                overview: String::from("Example"),
                poster_path: Some(String::from("/eifGNCSDuxJeS1loAXil5bIGgvC.jpg")),
                release_date: String::from("2016-01-23"),
                title: String::from("Justice League"),
            },
            providers: get_watch_providers().await,
        },
        MovieRecommendation {
            movie: Movie {
                id: 396535,
                overview: String::from("Example"),
                poster_path: Some(String::from("/vNVFt6dtcqnI7hqa6LFBUibuFiw.jpg")),
                release_date: String::from("2016-01-23"),
                title: String::from("Train to Busan"),
            },
            providers: get_watch_providers().await,
        },
        MovieRecommendation {
            movie: Movie {
                id: 76338,
                overview: String::from("Example"),
                poster_path: None,
                release_date: String::from("2016-01-23"),
                title: String::from("Thor: The Dark World"),
            },
            providers: get_watch_providers().await,
        },
    ]
}

pub async fn get_watch_providers() -> Vec<WatchProvider> {
    thread::sleep(Duration::from_secs(1));
    vec![
        WatchProvider {
            provider_id: 8,
            provider_name: "Netflix".to_string(),
            logo_path: "/".to_string(),
        },
        WatchProvider {
            provider_id: 13,
            provider_name: "Hulu".to_string(),
            logo_path: "/".to_string(),
        },
        WatchProvider {
            provider_id: 15,
            provider_name: "Apple TV".to_string(),
            logo_path: "/".to_string(),
        },
        WatchProvider {
            provider_id: 16,
            provider_name: "Peacock".to_string(),
            logo_path: "/".to_string(),
        },
        WatchProvider {
            provider_id: 17,
            provider_name: "Amazon Prime Video".to_string(),
            logo_path: "/".to_string(),
        },
        WatchProvider {
            provider_id: 18,
            provider_name: "Max".to_string(),
            logo_path: "/".to_string(),
        },
        WatchProvider {
            provider_id: 19,
            provider_name: "Disney Plus".to_string(),
            logo_path: "/".to_string(),
        },
        WatchProvider {
            provider_id: 20,
            provider_name: "Tubi".to_string(),
            logo_path: "/".to_string(),
        },
        WatchProvider {
            provider_id: 21,
            provider_name: "Crunchyroll".to_string(),
            logo_path: "/".to_string(),
        },
        WatchProvider {
            provider_id: 22,
            provider_name: "Paramount Plus".to_string(),
            logo_path: "/".to_string(),
        },
    ]
}

pub trait AddOrRemove<T> {
    fn add_or_remove(&mut self, entry: T);
}

impl<T: PartialEq> AddOrRemove<T> for Vec<T> {
    fn add_or_remove(&mut self, entry: T) {
        if self.contains(&entry) {
            self.retain(|e| *e != entry);
        } else {
            self.push(entry)
        }
    }
}
