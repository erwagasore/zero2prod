pub use tide::StatusCode;
use tide_testing::TideTestingExt;

use zero2prod::create_app;

#[async_std::test]
async fn health_check_route_tests() -> tide::Result<()> {
    let app = create_app().await;
    assert_eq!(app.get("/health_check").await?.status(), StatusCode::Ok);
    Ok(())
}

#[async_std::test]
async fn subscribe_route_tests() -> tide::Result<()> {
    let app = create_app().await;
    let body = "name=John%20Doe&email=john_doe%40zero2prod.com";
    let res = app
        .post("/subscribe")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .await?;
    assert_eq!(res.status(), StatusCode::Ok);

    let invalid_case = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    for (body, message) in invalid_case {
        let res = app
            .post("/subscribe")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .await?;
        assert_eq!(
            res.status(),
            StatusCode::BadRequest,
            "The API failed with 400 Bad Request because payload was {}",
            message
        );
    }

    Ok(())
}
