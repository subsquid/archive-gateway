use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};


#[derive(SimpleObject, Debug)]
pub struct BlockHeader {
    pub id: String,
    pub height: i32,
    pub hash: String,
    pub parent_hash: String,
    pub timestamp: DateTime<Utc>,
}


#[derive(sqlx::FromRow, SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct Extrinsic {
    pub id: String,
    pub block_id: String,
    pub index_in_block: i32,
    pub name: String,
    pub signature: Option<serde_json::Value>,
    pub success: bool,
    pub hash: String,
}


#[derive(sqlx::FromRow, SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct Call {
    pub id: String,
    pub index: i32,
    pub extrinsic_id: String,
    pub parent_id: Option<String>,
    pub success: bool,
    pub name: String,
    pub args: Option<serde_json::Value>,
    #[graphql(skip)]
    pub block_id: String,
}


#[derive(sqlx::FromRow, SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub block_id: String,
    pub index_in_block: i32,
    pub phase: String,
    pub extrinsic_id: Option<String>,
    pub call_id: Option<String>,
    pub name: String,
    pub args: Option<serde_json::Value>
}


#[derive(Debug)]
pub struct Block {
    pub header: BlockHeader,
}


#[derive(sqlx::FromRow, SimpleObject, Debug)]
pub struct Metadata {
    spec_version: i32,
    block_height: i32,
    block_hash: String,
    hex: String,
}


#[derive(sqlx::FromRow, SimpleObject, Debug)]
pub struct Status {
    pub head: i32,
}
