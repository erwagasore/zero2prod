use sqlx::types::chrono::Utc;
use sqlx::{Acquire, Postgres};
use tide::prelude::*;
use tide::{Response, StatusCode};
use tide_sqlx::SQLxRequestExt;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscribeFormData {
    name: String,
    email: String,
}

pub async fn health_check(mut _req: tide::Request<()>) -> tide::Result {
    let res = Response::new(StatusCode::Ok);
    Ok(res)
}

pub async fn subscribe(mut req: tide::Request<()>) -> tide::Result {
    match req.body_form::<SubscribeFormData>().await {
        Ok(form) => {
            let mut pg_conn = req.sqlx_conn::<Postgres>().await;
            match sqlx::query(
                r#"
                INSERT into subscriptions (id, name, email, subscribed_at)
                VALUES ($1, $2, $3, $4)
                "#
            )
            .bind(Uuid::new_v4())
            .bind(&form.name)
            .bind(&form.email)
            .bind(Utc::now())
            .execute(pg_conn.acquire().await?)
            .await {
                Ok(_) => Ok(Response::new(StatusCode::Created)),
                Err(_) => Ok(Response::new(StatusCode::ExpectationFailed))
            }

        }
        Err(_) => Ok(Response::new(StatusCode::BadRequest)),
    }
}
