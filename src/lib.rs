use tide::{StatusCode, Server};

pub async fn create_app() -> Server<()> {
    let mut app = Server::new();
    app.at("/health_check").get(|_| async { Ok(StatusCode::Ok) });
    app
}
