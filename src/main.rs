use std::io;

fn main() {
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
            "2" => start_trading(),
            "3" => println!(" 🛑 Stopping Trading..."),
            "4" => {
                println!(" ❌ Exiting...");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn start_trading() {
    loop {
        println!("📈 Select Trading Strategy:");
        println!("1. 🥇 Strategy A");
        println!("2. 🥈 Strategy B");
        println!("3. 🔙 Back to Main Menu");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim() {
            "1" => println!("Starting 🥇 Strategy A..."),
            "2" => println!("Starting 🥈 Strategy B..."),
            "3" => break,
            _ => println!("⚠️ Invalid choice. Please try again."),
        }
    }
}
