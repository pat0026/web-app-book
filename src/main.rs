mod processes;
mod state;
mod to_do;

use std::{env, string};

use serde_json::value::Value;
use serde_json::Map;

use processes::process_input;
use state::read_file;
use to_do::enums::TaskStatus;
use to_do::to_do_factory;

use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", name)
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         println!("http server factory is firing");
//         App::new()
//             .route("/", web::get().to(greet))
//             .route("/{name}", web::get().to(greet))
//             .route("/say/hello", web::get().to(|| async { "Hello Again!" }))
//     })
//     .bind("127.0.0.1:8080")?
//     .workers(3)
//     .run()
//     .await
// }

fn add_double(closure: fn(i32) -> i32,
                one: i32, two: i32) -> i32 {
                    closure(one) + closure(two)
                }

fn add_doubles(closure: Box<dyn Fn(i32) -> i32>,
                one: i32, two: i32) -> i32 {
                    closure(one)  + closure(two)
                }

fn main() {
    let test_closure = |string_input| {
        println!("{}", string_input);
    };
    test_closure("test");
    // Can't be
    // test_closure(23);

    println!();
    // // Can't be
    // {
    //     let test_closure_2 = |string_input| {
    //         println!("{}", string_input);
    //     };
    // }

    // // test_closure_2("test");

    println!();

    let another_str = "case";
    let test_closure_3 = |string_input| {
        println!("{} {}", string_input, another_str);
    };

    test_closure_3("test");

    println!();

    let closure = |int_input| int_input * 2;
    let outcome = add_double(closure, 2, 3);
    println!("{}", outcome);

    println!();

    let one = 2;

    let closure = move |int_input| {
        int_input * one
    };
    let outcome = add_doubles(Box::new(closure), 2, 3);
    println!("{}", outcome);
}