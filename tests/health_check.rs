use tide::StatusCode;
use tide_testing::TideTestingExt;

use zero2prod::create_app;

#[async_std::test]
async fn health_check_route_test() -> tide::Result<()>{
    let app = create_app().await;
    assert_eq!(app.get("/health_check").await?.status(), StatusCode::Ok);
    Ok(())
}
