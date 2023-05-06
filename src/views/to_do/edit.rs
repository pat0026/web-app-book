use crate:: diesel;
use diesel::prelude::*;

use actix_web::{web, HttpResponse};

use crate::json_serialization::{to_do_item::ToDoItem,
                                to_do_items::ToDoItems};
use crate::jwt::JWToken;
use crate::database::DB;
use crate::schema::to_do;

pub async fn edit(to_do_item: web::Json<ToDoItem>, token: JWToken, db:DB) -> HttpResponse {
    let results = to_do::table
        .filter(to_do::columns::title
            .eq(&to_do_item.title))
        .filter(to_do::columns::user_id.eq(&token.user_id));
    
    diesel::update(results)
            .set(to_do::columns::status.eq("DONE"))
            .execute(&db.connection);

    return HttpResponse::Ok().json(ToDoItems::get_state(token.user_id))
}

