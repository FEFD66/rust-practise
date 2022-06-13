use std::net::SocketAddr;

use axum::{Router, routing::get, response::Html};

mod filemanager;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    
    let app = Router::new()
        .route("/", get(root));

    let addr = SocketAddr::from(([127,0,0,1],3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root()->Html<String>{
    Html::from("Hello".to_string())
}

