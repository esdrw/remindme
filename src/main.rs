use std::env;

mod reminder;

fn main() {
    let args: Vec<String> = env::args().collect();

    let action = &args[1];
    if action == "new" {
        reminder::new();
    } else if action == "raw" {
        reminder::raw();
    } else if action == "today" {
        reminder::today();
    } else if action == "done" {
        reminder::done();
    } else if action == "help" {
        println!("NYI");
    } else {
        println!("Unrecognized argument {}", action);
    }
}
