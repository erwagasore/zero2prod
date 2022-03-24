mod public;

use public::create_public_app;

use tide::Server;

pub async fn create_app() -> Server<()> {
    let mut app = Server::new();
    app.at("/").nest(create_public_app());
    app
}
