use anyhow::Result;
use fake::{Fake, faker::lorem::en::Words, locales::EN, faker::chrono::raw::{DateTime as FakeDateTime, DateTimeBefore, DateTimeAfter, DateTimeBetween}};
use postgres_talk::utils::{run_structure, reset_database, establish_connection};
use chrono::{DateTime, Utc, Days};
use sqlx::PgPool;

const STRUCTURE_SQL: &'static str = include_str!("./structure.sql");

async fn seed(pool: PgPool) -> Result<()> {
    let mut tx = pool.begin().await?;
    let end_dt: DateTime<Utc> = Utc::now();
    let start_dt: DateTime<Utc> = end_dt.checked_sub_days(Days::new(365)).unwrap();
    for _ in 0..10_0000 {
        let message: Vec<String> = Words(10..20).fake();
        let created_at: DateTime<Utc> = DateTimeBetween(EN, start_dt.clone(), end_dt.clone()).fake();
        sqlx::query("insert into logs(message, created_at) values($1, $2)")
              .bind(message.join(" "))
              .bind(created_at)
              .execute(&mut tx).await?;
    }
    tx.commit().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    reset_database("pivot_table").await?;
    run_structure("pivot_table", STRUCTURE_SQL).await?;
    let pool = establish_connection("pivot_table").await?;
    seed(pool.clone()).await?;
    Ok(())
}
