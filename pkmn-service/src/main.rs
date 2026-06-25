use pkmn_service::Cards;
use proto_packet::axum::{Router, serve};
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let router: Router = pkmn_schema::api::v0::cards::router(Arc::new(Cards));
    let listener: TcpListener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    serve(listener, router).await.unwrap();
}
