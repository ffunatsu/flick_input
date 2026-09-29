mod input;

use axum::{
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Json, Router,
};
use clap::Parser;
use qrcode::render::unicode::Dense1x2;
use qrcode::QrCode;
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Parser, Debug)]
#[command(version, about = "smartphone as a wireless keyboard for PC")]
struct Args {
    /// Port to listen on
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
}

#[derive(Deserialize)]
struct InputPayload {
    text: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let port = args.port;
    let local_ip = local_ip_address::local_ip().unwrap_or(std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)));
    let url = format!("http://{}:{}", local_ip, port);

    println!("\n=== Flick Input Server ===");
    println!("URL: {}\n", url);

    if let Ok(code) = QrCode::new(url.as_bytes()) {
        let image = code.render::<Dense1x2>().quiet_zone(true).build();
        println!("{}", image);
    }

    println!("Scan the QR code or open the URL on your mobile phone.\n");

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/api/input", post(input_handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind port");

    axum::serve(listener, app).await.expect("Server error");
}

async fn index_handler() -> Html<&'static str> {
    Html(include_str!("index.html"))
}

async fn input_handler(Json(payload): Json<InputPayload>) -> StatusCode {
    input::type_text(&payload.text);
    StatusCode::OK
}
