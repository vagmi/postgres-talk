use std::net::SocketAddr;
use axum::extract::{State, Path, Query};
use axum::http::{StatusCode, HeaderMap, HeaderValue, header::CONTENT_TYPE};
use postgres_talk::utils::{establish_connection, run_structure};
use anyhow::Result;
use axum::{Router, Server};
use axum::routing::get;
use serde::Deserialize;
use serde_json::Value;
use sqlx::types::Json as SqlJson;
use sqlx::PgPool;


const STRUCTURE_SQL: &'static str = include_str!("./sql/structure.sql");
#[derive(sqlx::FromRow, Deserialize, Debug)]
struct MovieResult {
    data: SqlJson<Value>
}

#[derive(Deserialize)]
struct SearchQuery {
    q: String
}

async fn search_movies(State(pool): State<PgPool>, Query(query): Query<SearchQuery>) -> (StatusCode, HeaderMap, String) {
    const QUERY: &'static str = include_str!("./sql/query_search.sql");
    let row = sqlx::query_as::<_, MovieResult>(QUERY).bind(query.q).fetch_one(&pool).await;
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    if let Ok(r) = row {
        return (StatusCode::OK, headers,  r.data.to_string());
    } else {
        return (StatusCode::INTERNAL_SERVER_ERROR, headers, "boom".to_string());
    }

}
async fn get_movie(State(pool): State<PgPool>, Path(id): Path<i64>) -> (StatusCode, HeaderMap, String) {
    const QUERY: &'static str = include_str!("./sql/query.sql");
    let row = sqlx::query_as::<_, MovieResult>(QUERY).bind(id).fetch_one(&pool).await;
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    if let Ok(r) = row {
        return (StatusCode::OK, headers,  r.data.to_string());
    } else {
        return (StatusCode::INTERNAL_SERVER_ERROR, headers, "boom".to_string());
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    run_structure("omdb", STRUCTURE_SQL).await?;
    let pool = establish_connection("omdb").await?;
    let router = Router::new()
                  .route("/movies/:id", get(get_movie))
                  .route("/movies/search", get(search_movies))
                  .with_state(pool.clone());
    let addr: SocketAddr = "0.0.0.0:3000".parse()?;
    Server::bind(&addr).serve(router.into_make_service()).await?;
    Ok(())
}
