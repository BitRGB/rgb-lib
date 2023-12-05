//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "txo"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Hash)]
pub struct Model {
    pub idx: i32,
    pub txid: String,
    pub vout: u32,
    pub btc_amount: String,
    pub spent: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Idx,
    Txid,
    Vout,
    BtcAmount,
    Spent,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Idx,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i32;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Coloring,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Idx => ColumnType::Integer.def(),
            Self::Txid => ColumnType::String(None).def(),
            Self::Vout => ColumnType::Integer.def(),
            Self::BtcAmount => ColumnType::String(None).def(),
            Self::Spent => ColumnType::Boolean.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Coloring => Entity::has_many(super::coloring::Entity).into(),
        }
    }
}

impl Related<super::coloring::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Coloring.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
