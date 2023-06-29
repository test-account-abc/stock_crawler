use sea_orm::*;
use std::error::Error;

use crate::entity::stock;

pub struct StockService {}

impl StockService {
    pub async fn create(
        connection: DatabaseConnection,
        code: i32,
        name: String,
        url: String,
    ) -> Result<stock::ActiveModel, Box<dyn Error>> {
        let stock = stock::ActiveModel {
            code: Set(code),
            name: Set(name),
            url: Set(url),
            ..Default::default()
        };
        let saved_stock = stock.save(&connection).await?;
        Ok(saved_stock)
    }

    pub async fn get_by_id(
        connection: DatabaseConnection,
        id: i32,
    ) -> Result<Option<stock::ActiveModel>, Box<dyn Error>> {
        let stock = stock::Entity::find_by_id(id).one(&connection).await?;
        if stock.is_none() {
            return Ok(None);
        }
        Ok(Some(stock.unwrap().into()))
    }

    pub async fn list(
        connection: DatabaseConnection,
    ) -> Result<Vec<stock::ActiveModel>, Box<dyn Error>> {
        let stocks = stock::Entity::find().all(&connection).await?;
        let stocks = stocks
            .iter()
            .map(|stock| {
                let stock: stock::ActiveModel = stock.clone().into();
                return stock;
            })
            .collect();
        Ok(stocks)
    }

    pub async fn delete(connection: DatabaseConnection, id: i32) -> Result<(), Box<dyn Error>> {
        let model = stock::Entity::find_by_id(id).one(&connection).await?;
        model.clone().unwrap().delete(&connection).await?;
        Ok(())
    }
}
