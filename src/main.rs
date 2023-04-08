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
use futures::future::join_all;

async fn do_something(number: i8) -> i8 {
    println!("number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    return 2;
}

fn main() {
    let now = time::Instant::now();
    let future_one = do_something(1);
    let two_seconds = time::Duration::new(2,0);
    thread::sleep(two_seconds);
    let outcome = block_on(future_one);
    println!("time elapsed {:?}", now.elapsed());
    println!("Here is the outcome: {}", outcome);

    let future_two = async {
        return do_something(2).await
    };
    let future_two = block_on(future_two);
    println!("Here is the outcome: {:?}", future_two);

    let future_three = async {
        let outcome_one = do_something(2).await;
        let outcome_two = do_something(3).await;
        return outcome_one + outcome_two
    };

    let future_outcome = block_on(future_three);
    println!("Here is the outcome: {:?}", future_outcome);

    let future_four = async {
        let outcome_one = do_something(2);
        let outcome_two = do_something(3);
        let results = join!( outcome_one, outcome_two);
        return results.0 + results.1
    };

    let now = time::Instant::now();
    let result = block_on(future_four);
    println!("tiem elapsed {:?}", now.elapsed());
    println!("here is the result: {:?}", result);

    let async_outcome = async {
        // 1
        let mut futures_vec = Vec::new();
        let future_four = do_something(4);
        let future_five = do_something(5);

        // 2
        futures_vec.push(future_four); 
        futures_vec.push(future_five);

        // 3
        let handles = futures_vec.into_iter().map(
            async_std::task::spawn).collect::<Vec<_>>();
        
        // 4
        let results = join_all(handles).await;
        return results.into_iter().sum::<i8>();
    };

    let now = time::Instant::now();
    let result = block_on(async_outcome);
    println!("time elapsed for join vec {:?}", now.elapsed());
    println!("Here is the result: {:?}", result);
}