use dotenv::dotenv;
use std::env;
use std::time::Instant;
use surrealdb::engine::any::{connect, Any};
use surrealdb::engine::remote::ws::{Ws, Client};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

pub async fn estalbish_connection() -> surrealdb::Result<Surreal<Any>>{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_user = env::var("DATABASE_USER").expect("DATABASE_USER must be set");
    let database_password = env::var("DATABASE_PASS").expect("DATABASE_PASS must be set"); 
    println!("{database_url}");
    let db = connect(database_url).await?;
    let now = Instant::now();
    // let db = Surreal::new::<Ws>("localhost:8080").await?;
    println!("Connected");
    db.signin( Root {
        username: database_user.as_str(),
        password: database_password.as_str()
    }).await?;

    db.use_ns("web_app").use_db("web_app").await?;
    println!("{}", now.elapsed().as_secs_f64());
    Ok(db)
} 