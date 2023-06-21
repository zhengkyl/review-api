// Modified from https://github.com/diesel-rs/diesel/blob/master/examples/postgres/advanced-blog-cli/src/pagination.rs

use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::*;
use diesel::query_dsl::methods::LoadQuery;
use diesel::sql_types::BigInt;
use serde::Serialize;

const DEFAULT_PER_PAGE: i64 = 10;
const MAX_PER_PAGE: i64 = 50;

#[derive(Serialize)]
pub struct PaginatedResults<U> {
    results: Vec<U>,
    page: i64,
    total_pages: i64,
    total_results: i64,
}

pub trait Paginate: Sized {
    fn paginate(self, page: i64, per_page: i64) -> Paginated<Self>;
    fn paginate_safe(self, page: Option<i64>, per_page: Option<i64>) -> Paginated<Self>;
}

impl<T> Paginate for T {
    fn paginate(self, page: i64, per_page: i64) -> Paginated<Self> {
        Paginated {
            query: self,
            page,
            per_page,
            offset: (page - 1) * per_page,
        }
    }
    fn paginate_safe(self, page: Option<i64>, per_page: Option<i64>) -> Paginated<Self> {
        let page = match page {
            Some(page) if page > 0 => page,
            _ => 1,
        };

        let per_page = match per_page {
            Some(per_page) if per_page > 0 && per_page <= MAX_PER_PAGE => per_page,
            _ => DEFAULT_PER_PAGE,
        };

        self.paginate(page, per_page)
    }
}

#[derive(Debug, Clone, Copy, QueryId)]
pub struct Paginated<T> {
    query: T,
    page: i64,
    per_page: i64,
    offset: i64,
}

impl<T> Paginated<T> {
    pub fn load_paginated<'a, U>(self, conn: &mut PgConnection) -> QueryResult<PaginatedResults<U>>
    where
        Self: LoadQuery<'a, PgConnection, (U, i64)>,
    {
        let page = self.page;
        let per_page = self.per_page;

        let records = self.load::<(U, i64)>(conn)?;
        let total_results = records.get(0).map(|x| x.1).unwrap_or(0);
        let results = records.into_iter().map(|x| x.0).collect();
        let total_pages = (total_results as f64 / per_page as f64).ceil() as i64;

        Ok(PaginatedResults {
            results,
            page,
            total_pages,
            total_results,
        })
    }
}

impl<T: Query> Query for Paginated<T> {
    type SqlType = (T::SqlType, BigInt);
}

impl<T> RunQueryDsl<PgConnection> for Paginated<T> {}

impl<T> QueryFragment<Pg> for Paginated<T>
where
    T: QueryFragment<Pg>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Pg>) -> QueryResult<()> {
        // https://stackoverflow.com/questions/28888375/run-a-query-with-a-limit-offset-and-also-get-the-total-number-of-rows
        // When offset > rows, this returns a row of nulls + full_count
        // However, can't converted nulls into non nullable...

        // out.push_sql("WITH cte AS ( SELECT * FROM (");
        // self.query.walk_ast(out.reborrow())?;
        // out.push_sql(") t ) ");

        // out.push_sql("SELECT * FROM (");
        // out.push_sql(" TABLE cte");
        // out.push_sql(" LIMIT ");
        // out.push_bind_param::<BigInt, _>(&self.per_page)?;
        // out.push_sql(" OFFSET ");
        // out.push_bind_param::<BigInt, _>(&self.offset)?;

        // out.push_sql(" ) sub ");
        // out.push_sql("RIGHT JOIN (SELECT count(*) FROM cte) c(full_count) ON true;");

        // Original
        // This returns a count of 0 if offset > rows
        out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") t LIMIT ");
        out.push_bind_param::<BigInt, _>(&self.per_page)?;
        out.push_sql(" OFFSET ");
        out.push_bind_param::<BigInt, _>(&self.offset)?;
        Ok(())
    }
}
