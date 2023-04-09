mod processes;
mod state;
mod to_do;
mod views;

use actix_web::{App, HttpServer};

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