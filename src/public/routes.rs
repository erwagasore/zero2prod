use tide::prelude::*;
use tide::{Response, StatusCode};

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
        Ok(_) => Ok(Response::new(StatusCode::Ok)),
        Err(_) => Ok(Response::new(StatusCode::BadRequest)),
    }
}
