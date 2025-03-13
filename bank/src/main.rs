#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

#[derive(Debug)]
struct Account {
    balance: i32,
    id: u32,
    holder: String,
}

fn main() {
    println!("Hello, world!");
}
