use std::fmt::format;
use std::result;

use actix_web::{web, HttpResponse};
use serde_json::value::Value;
use serde_json::Map;

use crate::json_serialization::{to_do_item::ToDoItem, to_do_items::ToDoItems};
use crate::jwt::JWToken;
use crate::processes::process_input;
use crate::state::read_file;
use crate::to_do::{enums::TaskStatus, to_do_factory};

pub async fn delete(to_do_item: web::Json<ToDoItem>, token: JWToken) -> HttpResponse {
    let state = read_file("./state.json");

    let status: TaskStatus;
    match state.get(&to_do_item.title) {
        Some(result) => {
            status = TaskStatus::from_string(result.as_str().unwrap().to_string());
        }
        None => {
            return HttpResponse::NotFound().json(format!("{} not in state", &to_do_item.title));
        }
    }

    let existing_item = to_do_factory(to_do_item.title.as_str(), status);
    process_input(existing_item, "delete".to_string(), &state);

    HttpResponse::Ok().json(ToDoItems::get_state().await)
}
