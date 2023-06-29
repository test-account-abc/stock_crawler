use axum::{
    routing::{get, post},
    Router, Server,
};
use database::connect_db;
use request::get_request;
use router::{
    amount_alert_router::{list_amount_alerts, post_amount_alert},
    stock_router::{delete_stock, get_stock, list_stock, post_stock, post_stock_crawling},
};
use scraping::get_stock_value;
use sea_orm::DatabaseConnection;
use std::{error::Error, net::SocketAddr, sync::Arc};

mod database;
mod entity;
mod request;
mod router;
mod scraping;
mod service;

#[derive(Clone)]
pub struct AppState {
    connection: DatabaseConnection,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let connection = connect_db("sample.db".to_string()).await?;
    let state = Arc::new(AppState {
        connection: connection.clone(),
    });

    let app = create_router(state);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

fn create_router(state: Arc<AppState>) -> Router {
    return Router::new()
        .route("/stocks", get(list_stock).post(post_stock))
        .route("/stocks/:id", get(get_stock).delete(delete_stock))
        .route("/stocks/:id/crawling", post(post_stock_crawling))
        .route(
            "/stocks/:id/amount_alerts",
            get(list_amount_alerts).post(post_amount_alert),
        )
        .with_state(state);
}
