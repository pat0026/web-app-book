mod processes;
mod state;
mod to_do;

use std::thread::JoinHandle;
use std::{env, thread, time};

use serde_json::value::Value;
use serde_json::Map;

use processes::process_input;
use state::read_file;
use to_do::enums::TaskStatus;
use to_do::to_do_factory;

use actix_web::{web, App, HttpRequest, HttpServer, Responder};

fn do_something(number: i8) -> i8 {
    println!("number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    return 2;
}

fn main() {
    // let now = time::Instant::now();
    // let one: i8 = do_something(1);
    // let two: i8 = do_something(2);
    // let three: i8 = do_something(3);

    // println!("time elapsed {:?}", now.elapsed());
    // println!("result {}", one + two + three);

    let now = time::Instant::now();
    let thread_one: JoinHandle<i8> = thread::spawn(|| do_something(1));
    let thread_two: JoinHandle<i8> = thread::spawn(|| do_something(2));
    let thread_three: JoinHandle<i8> = thread::spawn(|| do_something(3));

    let result_one = thread_one.join();
    let result_two = thread_two.join();
    let result_three = thread_three.join();

    println!("time elapsed {:?}", now.elapsed());

    match result_one {
        Ok(result) => {
            println!("the result for thread one is {}", result);
        }
        Err(ref result) => {
            if let Some(string) = result.downcast_ref::<String>() {
                println!("the error for thread one is: {}", string);
            } else {
                println!("ther error for thread one does not have a message");
            }
        }
    };

    println!(
        "result {}",
        result_one.unwrap() + result_two.unwrap() + result_three.unwrap()
    );
}
