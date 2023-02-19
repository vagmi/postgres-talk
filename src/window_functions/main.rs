use std::collections::HashMap;

use rand::prelude::*;
use chrono::{DateTime, Utc, Days};
use fake::{Fake, faker::chrono::en::DateTimeBetween};
use lazy_static::lazy_static;
use uuid::Uuid;
use postgres_talk::utils::{reset_database, run_structure, establish_connection};
use anyhow::{Result, anyhow};
use sqlx::PgPool;
use rand_distr::Normal;

const STRUCTURE_SQL: &'static str = include_str!("./sql/structure.sql");
lazy_static! {
    static ref VIDEO_IDS: Vec<Uuid> = {
        let mut video_ids: Vec<Uuid> = vec![];
        for _ in 0..100 {
            video_ids.push(Uuid::new_v4());
        }
        video_ids
    };
    static ref USER_IDS: Vec<Uuid> = {
        let mut user_ids: Vec<Uuid> = vec![];
        for _ in 0..10_000 {
            user_ids.push(Uuid::new_v4());
        }
        user_ids
    };
}

async fn seed(pool: PgPool) -> Result<()> {
    let mut rng = thread_rng();
    let mut tx = pool.begin().await?;
    let mut created_times: HashMap<Uuid, DateTime<Utc>> = HashMap::new();

    for n in 0..100 {
        let norm = Normal::new(0.5, 0.25)?;
        let uid_offset = (norm.sample(&mut rng)*50 as f64).floor() as usize;
        let uid = USER_IDS.iter().nth(uid_offset+300)
            .ok_or(anyhow!("Boom user id "))?;
        let vid = VIDEO_IDS.iter().nth(n)
            .ok_or(anyhow!("Boom video id "))?;
        let mut end_dt: DateTime<Utc> = Utc::now();
        end_dt = end_dt.checked_sub_days(Days::new(30)).ok_or(anyhow!("date calc"))?;
        let start_dt = end_dt.checked_sub_days(Days::new(365))
            .ok_or(anyhow!("unable to sub days"))?;

        let created_at: DateTime<Utc> = DateTimeBetween(start_dt.clone(), end_dt.clone()).fake();
        created_times.insert(vid.clone(), created_at.clone());
        sqlx::query(r#"
                    insert into videos(id, creator_id, created_at) 
                    values ($1, $2, $3)
                    "#).bind(vid).bind(uid).bind(created_at)
            .execute(&mut tx).await?;
    }

    for _ in 0..100_000 {
        let norm = Normal::new(0.5, 0.25)?;
        let mut vid_offset = (norm.sample(&mut rng)*100.0 as f64).floor() as usize;
        if vid_offset > 99 { vid_offset = 99; }
        let vid = VIDEO_IDS.iter().nth(vid_offset)
            .ok_or(anyhow!("Boom video id "))?;
        let mut uid_offset = (norm.sample(&mut rng)*10_000.0 as f64).floor() as usize;
        if uid_offset > 9999 { uid_offset = 9999; }
        let uid = USER_IDS.iter().nth(uid_offset)
            .ok_or(anyhow!("Boom user id "))?;
        let secs = rng.gen_range(0..600);
        let end_dt: DateTime<Utc> = Utc::now();
        let start_dt = created_times.get(vid).ok_or(anyhow!("unable to find video"))?;
        let played_at: DateTime<Utc> = DateTimeBetween(start_dt.clone(), end_dt.clone()).fake();
        sqlx::query(r#"
                    insert into views(video_id, user_id, time_played, created_at) 
                    values ($1, $2, $3, $4)
                    "#).bind(vid).bind(uid).bind(secs).bind(played_at)
            .execute(&mut tx).await?;
    }
    tx.commit().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    reset_database("window_functions").await?;
    run_structure("window_functions", STRUCTURE_SQL).await?;
    let pool = establish_connection("window_functions").await?;
    seed(pool.clone()).await?;
    Ok(())
}
