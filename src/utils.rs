use crate::actions::Actions;
use std::io;

pub fn get_input() -> Option<String> {
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

pub fn get_action() -> Option<Actions> {
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

pub fn get_welcome_note() {
    println!(" \nKindly select an action");
    println!("1. Add Bill");
    println!("2. Update Bill ");
    println!("3. Remove Bill ");
    println!("4. View All Bills ");
    println!("-----------------");
    println!("type 'cancel' to cancel\n")
}
