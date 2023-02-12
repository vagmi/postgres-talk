use chrono::{DateTime, Utc, Days};
use fake::{Fake, faker::chrono::en::DateTimeBetween};
use uuid::Uuid;
use async_trait::async_trait;
use postgres_talk::utils::{reset_database, run_structure, establish_connection};
use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use sqlx::{PgPool, PgExecutor};

const STRUCTURE_SQL: &'static str = include_str!("./sql/structure.sql");

#[async_trait]
trait Save {
    async fn save<'a, E>(&self, ex: E) -> Result<()>
         where E: 'a + PgExecutor<'a>;
}

macro_rules! implement_save {
    ($event: ty) => {
        #[async_trait]
        impl Save for $event {
            async fn save<'a, E>(&self, ex: E) -> Result<()> 
                where E: 'a + PgExecutor<'a>
            {
                let mut data = serde_json::to_value(self)?;
                let m = data.as_object_mut().ok_or(anyhow!("Unable to get as object"))?;
                m.insert("type".into(), Value::String(stringify!($event).into()));
                sqlx::query("insert into events(visitor_id, data, created_at) values($1, $2, $3)")
                     .bind(self.visitor_id.clone())
                     .bind(sqlx::types::Json(data))
                     .bind(self.event_time.clone())
                     .execute(ex).await?;
                Ok(())
            }
        }
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VisitedEvent {
    pub visitor_id: Uuid,
    pub event_time: DateTime<Utc>
}
implement_save!(VisitedEvent);

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisteredEvent {
    pub visitor_id: Uuid,
    pub event_time: DateTime<Utc>,
    pub user_id: u32
}
implement_save!(RegisteredEvent);

#[derive(Debug, Serialize, Deserialize)]
pub struct PaidUserEvent {
    pub visitor_id: Uuid,
    pub event_time: DateTime<Utc>,
    pub user_id: u32,
    pub plan_id: u32
}
implement_save!(PaidUserEvent);

async fn seed(pool: PgPool) -> Result<()> {
    let mut tx = pool.begin().await?;
    for _ in 0..10_0000 {
        let visitor_id = Uuid::new_v4();
        let end_dt: DateTime<Utc> = Utc::now();
        let start_dt : DateTime<Utc> = end_dt.checked_sub_days(Days::new(365)).ok_or(anyhow!("unable to sub days"))?;
        let event_time: DateTime<Utc> = DateTimeBetween(start_dt.clone(), end_dt.clone()).fake();
        let reg_prob: f32 = rand::random();
        let paid_prob: f32 = rand::random();
        let v_event = VisitedEvent { visitor_id: visitor_id.clone(), event_time: event_time.clone()};
        v_event.save(&mut tx).await?;
        if reg_prob > 0.3 {
            let user_id: u32 = rand::random();
            let reg_dt: DateTime<Utc> = DateTimeBetween(event_time, end_dt.clone()).fake();
            let reg_event = RegisteredEvent { visitor_id: visitor_id.clone(), event_time: reg_dt.clone(), user_id};
            reg_event.save(&mut tx).await?;
            if paid_prob > 0.5 {
                let paid_dt: DateTime<Utc> = DateTimeBetween(reg_dt, end_dt.clone()).fake();
                let paid_event = PaidUserEvent { visitor_id: visitor_id.clone(), event_time: paid_dt, user_id, plan_id: 2 };
                paid_event.save(&mut tx).await?;
            }
        }
    }
    tx.commit().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    reset_database("lateral_join").await?;
    run_structure("lateral_join", STRUCTURE_SQL).await?;
    let pool = establish_connection("lateral_join").await?;
    seed(pool.clone()).await?;
    Ok(())
}
