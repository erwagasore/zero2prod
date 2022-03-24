mod routes;

use routes::{health_check, subscribe};

use tide::Server;

pub fn create_public_app() -> Server<()> {
    let mut public = tide::new();
    public.at("health_check").get(health_check);
    public.at("subscribe").post(subscribe);
    public
}
