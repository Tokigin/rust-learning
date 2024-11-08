fn main() {
    let mut account: BankAccount = BankAccount {
        owner: "Mya Mya".to_string(),
        balance: 2000.25,
    };
    account.check_balance();
    account.withdraw(500.55);
    account.check_balance();
}

struct BankAccount {
    owner: String,
    balance: f64,
}
impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing {} from {} account. ", amount, self.owner);
        self.balance -= amount;
    }
    fn check_balance(&self) {
        println!("Account owned by {} has {} ", self.owner, self.balance);
    }
}
