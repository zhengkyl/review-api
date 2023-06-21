use diesel::{associations::HasTable, ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::Deserialize;

use crate::{
    errors::DbError,
    models::{EditReview, MediaCategory, NewReview, Review, WatchStatus},
    pagination::{Paginate, PaginatedResults},
    PooledConn,
};

#[derive(Deserialize)]
pub enum SortBy {
    #[serde(rename = "tmdb_id.asc")]
    TmdbIdAsc,
    #[serde(rename = "tmdb_id.desc")]
    TmdbIdDesc,
    #[serde(rename = "created_at.asc")]
    CreatedAtAsc,
    #[serde(rename = "created_at.desc")]
    CreatedAtDesc,
    #[serde(rename = "updated_at.asc")]
    UpdatedAtAsc,
    #[serde(rename = "updated_at.desc")]
    UpdatedAtDesc,
}

#[derive(Deserialize)]
pub struct ReviewsQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub sort_by: Option<SortBy>,
    pub user_id: Option<i32>,
    pub category: Option<MediaCategory>,
    pub tmdb_id: Option<i32>,
    pub season: Option<i32>,
    pub status: Option<WatchStatus>,
    pub fun_before: Option<bool>,
    pub fun_during: Option<bool>,
    pub fun_after: Option<bool>,
}

pub fn get_all_reviews(
    conn: &mut PooledConn,
    params: ReviewsQuery,
) -> Result<PaginatedResults<Review>, DbError> {
    use crate::schema::reviews::dsl::*;

    let mut query = reviews::table().into_boxed();

    if let Some(user_id_in) = params.user_id {
        query = query.filter(user_id.eq(user_id_in));
    }
    if let Some(category_in) = params.category {
        query = query.filter(category.eq(category_in));
    }
    if let Some(tmdb_id_in) = params.tmdb_id {
        query = query.filter(tmdb_id.eq(tmdb_id_in));
    }
    if let Some(season_in) = params.season {
        query = query.filter(season.eq(season_in))
    }
    if let Some(status_in) = params.status {
        query = query.filter(status.eq(status_in));
    }
    if let Some(fun_before_in) = params.fun_before {
        query = query.filter(fun_before.eq(fun_before_in));
    }
    if let Some(fun_during_in) = params.fun_during {
        query = query.filter(fun_during.eq(fun_during_in));
    }
    if let Some(fun_after_in) = params.fun_after {
        query = query.filter(fun_after.eq(fun_after_in));
    }

    if let Some(sort_by) = params.sort_by {
        query = match sort_by {
            SortBy::TmdbIdAsc => query.order(tmdb_id.asc()),
            SortBy::TmdbIdDesc => query.order(tmdb_id.desc()),
            SortBy::CreatedAtAsc => query.order(created_at.asc()),
            SortBy::CreatedAtDesc => query.order(created_at.desc()),
            SortBy::UpdatedAtAsc => query.order(updated_at.asc()),
            SortBy::UpdatedAtDesc => query.order(updated_at.desc()),
        }
    } else {
        query = query.order(updated_at.desc())
    }

    let results = query
        .paginate_safe(params.page, params.per_page)
        .load_paginated(conn)?;

    Ok(results)
}

#[derive(Deserialize, Debug)]
pub struct InputReview {
    tmdb_id: i32,
    category: MediaCategory,
    status: WatchStatus,
    season: Option<i32>,
}

pub fn create_review_for_user(
    conn: &mut PooledConn,
    idx: i32,
    input_review: InputReview,
) -> Result<Review, DbError> {
    use crate::schema::reviews::dsl::*;

    let new_review = NewReview {
        user_id: idx,
        tmdb_id: input_review.tmdb_id,
        category: input_review.category,
        season: input_review.season,
        status: input_review.status,
        text: "",
        fun_before: false,
        fun_during: false,
        fun_after: false,
    };

    let res = diesel::insert_into(reviews)
        .values(new_review)
        .get_result::<Review>(conn)?;

    Ok(res)
}

pub fn update_review(
    conn: &mut PooledConn,
    user_id_v: i32,
    tmdb_id_v: i32,
    category_v: MediaCategory,
    season_v: Option<i32>,
    edits: EditReview,
) -> Result<Review, DbError> {
    use crate::schema::reviews::dsl::*;

    let season_v = season_v.unwrap_or(-1);

    let review = diesel::update(
        reviews
            .filter(user_id.eq(user_id_v))
            .filter(tmdb_id.eq(tmdb_id_v))
            .filter(category.eq(category_v))
            .filter(season.eq(season_v)),
    )
    .set(edits)
    .get_result(conn)?;

    Ok(review)
}

pub fn delete_review(
    conn: &mut PooledConn,
    user_id_v: i32,
    tmdb_id_v: i32,
    category_v: MediaCategory,
    season_v: Option<i32>,
) -> Result<usize, DbError> {
    use crate::schema::reviews::dsl::*;

    let season_v = season_v.unwrap_or(-1);

    let deleted = diesel::delete(
        reviews
            .filter(user_id.eq(user_id_v))
            .filter(tmdb_id.eq(tmdb_id_v))
            .filter(category.eq(category_v))
            .filter(season.eq(season_v)),
    )
    .execute(conn)?;

    Ok(deleted)
}
