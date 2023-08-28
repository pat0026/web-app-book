use chrono::{DateTime,Utc};
// use std::borrow::Cow;
use serde::{Serialize, Deserialize};
// use uuid::Uuid;
use surrealdb::sql::Thing;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Item {
    pub id: Thing, //Cow<'static, str>,
    pub title: String, // Cow<'static, str>,
    pub status: String, //Cow<'static, str>,
    pub date:DateTime<Utc>
}