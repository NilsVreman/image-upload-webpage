use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use serde_json::Value;
use std::env;
use tempfile::TempDir;
use tower::Service;

use server::create_app;

// A helper function to set up environment variables and change the working directory.
fn setup_test_env() -> TempDir {
    let tmp_dir = tempfile::tempdir().expect("failed to create temp dir");
    env::set_current_dir(&tmp_dir).expect("failed to set current dir");

    env::set_var("JWT_SECRET", "testsecret");
    env::set_var("JWT_EXPIRATION_TIME", "3600"); // expiration in seconds

    // hash the password "password123" with a low cost for testing.
    let hashed = bcrypt::hash("password123", 4).expect("failed to hash password");
    env::set_var("SHARED_PASSWORD_HASH", hashed);

    env::set_var("HOST", "http://localhost");
    env::set_var("PORT", "3000");
    env::set_var("CLIENT_URL", "http://localhost:3000");

    tmp_dir
}

#[tokio::test]
async fn test_health_endpoint() {
    // Build the app.
    let _tmp_dir = setup_test_env();
    let app = create_app().await.expect("failed to create app");

    // Create a GET request to /health.
    let request = Request::builder()
        .method("GET")
        .uri("/health")
        .body(Body::empty())
        .unwrap();
    let response = app.clone().call(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Read and parse the response body.
    let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body_bytes).expect("failed to parse JSON");

    assert_eq!(json.get("health").unwrap(), "OK");
}

#[tokio::test]
async fn test_login_success_and_check_session() {
    // Build the app.
    let _tmp_dir = setup_test_env();
    let mut app = create_app().await.expect("failed to create app");

    // Prepare a valid login JSON payload.
    let login_body = r#"{"password": "password123"}"#;

    let login_request = Request::builder()
        .method("POST")
        .uri("/login")
        .header("content-type", "application/json")
        .body(Body::from(login_body))
        .unwrap();

    let login_response = app.clone().call(login_request).await.unwrap();

    // Expect success and a Set-Cookie header.
    assert_eq!(login_response.status(), StatusCode::OK);
    let set_cookie = login_response
        .headers()
        .get("set-cookie")
        .expect("expected Set-Cookie header")
        .to_str()
        .unwrap()
        .to_string();

    // simulate a session check by sending the cookie.
    let check_session_request = Request::builder()
        .method("GET")
        .uri("/check-session")
        .header("cookie", set_cookie)
        .body(Body::empty())
        .unwrap();

    let check_response = app.call(check_session_request).await.unwrap();

    // Check that the session is valid.
    assert_eq!(check_response.status(), StatusCode::OK);

    let body_bytes = to_bytes(check_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body_bytes).expect("failed to parse JSON");

    assert_eq!(json.get("valid").unwrap(), true);
}

#[tokio::test]
async fn test_login_failure() {
    // Build the app.
    let _tmp_dir = setup_test_env();
    let mut app = create_app().await.expect("failed to create app");

    // Prepare a login payload with an incorrect password.
    let login_body = r#"{"username": "testuser", "password": "wrongpassword"}"#;

    let login_request = Request::builder()
        .method("POST")
        .uri("/login")
        .header("content-type", "application/json")
        .body(Body::from(login_body))
        .unwrap();

    let response = app.call(login_request).await.unwrap();

    // Expect unauthorized.
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_check_session_without_cookie() {
    // Build the app.
    let _tmp_dir = setup_test_env();
    let mut app = create_app().await.expect("failed to create app");

    // Create a GET request to /check-session without any cookie.
    let request = Request::builder()
        .method("GET")
        .uri("/check-session")
        .body(Body::empty())
        .unwrap();

    let response = app.call(request).await.unwrap();

    // The middleware should respond with Unauthorized when no token is provided.
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
