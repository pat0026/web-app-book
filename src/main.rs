mod to_do;
use to_do::structs::done::Done;
use to_do::structs::pending::Pending;

fn main() {
    println!("{}", to_do::enums::TaskStatus::DONE);
    println!("{}", to_do::enums::TaskStatus::PENDING);
    let outcome = to_do::enums::TaskStatus::DONE.to_string();
    println!("{}", outcome);

    println!();

    let done = Done::new("shopping");
    println!("{}", done.super_struct.title);
    println!("{}", done.super_struct.status.stringify());

    let pending = Pending::new("laundry");
    println!("{}", pending.super_struct.title);
    println!("{}", pending.super_struct.status.stringify());
}