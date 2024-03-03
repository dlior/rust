use std::io::{ self, Write };

fn prompt_message(msg: &str) {
    print!("{msg}");
    io::stdout().flush().unwrap();
}

pub fn account_activity() {
    let factor: f64;

    loop {
        prompt_message("Please enter a positive number: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line.");

        match input.trim().parse::<f64>() {
            Ok(parsed) if parsed > 0.0 => {
                factor = parsed;
                break;
            }
            _ => prompt_message("Invalid number. Please enter again: "),
        }
    }

    prompt_message("Enter the account activity: ");
    let mut transactions = String::new();
    io::stdin().read_line(&mut transactions).expect("Failed to read line.");

    let transactions: Vec<i64> = transactions
        .split_whitespace()
        .map(|t| t.parse::<i64>().expect("Invalid input."))
        .take_while(|&t| t != 0)
        .collect();
    let account_balance: i64 = transactions.iter().sum();
    let total_amount: f64 = transactions
        .iter()
        .map(|&t| t.abs() as f64)
        .sum();
    let transactions_count = transactions.len();
    let average_amount = total_amount / (transactions_count as f64);
    let has_suspicious_activity = transactions
        .iter()
        .any(|&t| { (t.abs() as f64) > average_amount * factor });

    println!("Account balance: {account_balance}");
    println!("Average amount: {:.2}", average_amount);
    match has_suspicious_activity {
        true => println!("Suspicious account!"),
        false => println!("No suspicious activity detected."),
    }
}
