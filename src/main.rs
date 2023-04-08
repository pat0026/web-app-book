mod processes;
mod state;
mod to_do;

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

async fn do_something(number: i8) -> i8 {
    println!("number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    return 2;
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", name)
}

async fn utils_one() -> impl Responder {
    "Utils one reached\n"
}

async fn health() -> impl Responder {
    "All good\n"
}

// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     let s1 = HttpServer::new(move || {
//         App::new().service(web::scope("/utils").route(
//             "/one", web::get().to(utils_one)))
//     })
//     .bind("0.0.0.0:8081")?
//     .run();
//     let s2 = HttpServer::new( move || {
//         App::new().service(web::resource("/health").route(
//             web::get().to(health)))
//     })
//     .bind("0.0.0.0:8080")?
//     .run();

//     try_join(s1, s2).await?;

//     Ok(())
// }

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( || {
        App::new()
        .route("/", web::get().to(greet))
        .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8080")?
    .bind("127.0.0.1:8081")?
    .workers(3)
    .run()
    .await
}