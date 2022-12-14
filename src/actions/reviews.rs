use diesel::{associations::HasTable, ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::Deserialize;

use crate::{
    errors::DbError,
    models::{EditReview, MediaCategory, NewReview, Review, WatchStatus},
    pagination::{Paginate, PaginatedResults},
    PooledConn,
};

#[derive(Deserialize)]
pub struct ReviewsQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub sort_by: Option<String>,
    pub user_id: Option<i32>,
    pub category: Option<MediaCategory>,
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
        query = match sort_by.as_ref() {
            "tmdb_id.asc" => query.order(tmdb_id.asc()),
            "tmdb_id.desc" => query.order(tmdb_id.desc()),
            "created_at.asc" => query.order(created_at.asc()),
            "created_at.desc" => query.order(created_at.desc()),
            "updated_at.asc" => query.order(updated_at.asc()),
            "updated_at.desc" => query.order(updated_at.desc()),
            _ => query.order(tmdb_id.asc()),
        }
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
}

pub fn create_review_for_user(
    conn: &mut PooledConn,
    idx: i32,
    input_review: InputReview,
) -> Result<Review, DbError> {
    use crate::schema::reviews::dsl::*;

    let new_film_review = NewReview {
        category: input_review.category,
        tmdb_id: input_review.tmdb_id,
        status: input_review.status,
        user_id: idx,
        text: "",
        fun_before: false,
        fun_during: false,
        fun_after: false,
    };

    let res = diesel::insert_into(reviews)
        .values(new_film_review)
        .get_result::<Review>(conn)?;

    Ok(res)
}

pub fn update_review(
    conn: &mut PooledConn,
    user_idx: i32,
    idx: i32,
    cat: MediaCategory,
    edits: EditReview,
) -> Result<Review, DbError> {
    use crate::schema::reviews::dsl::*;

    let review = diesel::update(
        reviews
            .filter(user_id.eq(user_idx))
            .filter(tmdb_id.eq(idx))
            .filter(category.eq(cat)),
    )
    .set(edits)
    .get_result(conn)?;

    Ok(review)
}

pub fn delete_review(
    conn: &mut PooledConn,
    user_idx: i32,
    idx: i32,
    cat: MediaCategory,
) -> Result<usize, DbError> {
    use crate::schema::reviews::dsl::*;

    let deleted = diesel::delete(
        reviews
            .filter(user_id.eq(user_idx))
            .filter(tmdb_id.eq(idx))
            .filter(category.eq(cat)),
    )
    .execute(conn)?;

    Ok(deleted)
}
