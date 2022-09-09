
use std::collections::HashMap;
use std::io;

#[derive(Debug)]
enum Actions {
    Add,
    View,
    Remove,
    Edit,
    Cancel,
}
#[derive(Debug)]
struct Bill {
    name: String,
    amount: f32,
}

impl Bill {
    fn print(&self) {
        println!("{:?}", self)
    }
    fn update_amount(&mut self, amount: f32) {
        self.amount = amount;
    }
}

fn get_input() -> Option<String> {
    let mut input = String::new();
    while io::stdin().read_line(&mut input).is_err() {
        println!("an error occured, please try again")
    }
    input = input.trim().to_owned();
    if input == "" {
        return None;
    }
    Some(input)
}

fn get_action() -> Option<Actions> {
    match get_input() {
        Some(action) => {
            use Actions::*;
            match action.to_lowercase().as_str() {
                "add" => Some(Add),
                "1" => Some(Add),
                "edit" => Some(Edit),
                "2" => Some(Edit),
                "view" => Some(View),
                "4" => Some(View),
                "remove" => Some(Remove),
                "3" => Some(Remove),
                "cancel" => Some(Cancel),
                _ => {
                    println!("invalid command, try again");
                    None
                }
            }
        }
        _ => None,
    }
}

fn add_bill(bills: &mut HashMap<String, Bill>) -> String {
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

fn view_bils(bills: &mut HashMap<String, Bill>) {
    for bill in bills.iter() {
        bill.1.print()
    }
}

fn edit_bill(bills: &mut HashMap<String, Bill>) {
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
                    bill.update_amount(amount)
                }
            }
        }
        None => println!("An error occurred"),
    }
}

fn remove_bill(bills: &mut HashMap<String, Bill>) {
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

fn get_welcome_note() {
    println!(" \nKindly select an action");
    println!("1. Add Bill");
    println!("2. Update Bill ");
    println!("3. Remove Bill ");
    println!("4. View All Bills ");
    println!("-----------------");
    println!("type 'cancel' to cancel\n")
}
use Actions::*;
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
                        view_bils(&mut bills)
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
