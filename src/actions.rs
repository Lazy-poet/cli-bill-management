use crate::bill::Bill;
use crate::utils::get_input;
use std::collections::HashMap;

#[derive(Debug)]

pub enum Actions {
    Add,
    View,
    Remove,
    Edit,
    Cancel,
}
pub fn add_bill(bills: &mut HashMap<String, Bill>) -> String {
    let mut name = String::new();
    let mut amount = 0.;
    let mut count = 1;

    while count <= 2 {
        if count == 1 {
            println!("please enter a name");
            match get_input() {
                Some(name_input) => {
                    name = name_input;
                }
                _ => panic!("Invalid input"),
            }
        } else {
            println!("please enter amount owed");
            match get_input() {
                Some(amount_input) => {
                    amount = amount_input.parse::<f32>().unwrap();
                }
                _ => (),
            }
        };
        count += 1;
    }
    let name_copy = name.clone();
    let bill = Bill { name, amount };
    bills.insert(bill.name.to_owned(), bill);
    return name_copy;
}

pub fn view_bills(bills: &mut HashMap<String, Bill>) {
    for bill in bills.iter() {
        bill.1.print()
    }
}
pub fn edit_bill(bills: &mut HashMap<String, Bill>) {
    println!("enter user name");
    match get_input() {
        Some(name) => {
            if !bills.get_mut(&name).is_some() {
                println!("user not found");
                return;
            }
            let bill = bills.get_mut(&name).unwrap();
            // match bill {
            // Some(val) => {
            println!("Enter new amount");
            match get_input().unwrap() {
                amount_input => {
                    let amount = amount_input.parse::<f32>().unwrap();
                    bill.update_amount(amount);
                    println!("bill updated successfully")
                }
            }
        }
        None => println!("An error occurred"),
    }
}
pub fn remove_bill(bills: &mut HashMap<String, Bill>) {
    println!("enter user name of bill to remove");
    match get_input() {
        Some(name) => {
            println!("this is a destructive action and cannot be reversed");
            println!("Do you want to proceed ? (Y / any other key to cancel)");
            match get_input().unwrap() {
                input => {
                    if input.to_lowercase() != "y" {
                        println!("cancelled");
                        ()
                    }
                }
            }

            let key = bills.remove(name.as_str());
            if !key.is_some() {
                println!("User {:?} not found", name)
            } else {
                println!("{} removed successfully", name)
            }
        }
        _ => (),
    }
}
