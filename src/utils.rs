use sqlx::{postgres::PgPoolOptions, PgPool, Executor};
use tokio::process::Command;
use anyhow::{Result, bail};

pub async fn drop_db(db_name: &str) -> Result<()> {
    let mut drop_child = Command::new("dropdb").arg("--if-exists").arg(db_name).spawn()?;
    let status = drop_child.wait().await?;

    match status.success() {
        true => Ok(()),
        false => bail!("unable to drop db")
    }
}

pub async fn create_db(db_name: &str) -> Result<()> {
    let mut create_child = Command::new("createdb").arg(db_name).spawn()?;
    let status = create_child.wait().await?;

    match status.success() {
        true => Ok(()),
        false => bail!("unable to create db")
    }
}

pub async fn reset_database(db_name: &str) -> Result<()> {
    drop_db(db_name).await?;
    create_db(db_name).await
}

pub async fn run_structure(db_name: &str, sql: &str) -> Result<()> {
    let pool = PgPoolOptions::new().connect(format!("postgres:///{}?sslmode=disable", db_name).as_str()).await?;
    let mut tx = pool.begin().await?;
    tx.execute(sql).await?;
    tx.commit().await?;
    Ok(())
}

pub async fn establish_connection(db_name: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new().connect(format!("postgres:///{}?sslmode=disable", db_name).as_str()).await?;
    Ok(pool)
}
