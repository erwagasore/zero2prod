use zero2prod::create_app;

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();
    let app = create_app().await;
    app.listen("0.0.0.0:6000").await?;
    Ok(())
}
