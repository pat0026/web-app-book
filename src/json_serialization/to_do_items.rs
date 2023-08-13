use serde::{Serialize, Deserialize};
use std::vec::Vec;

use serde_json::value::Value;
use serde_json::Map;

use actix_web::{
    body::BoxBody, http::header::ContentType,
    HttpRequest, HttpResponse, Responder
};

use crate::to_do::structs::base::Base;
use crate::to_do::ItemTypes;
use crate::state::read_file;
use crate::to_do::{to_do_factory, enums::TaskStatus};

use surrealdb::sql;
use std::time::Instant;

use crate::database::CLIENT;

#[derive(Serialize)]
pub struct ToDoItems {
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_item_count: i8,
    pub done_item_count: i8,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    date: sql::Datetime,
    id: sql::Thing,
    status: sql::Strand,
    title: sql::Strand
}

impl ToDoItems {
    pub fn new(input_items: Vec<ItemTypes>) -> ToDoItems {
        let mut pending_array_buffer = Vec::new();
        let mut done_array_buffer = Vec::new();

        for item in input_items {
            match item {
                ItemTypes::Pending(packed) => pending_array_buffer.push(packed.super_struct),
                ItemTypes::Done(packed) => done_array_buffer.push(packed.super_struct),
            }
        }

        let done_count: i8 = done_array_buffer.len() as i8;
        let pending_count: i8 = pending_array_buffer.len() as i8;

        ToDoItems {
            pending_items: pending_array_buffer,
            done_items: done_array_buffer,
            pending_item_count: pending_count,
            done_item_count: done_count,
        }
    }

    pub async fn get_state() -> ToDoItems {
        let now = Instant::now();
        let hehe: Vec<Record> = CLIENT.select("to_do").await.expect("hehe");
        println!("{:?}", hehe);
        println!("{}", now.elapsed().as_secs_f64());
        println!("Connection made");
        let state: Map<String, Value> = read_file("./state.json");
        let mut array_buffer = Vec::new();

        for (key, value) in state {
            let status = TaskStatus::from_string(value
                .as_str().unwrap().to_string());
            let item = to_do_factory(&key, status);
            array_buffer.push(item);
        }
        return ToDoItems::new(array_buffer);
    }
}

impl Responder for ToDoItems {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
