use std::io;
mod trading;

pub fn main_menu() {
    loop {
        println!("🚀 Trading Bot Menu:");
        println!("1. 📊 View Bot Status");
        println!("2. 💼 Start Trading");
        println!("3. 🛑 Stop Trading");
        println!("4. ❌ Exit");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim() {
            "1" => println!(" 📊 Bot Status: Active"),
            "2" => trading::start_trading(),
            "3" => println!(" 🛑 Stopping Trading..."),
            "4" => {
                println!(" ❌ Exiting...");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}
