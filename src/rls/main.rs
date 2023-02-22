use std::net::SocketAddr;
use axum::{Router, routing::{get, post}, Server};
use fake::{Fake, faker::{internet::en::Username, lorem::en::Words}};
use postgres_talk::utils::{reset_database, run_structure, establish_connection};
use sqlx::PgPool;
use anyhow::Result;
use routes::{get_users, update_user};


const AUTH_SYSTEM: &'static str = include_str!("./sql/auth-system.sql");
const STRUCTURE_SQL: &'static str = include_str!("./sql/structure.sql");

mod routes;

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
                  .route("/users/:user_id", post(update_user))
                  .with_state(pool.clone());
    let addr: SocketAddr = "0.0.0.0:3000".parse()?;
    Server::bind(&addr).serve(router.into_make_service()).await?;

    Ok(())
}
