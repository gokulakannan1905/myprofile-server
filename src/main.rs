use std::net::{SocketAddr, Ipv4Addr};

use axum::{routing::get, Router, response::{IntoResponse, Html}};

#[tokio::main]
async fn main() {    
    //router
    let app = Router::new()
        .route("/", get(index))
        .route("/welcome",get(root));

    let ip = Ipv4Addr::new(0,0,0,0);
    let port = 80;
    let addr = SocketAddr::new(ip.into(),port);

    println!("Server is listening on all interfaces at PORT {port}");
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

async fn index() -> impl IntoResponse{
    let markup = tokio::fs::read_to_string("src/index.html").await.unwrap();
    Html(markup)
}

async fn root() -> &'static str {
    "Welcome to Rust based webpage!"
}
