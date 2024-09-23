mod files;

use axum::Router;
use files::routers::create_files_router;

#[tokio::main]
async fn main() {
    // Create the files router
    let files_router = create_files_router();

    // Build our application with the files router
    let app = Router::new().nest("/files", files_router);

    // Run our app
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    println!("Server running at http://0.0.0.0:3000");
}
