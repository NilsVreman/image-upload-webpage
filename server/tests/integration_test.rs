use server;
use tokio; // Import from the common module
use tower::ServiceExt;

#[derive(serde::Deserialize, serde::Serialize)]
struct HealthResponse {
    health: String,
}

#[tokio::test]
async fn test_health_check() {
    // Initialize
    let app = server::create_app();

    // Act
    let response = app
        .oneshot(
            hyper::Request::builder()
                .uri("/health")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    assert_eq!(response.status(), hyper::StatusCode::OK);
    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    assert_eq!(&body_bytes[..], b"{\"health\":\"OK\"}");
}
