//! Result types for database methods.

#![allow(proc_macro_derive_resolution_fallback)]

use std::collections::HashMap;

use diesel::sql_types::{BigInt, Integer, Nullable, Text};
use serde::Serialize;

use super::params;
use db::util::SyncTimestamp;

pub type LockCollection = ();
pub type GetBsoTimestamp = SyncTimestamp;
pub type GetCollectionTimestamps = HashMap<String, SyncTimestamp>;
pub type GetCollectionTimestamp = SyncTimestamp;
pub type GetCollectionCounts = HashMap<String, i64>;
pub type GetCollectionUsage = HashMap<String, i64>;
pub type GetStorageTimestamp = SyncTimestamp;
pub type GetStorageUsage = u64;
pub type DeleteStorage = ();
pub type DeleteCollection = SyncTimestamp;
pub type DeleteBsos = SyncTimestamp;
pub type DeleteBso = SyncTimestamp;
pub type PutBso = SyncTimestamp;

pub type CreateBatch = i64;
pub type ValidateBatch = bool;
pub type AppendToBatch = ();
pub type GetBatch = params::Batch;
pub type DeleteBatch = ();
pub type CommitBatch = PostBsos;

#[derive(Debug, Default, Deserialize, Queryable, QueryableByName, Serialize)]
pub struct GetBso {
    #[sql_type = "Text"]
    pub id: String,
    #[sql_type = "BigInt"]
    pub modified: SyncTimestamp,
    #[sql_type = "Text"]
    pub payload: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[sql_type = "Nullable<Integer>"]
    pub sortindex: Option<i32>,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    #[sql_type = "BigInt"]
    pub expiry: i64,
}

#[derive(Debug, Default)]
pub struct Paginated<T>
where
    T: Serialize,
{
    pub items: Vec<T>,
    pub offset: Option<i64>, // XXX: i64?
}

pub type GetBsos = Paginated<GetBso>;
pub type GetBsoIds = Paginated<String>;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PostBsos {
    pub modified: SyncTimestamp,
    pub success: Vec<String>,
    pub failed: HashMap<String, String>,
}
