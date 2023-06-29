use sea_orm::*;

use crate::entity::{amount_alert, stock};

pub async fn connect_db(
    file_name: String,
) -> Result<DatabaseConnection, Box<dyn std::error::Error>> {
    let url = format!("sqlite://{}?mode=rwc", file_name);
    let connection = Database::connect(url).await?;
    let backend = connection.get_database_backend();
    let schema = Schema::new(backend);

    let stock_statement = backend.build(
        schema
            .create_table_from_entity(stock::Entity)
            .if_not_exists(),
    );
    connection.execute(stock_statement).await?;
    let amount_alert_statement = backend.build(
        schema
            .create_table_from_entity(amount_alert::Entity)
            .if_not_exists(),
    );
    connection.execute(amount_alert_statement).await?;

    Ok(connection)
}
