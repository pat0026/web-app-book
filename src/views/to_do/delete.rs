use diesel;
use diesel::prelude::*;

use actix_web::{web, HttpResponse};
use crate::database::establish_connection;
use crate::schema::to_do;

use crate::json_serialization::{to_do_item::ToDoItem, to_do_items::ToDoItems};
use crate::jwt::JWToken;
use crate::models::items::item::Item;

pub async fn delete(to_do_item: web::Json<ToDoItem>, token: JWToken) -> HttpResponse {
    let connection = establish_connection();

    let item = to_do::table
        .filter(to_do::columns::title.eq(&to_do_item.title.as_str()))
        .order(to_do::columns::id.asc())
        .load::<Item>(&connection)
        .unwrap();

    diesel::delete(&item[0]).execute(&connection);
    HttpResponse::Ok().json(ToDoItems::get_state())
}
