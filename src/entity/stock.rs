use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "stocks")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment)]
    pub id: i32,
    #[sea_orm(unique)]
    pub code: i32,
    pub name: String,
    pub url: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::amount_alert::Entity")]
    AmountAlert,
}

impl Related<super::amount_alert::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AmountAlert.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
