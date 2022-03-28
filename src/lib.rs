pub mod configuration;
mod public;

use configuration::read_configs;
use public::create_public_app;

use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::ConnectOptions; // Or sqlx::prelude::*;
use tide::Server;
use tide_sqlx::SQLxMiddleware;

pub async fn create_app() -> Server<()> {
    let configs = read_configs().expect("Failed to read configurations");
    let mut connect_opts = PgConnectOptions::new()
        .username(&configs.database.username)
        .password(&configs.database.password)
        .host(&configs.database.host)
        .port(configs.database.port);
    connect_opts.log_statements(tide::log::LevelFilter::Debug);
    let pg_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(connect_opts)
        .await
        .expect("Failed to connect to the database");

    let mut app = Server::new();
    app.with(SQLxMiddleware::from(pg_pool));
    app.at("/").nest(create_public_app());
    app
}
