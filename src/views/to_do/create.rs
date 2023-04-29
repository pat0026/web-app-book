use crate::diesel;
use diesel::prelude::*;

use actix_web::{HttpRequest, HttpResponse};

use crate::database::DB;
use crate::models::items::{new_item::NewItem, item::Item};
use crate::schema::to_do;
use crate::json_serialization::to_do_items::ToDoItems;

pub async fn create(req: HttpRequest, db: DB) -> HttpResponse {
    let title: String = req.match_info().get("title").unwrap().to_string();
    
    let items = to_do::table
        .filter(to_do::columns::title.eq(&title.as_str()))
        .order(to_do::columns::id.asc())
        .load::<Item>(&db.connection)
        .unwrap();

    if items.len() == 0 {
        let new_post = NewItem::new(title, 1);
        diesel::insert_into(to_do::table)
            .values(&new_post)
            .execute(&db.connection);
    }

    HttpResponse::Ok().json(ToDoItems::get_state(db))
}