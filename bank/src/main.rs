#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}

#[derive(Debug)]
struct Account {
    balance: i32,
    id: u32,
    holder: String,
}

impl Account {
    fn new(holder_name: String, account_id: u32) -> Self {
        Account { holder: holder_name, id: account_id, balance: 0 }
    }
}

fn print_account(account: Account) {
    println!("{account:#?}");
}

fn main() {
    let bank = Bank::new();
    let account = Account::new(String::from("Steve"), 123);

    println!("{bank:#?}");
    print_account(account);
    print_account(account);
}
