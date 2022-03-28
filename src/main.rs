use zero2prod::configuration::read_configs;
use zero2prod::create_app;

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();
    let app = create_app().await;
    let configs = read_configs().expect("Failed to read configurations");
    let address = format!("0.0.0.0:{}", &configs.application_port);
    app.listen(address).await?;
    Ok(())
}
