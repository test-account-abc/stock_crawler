use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "amount_alerts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment)]
    pub id: i32,
    pub stock_code: i32,
    pub mode: String,
    pub amount: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::stock::Entity",
        from = "Column::StockCode",
        to = "super::stock::Column::Code"
    )]
    Stock,
}

impl Related<super::stock::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Stock.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
