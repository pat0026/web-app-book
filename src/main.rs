use std::collections::HashMap;

#[derive(Debug)]
enum CharacterValue {
    Name(String),
    Age(i32),
    Items(Vec<String>),
}

enum SomeValue {
    StringValue(String),
    IntValue(i32),
}

fn print(message: &str) {
    println!("{message}")
}

fn error_check(check: bool) -> Result<i8, &'static str> {
    if check {
        Err("this is an error")
    } else {
        Ok(1)
    }
}

fn print_2(value: &String) {
    println!("{value}");
}

fn print_3(value: &String, value_two: &String) {
    println!("{value}");
    println!("{value_two}");
}

fn print_4(value: &mut i8) {
    *value += 1;
    println!("In function the values is: {}", value);
}

fn get_highest<'a>(first_number: &'a i8, second_number: &'a i8) -> &'a i8 {
    if first_number > second_number {
            first_number
    }else {
        second_number
    }
}

fn filter<'a, 'b>(first_number:&'a i8, second_number: &'b i8) -> &'a i8 {
    if first_number < second_number {
        // &0
        let y = 0;
        let _ = &y;
        &0
    } else {
        first_number
    }
}

fn main() {
    print("Hello World");
    let number: u8 = 255;
    let number: i16 = 255;
    let number = 255i16;
    let number_two = 5i8;
    let result = number + number_two as i16;
    let result = number as i8 + number_two;
    // let result = i8::from(number) + number_two;
    println!("{result}");

    let int_array: [i32; 3] = [1, 2, 3];
    for i in int_array {
        println!("{i}");
    }
    println!("{}", int_array[1]);

    println!();

    let mut mutable_array: [i32; 3] = [1, 2, 0];
    mutable_array[2] = 3;
    println!("{:?}", mutable_array);
    println!("{}", mutable_array.len());

    println!();

    let slice_array: [i32; 100] = [0; 100];
    println!("length: {}", slice_array.len());
    println!("slice: {:?}", &slice_array[5..8]);
    println!("slice: {:?}", slice_array);

    println!();

    let multi_array: [SomeValue; 4] = [
        SomeValue::StringValue(String::from("one")),
        SomeValue::IntValue(2),
        SomeValue::StringValue(String::from("three")),
        SomeValue::IntValue(4),
    ];
    for i in multi_array {
        match i {
            SomeValue::StringValue(data) => {
                println!("The string is: {}", data);
            }
            SomeValue::IntValue(data) => {
                println!("The int is: {}", data);
            }
        }
    }

    println!();

    let mut string_vector: Vec<&str> = vec!["one", "two", "three"];
    println!("{:?}", string_vector);
    string_vector.push("four");
    println!("{:?}", string_vector);

    println!();

    let mut profile: HashMap<&str, CharacterValue> = HashMap::new();

    profile.insert("name", CharacterValue::Name("Maxwell".to_string()));
    profile.insert("age", CharacterValue::Age(32));
    profile.insert(
        "items",
        CharacterValue::Items(vec![
            "laptop".to_string(),
            "book".to_string(),
            "coat".to_string(),
        ]),
    );
    println!("{:?}", profile);

    match profile.get("name") {
        Some(value_data) => match value_data {
            CharacterValue::Name(name) => {
                println!("the name is: {}", name)
            }
            _ => panic!("name should be a string"),
        },
        None => {
            println!("name is not present");
        }
    }

    println!();

    println!("{:?}", error_check(false));
    println!("{:?}", error_check(false).is_err());
    println!("{:?}", error_check(true));
    println!("{:?}", error_check(true).is_err());
    // let rusult: i8 = error_check(true).expect("this has been caught");

    println!();

    let one: i8 = 10;
    let two: i8 = one + 5;
    println!("{one}");
    println!("{two}");

    let one = "one".to_string();
    let two = one.to_owned() + " two";
    println!("{}", one);
    println!("{}", two);

    println!();

    let one = "one".to_string();
    print_2(&one);
    println!("{one}");

    println!();

    let one = "one".to_string();
    print_3(&one, &one);
    println!("{one}");

    println!();

    let mut one: i8 = 5;
    print_4(&mut one);
    let mut two = &mut one;
    *two += 1;
    two = &mut (*two + 1);
    println!("In main hte value is: {one}");

    println!();

    let one = "one";
    let two: &str;

    {
        println!("{one}");
        two = "two";
    }
    println!("{one}");
    println!("{two}");

    println!();

    // let one: &i8;
    // {
    //     let two: i8 = 2;
    //     one = &two;
    // }
    // println!("r: {one}");

    // let one: i8 = 1;
    // let outcome: &i8;
    // {
    //     let two: i8 = 2;
    //     let outcome: &i8 = get_highest(&one, &two);
    // }
    // println!("{outcome}");

    let one: i8 = 3;
    let outcome: &i8;
    {
        let two = 2;
        outcome = filter(&one, &two);
    }
    println!("{outcome}");

    println!()

    
}
