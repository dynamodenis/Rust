#[derive(Debug)]
struct Account{
    id: u32,
    balance: i32,
    holder: String,
}

#[derive(Debug)]
struct Bank{
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self{
        Bank {accounts: vec![]}
    }

    fn add_account(&mut self, account: Account){
        self.accounts.push(account);
    }

    fn total_balance(&self) -> i32{
        self.accounts.iter().map(|account| account.balance).sum()
    }

    fn summary(&self) -> Vec<String>{
        self.accounts.iter().map(|account| account.summary()).collect::<Vec<String>>()
    }
}

impl Account{
    fn new(id: u32, holder: String) -> Self{
        Account {id: id, balance: 0, holder: holder}
    }

    fn deposit(&mut self, amount: i32){
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: i32){
        if self.balance >= amount {
            self.balance -= amount;
        } else {
            println!("Insufficient funds");
        }
    }

    fn summary(&self) -> String{
        format!("Account ID: {}, Holder: {}, Balance: {}", self.id, self.holder, self.balance)
    }
}


fn change_account(account: &mut Account){
    account.balance += 100;
}

fn main() {
    let mut bank = Bank::new();

    let mut account1 = Account::new(1, "Alice".to_string());
    account1.deposit(100);
    account1.withdraw(50);

    bank.add_account(account1);

    let mut account2 = Account::new(2, "Bob".to_string());
    account2.deposit(200);
    account2.withdraw(100);
    bank.add_account(account2);


    println!("Here is your bank summary: {:#?}", bank.summary());
    println!("Here is balance: {:#?}", bank.total_balance());
    // println!("Here is account1: {:#?}", &account1);
}
