use chrono::{DateTime, Utc};
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
#[derive(Debug)]
pub struct NewItem {
    pub title: String, // Cow<'static, str>,
    pub status: String, //Cow<'static, str>,
    pub date:DateTime<Utc>
}

impl NewItem {
    pub fn new(title: String) -> NewItem {
        let now = Utc::now();
        NewItem{
            title,
            status: String::from("PENDING"),
            date: now
        }
    }    
}