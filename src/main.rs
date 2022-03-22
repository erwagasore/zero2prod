use tide::{Response, StatusCode};

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/health_check").get(|_| async { Ok(Response::new(StatusCode::Ok)) });
    app.listen("0.0.0.0:6000").await?;
    Ok(())
}
