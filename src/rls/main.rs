use std::net::SocketAddr;
use axum::{Router, routing::get, http::{HeaderMap, StatusCode, HeaderName}, extract::State, Server, Json};
use fake::{Fake, faker::{internet::en::Username, lorem::en::Words}};
use postgres_talk::utils::{reset_database, run_structure, establish_connection};
use anyhow::Result;
use serde::{Serialize, Deserialize};
use sqlx::{PgPool , FromRow};

const AUTH_SYSTEM: &'static str = include_str!("./sql/auth-system.sql");
const STRUCTURE_SQL: &'static str = include_str!("./sql/structure.sql");

#[derive(FromRow, Debug, Serialize, Deserialize)]
struct User {
    id: i64,
    handle: String
}


async fn seed(pool: PgPool) -> Result<()> {
    let mut tx = pool.begin().await?;
    for _ in 0..10 {
        let user: String = Username().fake();
        let user_id: (i64, ) = sqlx::query_as("insert into users(handle) values($1) returning id")
                                .bind(&user).fetch_one(&mut tx).await?;
        for _ in 0..10 {

            let title: Vec<String> = Words(2..10).fake();
            let body: Vec<String> = Words(20..50).fake();
            sqlx::query("insert into posts(user_id,title, body) values($1, $2, $3)")
                .bind(user_id.0)
                .bind(title.join(" "))
                .bind(body.join("\n"))
                .execute(&mut tx).await?;
        }
    }
    tx.commit().await?;
    Ok(())
}

macro_rules! set_session {
    ($req_headers:ident, $tx: ident) => {

        let val = $req_headers.get(HeaderName::from_static("user"));
        match val {
            Some(user_val) => {
                let user_name = user_val.to_str().unwrap();
                sqlx::query("set role to app_rls_user").execute(&mut $tx).await.unwrap();
                sqlx::query("select set_config('rls.username', $1, true)").bind(user_name).execute(&mut $tx).await.unwrap();
            },
            None => {
                sqlx::query("set role to app_rls_anonymous").execute(&mut $tx).await.unwrap();
            }
        }
    }
}

async fn get_users(State(pool): State<PgPool>, req_headers: HeaderMap) -> (StatusCode, Json<Vec<User>>) {
    let mut tx = pool.begin().await.unwrap();
    set_session!(req_headers, tx);
    let res= sqlx::query_as::<_,User>("select * from users").fetch_all(&mut tx).await;
    match res {
        Ok(users) => (StatusCode::OK, Json(users)),
        Err(e) => {
            tracing::error!("An error occured {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(vec!()))
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();
    reset_database("rls_db").await?;
    run_structure("rls_db", AUTH_SYSTEM).await?;
    run_structure("rls_db", STRUCTURE_SQL).await?;
    let pool = establish_connection("rls_db").await?;
    seed(pool.clone()).await?;

    let router = Router::new()
                  .route("/users", get(get_users))
                  .with_state(pool.clone());
    let addr: SocketAddr = "0.0.0.0:3000".parse()?;
    Server::bind(&addr).serve(router.into_make_service()).await?;

    // seed(pool.clone()).await?;
    Ok(())
}
