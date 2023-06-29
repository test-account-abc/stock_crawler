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

use crate::{entity::amount_alert, service::amount_alert_service::AmountAlertService, AppState};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostAmountAlertBody {
    mode: String,
    amount: i32,
}

#[derive(Debug, Serialize)]
pub struct AmountAlertResponse {
    id: i32,
    mode: String,
    amount: i32,
}

fn convert_to_amount_alert_response(
    amount_alert: amount_alert::ActiveModel,
) -> AmountAlertResponse {
    AmountAlertResponse {
        id: amount_alert.id.unwrap(),
        mode: amount_alert.mode.unwrap(),
        amount: amount_alert.amount.unwrap(),
    }
}

pub async fn post_amount_alert(
    Path(id): Path<i32>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<PostAmountAlertBody>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let amount_alert =
        AmountAlertService::create(state.connection.clone(), body.mode, body.amount, id).await;
    match amount_alert {
        Ok(amount_alert) => {
            let amount_alert = convert_to_amount_alert_response(amount_alert);
            let amount_alert_response = json!({ "amount_alert": amount_alert });
            return Ok(Json(amount_alert_response));
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

pub async fn list_amount_alerts(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let amount_alerts = AmountAlertService::list(state.connection.clone()).await;
    match amount_alerts {
        Ok(amount_alerts) => {
            let amount_alerts: Vec<_> = amount_alerts
                .iter()
                .map(|amount_alert| convert_to_amount_alert_response(amount_alert.clone()))
                .collect();
            let amount_alerts_response = json!({ "amount_alerts": amount_alerts });
            return Ok(Json(amount_alerts_response));
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
