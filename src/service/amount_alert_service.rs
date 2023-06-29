use sea_orm::*;
use std::error::Error;

use crate::entity::{amount_alert, stock};

pub struct AmountAlertService {}

impl AmountAlertService {
    pub async fn get_by_id(
        connection: DatabaseConnection,
        id: i32,
    ) -> Result<Option<amount_alert::ActiveModel>, Box<dyn Error>> {
        let amount_alert = amount_alert::Entity::find_by_id(id)
            .one(&connection)
            .await?;
        if amount_alert.is_none() {
            return Ok(None);
        }
        Ok(Some(amount_alert.unwrap().into()))
    }

    pub async fn list(
        connection: DatabaseConnection,
    ) -> Result<Vec<amount_alert::ActiveModel>, Box<dyn Error>> {
        let amount_alerts = amount_alert::Entity::find().all(&connection).await?;
        let amount_alerts = amount_alerts
            .iter()
            .map(|amount_alert| {
                let amount_alert: amount_alert::ActiveModel = amount_alert.clone().into();
                return amount_alert;
            })
            .collect();
        Ok(amount_alerts)
    }

    pub async fn query_by_code(
        connection: DatabaseConnection,
        code: i32,
    ) -> Result<Vec<amount_alert::ActiveModel>, Box<dyn Error>> {
        let amount_alerts = amount_alert::Entity::find()
            .filter(amount_alert::Column::StockCode.eq(code))
            .all(&connection)
            .await?;
        let amount_alerts = amount_alerts
            .iter()
            .map(|amount_alert| {
                let amount_alert: amount_alert::ActiveModel = amount_alert.clone().into();
                return amount_alert;
            })
            .collect();
        Ok(amount_alerts)
    }

    pub async fn create(
        connection: DatabaseConnection,
        mode: String,
        amount: i32,
        stock_id: i32,
    ) -> Result<amount_alert::ActiveModel, Box<dyn Error>> {
        let stock = stock::Entity::find_by_id(stock_id).one(&connection).await?;
        if stock.is_none() {
            return Err("Stock not found".to_owned().into());
        }
        let stock: stock::ActiveModel = stock.unwrap().into();
        let model = amount_alert::ActiveModel {
            mode: Set(mode),
            amount: Set(amount),
            stock_code: Set(stock.code.unwrap()),
            ..Default::default()
        };
        let saved_amount_alert = model.save(&connection).await?;
        Ok(saved_amount_alert)
    }

    pub async fn update(
        connection: DatabaseConnection,
        id: i32,
        mode: String,
        amount: i32,
    ) -> Result<amount_alert::ActiveModel, Box<dyn Error>> {
        let model = amount_alert::Entity::find_by_id(id)
            .one(&connection)
            .await?;
        if model.is_none() {
            return Err("AmountAlert not found".to_owned().into());
        }
        let mut model: amount_alert::ActiveModel = model.unwrap().into();
        model.amount = Set(amount);
        model.mode = Set(mode);
        let saved_amount_alert = model.save(&connection).await?;
        Ok(saved_amount_alert)
    }

    pub async fn delete(connection: DatabaseConnection, id: i32) -> Result<(), Box<dyn Error>> {
        let model = amount_alert::Entity::find_by_id(id)
            .one(&connection)
            .await?;
        if model.is_none() {
            return Err("AmountAlert not found".to_owned().into());
        }
        model.clone().unwrap().delete(&connection).await?;
        Ok(())
    }
}
