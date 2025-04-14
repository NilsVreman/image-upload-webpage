use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;

fn main() {
    if let Err(err) = run_server() {
        eprintln!("Error running server: {}", err);
        std::process::exit(1);
    }
}

#[tokio::main]
async fn run_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Setting up server...");

    // Build our application
    let app = server::create_app().await?;
    let config = server::Config::from_env();
    let port = config.port;
    let addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), port);

    // Run our app
    println!("Starting to serve on https://{}...", addr);
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
