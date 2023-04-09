mod processes;
mod state;
mod to_do;
mod views;

use std::thread::JoinHandle;
use std::{env, thread, time, result};

use serde_json::value::Value;
use serde_json::Map;

use processes::process_input;
use state::read_file;
use to_do::enums::TaskStatus;
use to_do::to_do_factory;

use actix_web::{web, App, HttpRequest, HttpServer, Responder};

use futures::executor::block_on;
use futures::join;

use std::vec::Vec;
use async_std;
use futures::future::{join_all, try_join};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new()
            .configure(views::views_factory);
        app
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
} 