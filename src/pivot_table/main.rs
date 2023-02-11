use postgres_talk::utils::{run_structure, reset_database};

const structure_sql: &'static str = include_str!("./structure.sql");

#[tokio::main]
async fn main() {
    reset_database("pivot_table").await.unwrap();
    run_structure("pivot_table", structure_sql).await.unwrap();
}
