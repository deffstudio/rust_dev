use std::io;

pub fn start_trading() {
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
