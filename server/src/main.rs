use std::{
    net::{Ipv4Addr, SocketAddr, TcpListener},
    sync::Arc,
};

use axum_server::tls_rustls::RustlsConfig;
use server;

fn main() {
    if let Err(e) = run_server() {
        eprintln!("FAILED: {}", e);
        std::process::exit(1);
    }
}

#[tokio::main]
async fn run_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Setting up server...");

    // Build our application
    let app = server::create_app().await?;

    // Config auth
    let tls_config = server::load_tls_config()?;
    let axum_config = RustlsConfig::from_config(Arc::new(tls_config));

    let port = 3000;
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), port);

    // Run our app
    println!("Starting to serve on https://{}...", addr);

    let listener = TcpListener::bind(addr)?;
    axum_server::from_tcp_rustls(listener, axum_config)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
