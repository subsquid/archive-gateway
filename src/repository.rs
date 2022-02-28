use crate::entities::{Block, BlockHeader, Extrinsic, Call, Event, Metadata, Status};
use std::collections::HashMap;
use sqlx::{postgres::PgRow, Error, Pool, Postgres, Row};
use async_graphql::InputObject;


#[derive(InputObject, Clone)]
pub struct ExtrinsicFields {
    #[graphql(name="_all")]
    _all: Option<bool>,
    id: Option<bool>,
    block_id: Option<bool>,
    index_in_block: Option<bool>,
    name: Option<bool>,
    signature: Option<bool>,
    success: Option<bool>,
    hash: Option<bool>,
}


#[derive(InputObject, Clone)]
pub struct CallFields {
    #[graphql(name="_all")]
    _all: Option<bool>,
    id: Option<bool>,
    extrinsic: Option<ExtrinsicFields>,
    parent: Option<bool>,
    success: Option<bool>,
    name: Option<bool>,
    args: Option<bool>,
}


#[derive(InputObject)]
pub struct EventFields {
    #[graphql(name="_all")]
    _all: Option<bool>,
    id: Option<bool>,
    block_id: Option<bool>,
    index_in_block: Option<bool>,
    phase: Option<bool>,
    extrinsic: Option<ExtrinsicFields>,
    call_id: Option<bool>,
    name: Option<bool>,
    args: Option<bool>,
}


#[derive(InputObject)]
pub struct EventSelection {
    name: String,
    fields: EventFields,
}


#[derive(InputObject, Clone)]
pub struct CallSelection {
    name: String,
    fields: CallFields,
}


pub async fn get_blocks(
    pool: &Pool<Postgres>,
    limit: i32,
    from_block: i32,
    to_block: Option<i32>,
    events: Option<Vec<EventSelection>>,
    calls: &Option<Vec<CallSelection>>,
    include_all_blocks: Option<bool>
) -> Result<Vec<Block>, Error> {
    let mut events_name: Option<Vec<String>> = None;
    let mut calls_name: Option<Vec<String>> = None;
    if let Some(events) = events {
        events_name = Some(events.iter().map(|selection| selection.name.clone()).collect());
    }
    if let Some(calls) = calls {
        calls_name = Some(calls.iter().map(|selection| selection.name.clone()).collect());
    }
    let query = "SELECT
            id,
            height,
            hash,
            parent_hash,
            timestamp
        FROM block
        WHERE height >= $1
            AND ($2 IS null OR height < $2)
            AND ($3 IS true OR (
                EXISTS (SELECT 1 FROM event WHERE event.block_id = block.id AND event.name = ANY($4))
                OR EXISTS (
                    SELECT 1
                    FROM call INNER JOIN extrinsic ON call.extrinsic_id = extrinsic.id
                    WHERE extrinsic.block_id = block.id AND call.name = ANY($5)
                )
            ))
        ORDER BY height
        LIMIT $6";
    let blocks = sqlx::query(query)
        .bind(from_block)
        .bind(to_block)
        .bind(include_all_blocks)
        .bind(events_name)
        .bind(calls_name)
        .bind(limit)
        .map(|row: PgRow| Block {
            header: BlockHeader {
                id: row.get_unchecked("id"),
                height: row.get_unchecked("height"),
                hash: row.get_unchecked("hash"),
                parent_hash: row.get_unchecked("parent_hash"),
                timestamp: row.get_unchecked("timestamp"),
            },
        })
        .fetch_all(pool)
        .await?;
    Ok(blocks)
}


pub async fn get_extrinsics(pool: &Pool<Postgres>, blocks: &[String]) -> Result<Vec<Extrinsic>, Error> {
    let query = "SELECT id, block_id, index_in_block, name, signature, success, hash FROM extrinsic WHERE block_id = ANY($1::char(16)[])";
    let extrinsics = sqlx::query_as::<_, Extrinsic>(query)
        .bind(blocks)
        .fetch_all(pool)
        .await?;
    Ok(extrinsics)
}


pub async fn get_calls(pool: &Pool<Postgres>, blocks: &[String], selections: &[CallSelection]) -> Result<Vec<Call>, Error> {
    let calls_name: Vec<String> = selections.iter()
        .map(|selection| selection.name.clone())
        .collect();
    let query = "WITH RECURSIVE child_call AS (
            SELECT
                call.id,
                call.index,
                call.extrinsic_id,
                call.parent_id,
                call.success,
                call.name,
                call.args,
                extrinsic.block_id
            FROM call
            INNER JOIN extrinsic ON call.extrinsic_id = extrinsic.id
            WHERE extrinsic.block_id = ANY($1::char(16)[]) AND call.name = ANY($2)
        UNION
            SELECT
                call.id,
                call.index,
                call.extrinsic_id,
                call.parent_id,
                call.success,
                call.name,
                call.args,
                child_call.block_id
            FROM call INNER JOIN child_call ON child_call.parent_id = call.id
        ) SELECT * FROM child_call";
    let calls: HashMap<String, Call> = sqlx::query_as::<_, Call>(query)
        .bind(&blocks)
        .bind(&calls_name)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|call| {
            (call.id.clone(), call)
        })
        .collect();

    let mut recursive_calls = Vec::new();
    for call in calls.values() {
        if calls_name.contains(&call.name) {
            let mut recursive_call = call.clone();
            let mut stack: Vec<Call> = Vec::new();
            let mut current_call = &recursive_call;
            while current_call.parent_id.is_some() {
                current_call = calls.get(current_call.parent_id.as_ref().unwrap()).unwrap();
                stack.push(current_call.clone());
            }
            let mut parent: Option<serde_json::Value> = None;
            while let Some(mut top) = stack.pop() {
                top.parent = parent;
                parent = Some(serde_json::to_value(&top).unwrap());
            }
            recursive_call.parent = parent;
            recursive_calls.push(recursive_call);
        }
    }

    Ok(recursive_calls)
}


pub async fn get_events(pool: &Pool<Postgres>, blocks: &[String]) -> Result<Vec<Event>, Error> {
    let query = "SELECT id, block_id, index_in_block, phase, extrinsic_id, call_id, name, args FROM event WHERE block_id = ANY($1::char(16)[])";
    let events = sqlx::query_as::<_, Event>(query)
        .bind(blocks)
        .fetch_all(pool)
        .await?;
    Ok(events)
}


pub async fn get_metadata(pool: &Pool<Postgres>) -> Result<Vec<Metadata>, Error> {
    let query = "SELECT spec_version, block_height, block_hash, hex FROM metadata";
    let metadata = sqlx::query_as::<_, Metadata>(query)
        .fetch_all(pool)
        .await?;
    Ok(metadata)
}


pub async fn get_status(pool: &Pool<Postgres>) -> Result<Status, Error> {
    let query = "SELECT height as head FROM block ORDER BY height DESC LIMIT 1";
    let status = sqlx::query_as::<_, Status>(query)
        .fetch_one(pool)
        .await?;
    Ok(status)
}
