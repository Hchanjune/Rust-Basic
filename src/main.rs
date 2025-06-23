mod sample_struct;

use std::io;

fn main() {
    println!("Hello, world!");
    println!("Welcome");

    loop {
        println!("---------------------");
        println!("1 for Rust Code Convention");
        println!("2 dose nothing");
        println!("3 to quit");
        println!("---------------------");
        println!("Waiting For Your Command Input : ");

        let mut command_input = String::new();
        io::stdin()
            .read_line(&mut command_input)
            .expect("Failed to read line\nThe Command Input Must Be One of [1, 2, 3]");

        match command_input.trim() {
            "1" => {
                println!("Rust Code Convention");
                println!("Rust Uses snake_case! be aware of it!\nMight be not familiar");
                press_any_key_to_continue();
            }
            "2" => {
                press_any_key_to_continue();
            }
            "3" => {
                println!("Thank you!");
                break;
            }
            _ => {
                println!("Please enter a valid command!!!\nTheCommand Input Must Be One of [1, 2, 3]");
            }
        }
        println!();

    }



}

fn press_any_key_to_continue() {
    println!("Press any key to continue");
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).unwrap();
}
