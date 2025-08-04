use std::io;

enum Transaction {
    CheckBalance,
    Exit,
    Deposit(f64),
    Withdraw(f64),
}
#[allow(dead_code)]
struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn new(owner: String) -> Self {
        BankAccount {
            owner,
            balance: 0.0,
        }
    }

    fn check_transaction(&mut self, tx: Transaction) {
        match tx {
            Transaction::Deposit(amount) => {
                self.balance += amount;
                println!("The amount ₹{:.2} has been credited.", amount);
            }
            Transaction::Withdraw(amount) => {
                if amount <= self.balance {
                    self.balance -= amount;
                    println!("The amount ₹{:.2} has been debited.", amount);
                } else {
                    println!("Insufficient balance for withdrawal.");
                }
            }
            Transaction::CheckBalance => {
                println!(
                    "The current amount in your bank account is ₹{:.2}",
                    self.balance
                );
            }
            Transaction::Exit => {
                println!("We are exiting the program of the bank account.");
            }
        }
    }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Can't get what you have entered");
    buffer.trim().to_string()
}

fn parse_transaction(input: &str) -> Option<Transaction> {
    match input {
        "1" => {
            let amount = get_input("Enter your deposit amount:")
                .parse::<f64>()
                .unwrap_or(0.0);
            Some(Transaction::Deposit(amount))
        }
        "2" => {
            let amount = get_input("Enter your withdrawal amount:")
                .parse::<f64>()
                .unwrap_or(0.0);
            Some(Transaction::Withdraw(amount))
        }
        "3" => Some(Transaction::CheckBalance),
        "4" => Some(Transaction::Exit),
        _ => None,
    }
}

fn main() {
    let name = get_input("Enter your name:");
    let mut account = BankAccount::new(name);

    loop {
        println!("\n==== Banking Menu ====");
        println!("1. Deposit");
        println!("2. Withdraw");
        println!("3. Check Balance");
        println!("4. Exit");

        let choice = get_input("Choose an option:");

        match parse_transaction(&choice) {
            Some(Transaction::Exit) => {
                account.check_transaction(Transaction::Exit);
                break;
            }
            Some(tx) => {
                account.check_transaction(tx);
            }
            None => {
                println!("Invalid choice. Try again.");
            }
        }
    }
}
