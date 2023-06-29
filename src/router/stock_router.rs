use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::error;

use crate::{
    entity::{amount_alert, stock},
    request::get_request,
    scraping::get_stock_value,
    service::{amount_alert_service::AmountAlertService, stock_service::StockService},
    AppState,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostStockBody {
    code: i32,
    name: String,
    url: String,
}

#[derive(Debug, Serialize)]
pub struct StockResponse {
    id: i32,
    code: i32,
    name: String,
    url: String,
}

#[derive(Debug, Serialize)]
pub struct CrawlingResponse {
    name: String,
    alert_amount: i32,
    current_amount: i32,
}

fn convert_to_crawling_response(
    alert: amount_alert::ActiveModel,
    name: String,
    current_amount: i32,
) -> CrawlingResponse {
    CrawlingResponse {
        name,
        alert_amount: alert.amount.unwrap(),
        current_amount,
    }
}

fn convert_to_stock_response(stock: stock::ActiveModel) -> StockResponse {
    StockResponse {
        code: stock.code.unwrap(),
        id: stock.id.unwrap(),
        name: stock.name.unwrap(),
        url: stock.url.unwrap(),
    }
}

pub async fn post_stock(
    State(state): State<Arc<AppState>>,
    Json(body): Json<PostStockBody>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let stock =
        StockService::create(state.connection.clone(), body.code, body.name, body.url).await;
    match stock {
        Ok(stock) => {
            let stock = convert_to_stock_response(stock);
            let stock_response = json!({ "stock": stock });
            return Ok(Json(stock_response));
        }
        Err(err) => {
            error!("Unexpecte error: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unexpected error".to_string(),
            ));
        }
    }
}

pub async fn list_stock(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let stocks = StockService::list(state.connection.clone()).await;
    match stocks {
        Ok(stocks) => {
            let stocks: Vec<_> = stocks
                .iter()
                .map(|stock| convert_to_stock_response(stock.clone()))
                .collect();
            let stocks_response = json!({ "stocks": stocks });
            return Ok(Json(stocks_response));
        }
        Err(err) => {
            error!("Unexpecte error: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unexpected error".to_string(),
            ));
        }
    }
}

pub async fn get_stock(
    Path(id): Path<i32>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let stock = StockService::get_by_id(state.connection.clone(), id).await;
    match stock {
        Ok(stock) => {
            if stock.is_none() {
                return Ok(Json(json!({ "stock": null })));
            }
            let stock_response = json!({ "stock": convert_to_stock_response(stock.unwrap()) });
            return Ok(Json(stock_response));
        }
        Err(err) => {
            error!("Unexpected error: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unexpected error".to_string(),
            ));
        }
    }
}

pub async fn post_stock_crawling(
    Path(id): Path<i32>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let stock = match StockService::get_by_id(state.connection.clone(), id).await {
        Ok(value) => value,
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unexpected error".to_string(),
            ))
        }
    };
    let stock = match stock {
        Some(value) => value,
        None => return Err((StatusCode::NOT_FOUND, "Stock not found".to_string())),
    };
    let alerts = match AmountAlertService::query_by_code(
        state.connection.clone(),
        stock.code.unwrap(),
    )
    .await
    {
        Ok(value) => value,
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unexpected error".to_string(),
            ))
        }
    };
    let url = match stock.url.into_value() {
        Some(value) => value.to_string(),
        None => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unexpected error".to_string(),
            ))
        }
    };
    let html = match get_request(url).await {
        Ok(value) => value,
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unexpected error".to_string(),
            ))
        }
    };
    let current_amount = match get_stock_value(html) {
        Ok(value) => value,
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unexpected error".to_string(),
            ))
        }
    };
    let alerts = alerts.into_iter().filter(|alert| {
        is_alert(
            current_amount,
            alert.clone().amount.unwrap(),
            alert.clone().mode.unwrap(),
        )
    });
    let name = stock.name.unwrap();
    let crawling_responses: Vec<_> = alerts
        .map(|alert| convert_to_crawling_response(alert, name.clone(), current_amount))
        .collect();

    let response = json!({ "crawling_responses": crawling_responses });
    return Ok(Json(response));
}

fn is_alert(current_amount: i32, alert_amount: i32, mode: String) -> bool {
    let up = match mode.as_str() {
        "up" => true,
        _ => false,
    };
    if up {
        return current_amount > alert_amount;
    } else {
        return current_amount < alert_amount;
    }
}

pub async fn delete_stock(
    Path(id): Path<i32>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let result = StockService::delete(state.connection.clone(), id).await;
    match result {
        Ok(()) => return Ok(()),
        Err(err) => {
            error!("Unexpecte error: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unexpected error".to_string(),
            ));
        }
    }
}
