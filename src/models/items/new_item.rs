use chrono::{NaiveDateTime, Utc};
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct NewItem {
    pub title: String,
    pub status: String,
    pub date: NaiveDateTime
}

impl NewItem {
    pub fn new(title: String) -> NewItem {
        let now = Utc::now().naive_local();
        NewItem{
            title,
            status: String::from("PENDING"),
            date: now
        }
    }    
}