mod actions;
mod utils;
mod bill;

use std::collections::HashMap;

use utils::*;

use actions::*;
use Actions::*;

use bill::Bill;

fn main() {
    println!("Welcome to Bill Management Services\n");
    let mut bills: HashMap<String, Bill> = HashMap::new();
    loop {
        get_welcome_note();
        match get_action() {
            Some(action) => match action {
                Add => {
                    let new_name = add_bill(&mut bills);
                    println!("bill for {} added successully", new_name);
                }
                View => {
                    if bills.len() == 0 {
                        println!("There are currently no entries")
                    } else {
                        println!("here are the registered bills");
                        view_bills(&mut bills)
                    }
                }
                Edit => edit_bill(&mut bills),
                Remove => remove_bill(&mut bills),
                Cancel => {
                    println!("thanks for using our service, bye");
                    break;
                }
            },
            _ => (),
        }
    }
}
