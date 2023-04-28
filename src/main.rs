#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_service::Service;
use actix_web::{App, HttpServer};
use actix_cors::Cors;

mod json_serialization;
// mod processes;
mod state;
mod to_do;
mod views;
mod jwt;
mod database;
mod schema;
mod models;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin()
                                .allow_any_method()
                                .allow_any_header();

        let app = App::new()
            .wrap_fn(|req, srv| {
                println!("{}-{}", req.method(), req.uri());
                let future = srv.call(req);

                async {
                    let result = future.await?;
                    Ok(result)
                }
            })
            .configure(views::views_factory).wrap(cors);
        app
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}

// use core::fmt::Debug;

// #[derive(Debug)]
// struct TwoDposition {
//     x: i32,
//     y: i32,
// }

// #[derive(Debug)]
// struct ThreeDposition {
//     x: i32,
//     y: i32,
//     z: i32,
// }

// fn print_two(s: &TwoDposition) {
//     println!("{:?}", s);
// }

// fn print_three(s: &ThreeDposition) {
//     println!("{:?}", s);
// }

// fn print_debug<S>(s: &S)
// where
//     S: Debug,
// {
//     println!("{:?}", s);
// }

// fn main() {
//     let two = TwoDposition{x:1, y:2};
//     let three = ThreeDposition{x:1, y:2, z:3};
//     print_debug(&two);
//     print_debug(&three);
// }
