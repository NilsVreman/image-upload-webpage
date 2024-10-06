use server;

#[tokio::main]
async fn main() {
    // Build our application
    let app = server::create_app();

    // Run our app
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
