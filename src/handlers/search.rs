use actix_web::{get, web, HttpResponse};
use awc;
use serde::{Deserialize, Serialize};

use crate::errors::ServiceError;

lazy_static::lazy_static! {
  pub static ref TMDB_API_KEY: String = std::env::var("TMDB_API_KEY").unwrap();
}

const SEARCH_FILM_BASE: &str = "https://api.themoviedb.org/3/search/movie?";
const SEARCH_SHOW_BASE: &str = "https://api.themoviedb.org/3/search/tv?";
const API_PARAM: &str = "api_key";
const QUERY_PARAM: &str = "query";
const PAGE_PARAM: &str = "page";
const LANG_PARAM: &str = "language";
const FILM_YEAR_PARAM: &str = "year"; // primary_release_year is an alternative?
const SHOW_YEAR_PARAM: &str = "first_air_date_year";

#[derive(Deserialize)]
pub struct SearchInfo {
    query: String,
    page: Option<i32>,
    lang: Option<String>,
    year: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Movie {
    id: i32,
    title: String,
    original_title: String,
    original_language: String,
    release_date: String,
    overview: String,
    poster_path: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Show {
    id: i32,
    name: String,
    original_name: String,
    original_language: String,
    first_air_date: String,
    overview: String,
    poster_path: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Results<T> {
    results: Vec<T>,
    page: i32,
    total_pages: i32,
    total_results: i32,
}

#[get("/Film")]
pub async fn search_movies(info: web::Query<SearchInfo>) -> Result<HttpResponse, ServiceError> {
    let client = awc::Client::default();

    let mut params = vec![
        (API_PARAM, TMDB_API_KEY.to_string()),
        (QUERY_PARAM, info.query.to_string()),
    ];

    if let Some(page) = &info.page {
        params.push((PAGE_PARAM, page.to_string()));
    }
    if let Some(lang) = &info.lang {
        params.push((LANG_PARAM, lang.to_string()));
    }
    if let Some(year) = &info.year {
        params.push((FILM_YEAR_PARAM, year.to_string()));
    }

    let Ok(path_query) = serde_urlencoded::to_string(params) else {
        return Err(ServiceError::new(400, "Invalid search params"));
    };

    let req = client.get(SEARCH_FILM_BASE.to_owned() + &path_query);

    let mut res = req.send().await?;

    let body = res.json::<Results<Movie>>().await?;

    Ok(HttpResponse::Ok().json(body))
}

#[get("/Show")]
pub async fn search_shows(info: web::Query<SearchInfo>) -> Result<HttpResponse, ServiceError> {
    let client = awc::Client::default();

    let mut params = vec![
        (API_PARAM, TMDB_API_KEY.to_string()),
        (QUERY_PARAM, info.query.to_string()),
    ];

    if let Some(page) = &info.page {
        params.push((PAGE_PARAM, page.to_string()));
    }
    if let Some(lang) = &info.lang {
        params.push((LANG_PARAM, lang.to_string()));
    }
    if let Some(year) = &info.year {
        params.push((SHOW_YEAR_PARAM, year.to_string()));
    }

    let Ok(path_query) = serde_urlencoded::to_string(params) else {
        return Err(ServiceError::new(400, "Invalid search params"));
    };

    let req = client.get(SEARCH_SHOW_BASE.to_owned() + &path_query);

    let mut res = req.send().await?;

    let body = res.json::<Results<Show>>().await?;

    Ok(HttpResponse::Ok().json(body))
}
