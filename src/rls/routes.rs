use sqlx::{FromRow, PgPool};
use axum_macros::debug_handler;
use serde::{Serialize, Deserialize};
use axum::{http::{HeaderMap, StatusCode, HeaderName}, extract::{self, State, Path}, Json};

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct User {
    id: i64,
    handle: String,
    email: Option<String>
}

macro_rules! set_session {
    ($req_headers:ident, $tx: ident) => {

        let val = $req_headers.get(HeaderName::from_static("user"));
        tracing::info!("the user is {:?}", val);
        match val {
            Some(user_val) => {
                let user_name = user_val.to_str().unwrap();
                match user_name  {
                    "admin" => {
                        sqlx::query("set role to app_rls_admin").execute(&mut $tx).await.unwrap();
                        sqlx::query("select set_config('rls.username', 'admin', true)").execute(&mut $tx).await.unwrap();
                    }
                    _ => {
                        sqlx::query("set role to app_rls_user").execute(&mut $tx).await.unwrap();
                        sqlx::query("select set_config('rls.username', $1, true)").bind(user_name).execute(&mut $tx).await.unwrap();
                    }
                }
            },
            None => {
                sqlx::query("set role to app_rls_anonymous").execute(&mut $tx).await.unwrap();
            }
        }
    }
}

#[debug_handler]
pub async fn get_users(State(pool): State<PgPool>, req_headers: HeaderMap) -> (StatusCode, Json<Vec<User>>) {
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

#[derive(Serialize, Deserialize)]
pub struct UpdateUser {
    email: String
}

#[debug_handler]
pub async fn update_user( State(pool): State<PgPool>, 
                          Path(user_id): Path<i64>, 
                          req_headers: HeaderMap,
                          extract::Json(body): extract::Json<UpdateUser>, 
                        ) -> (StatusCode, Json<Vec<User>>) {
    let mut tx = pool.begin().await.unwrap();
    set_session!(req_headers, tx);
    let res= sqlx::query_as::<_, User>("update users set email=$1 where id=$2 returning *")
        .bind(body.email).bind(user_id)
        .fetch_all(&mut tx).await;

    match res {
        Ok(users) => (StatusCode::OK, Json(users)),
        Err(e) => {
            tracing::error!("An error occured {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(vec!()))
        }
    }
}
