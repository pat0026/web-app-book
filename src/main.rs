// use std::env;
use clap::{Arg, App};

fn main() {
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    // let path: &str = &args[0];

    // if path.contains("debug") {
    //     println!("Debug is running");
    // } else if path.contains("release") {
    //     println!("release is running");
    // } else {
    //     panic!("The setting is neither debug or release");
    // }

        let app = App::new("booking")
            .version("1.0")
            .about("Books in user")
            .author("Maxwell Flitton");
        let first_name = Arg::new("first name")
            .long("f")
            .takes_value(true)
            .help("first name of user")
            .required(true);
        let last_name = Arg::new("last name")
            .long("l")
            .takes_value(true)
            .help("first name of user")
            .required(true);
        let age = Arg::new("age")
            .long("a")
            .takes_value(true)
            .help("age of the user")
            .required(true);

        let app = app.arg(first_name).arg(last_name).arg(age);
        
        let matches = app.get_matches();
        let name = matches.value_of("first name")
            .expect("First name is required");
        let surname = matches.value_of("last name")
            .expect("Surname is required");
        let age = matches.value_of("age")
            .expect("Age is required").parse::<i8>().unwrap();

        println!("{:?}", name);
        println!("{:?}", surname);
        println!("{:?}", age);
}
