use actix_web::{get, web, HttpResponse};
use awc;
use serde::{Deserialize, Serialize};

use crate::errors::ServiceError;

lazy_static::lazy_static! {
  pub static ref TMDB_API_KEY: String = std::env::var("TMDB_API_KEY").unwrap();
}

const SEARCH_MOVIE_BASE: &str = "https://api.themoviedb.org/3/search/movie?api_key=";
const SEARCH_SHOW_BASE: &str = "https://api.themoviedb.org/3/search/tv?api_key=";
const QUERY_PARAM: &str = "&query=";
const PAGE_PARAM: &str = "&page=";
const LANG_PARAM: &str = "&language=";
const SHOW_YEAR_PARAM: &str = "&year="; // primary_release_year is an alternative?
const MOVIE_YEAR_PARAM: &str = "&first_air_date_year=";

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
    page: i32,
    results: Vec<T>,
    total_results: i32,
    total_pages: i32,
}

#[get("/films")]
pub async fn search_movies(info: web::Query<SearchInfo>) -> Result<HttpResponse, ServiceError> {
    let client = awc::Client::default();

    let mut url = SEARCH_MOVIE_BASE.to_owned() + &TMDB_API_KEY + QUERY_PARAM + &info.query;

    if let Some(page) = &info.page {
        url += PAGE_PARAM;
        url += &page.to_string();
    }
    if let Some(lang) = &info.lang {
        url += LANG_PARAM;
        url += lang;
    }
    if let Some(year) = &info.year {
        url += MOVIE_YEAR_PARAM;
        url += &year.to_string();
    }

    let req = client.get(url);

    let mut res = req.send().await?;

    let body = res.json::<Results<Movie>>().await?;

    Ok(HttpResponse::Ok().json(body))
}

#[get("/shows")]
pub async fn search_shows(info: web::Query<SearchInfo>) -> Result<HttpResponse, ServiceError> {
    let client = awc::Client::default();

    let mut url = SEARCH_SHOW_BASE.to_owned() + &TMDB_API_KEY + QUERY_PARAM + &info.query;

    if let Some(page) = &info.page {
        url += PAGE_PARAM;
        url += &page.to_string();
    }
    if let Some(lang) = &info.lang {
        url += LANG_PARAM;
        url += &lang;
    }
    if let Some(year) = &info.year {
        url += SHOW_YEAR_PARAM;
        url += &year.to_string();
    }

    let req = client.get(url);

    let mut res = req.send().await?;

    let body = res.json::<Results<Show>>().await?;

    Ok(HttpResponse::Ok().json(body))
}
