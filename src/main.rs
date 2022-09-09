mod actions;
mod utils;
mod bill;

use utils::get_action;
use utils::get_welcome_note;

use actions::add_bill;
use actions::edit_bill;
use actions::remove_bill;
use actions::view_bills;
use actions::Actions::*;
use std::collections::HashMap;


use crate::bill::Bill;

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
