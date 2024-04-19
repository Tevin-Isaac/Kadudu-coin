use std::collections::HashMap;
use candid::CandidType;
use candid::Principal;
#[derive(CandidType)]
struct ICRC1MemeToken {
    id: u64,
    name: String,
    symbol: String,
    total_supply: u128,
    ledger: HashMap<Principal, u128>,
    meme_url: String,
    meme_description: String,
    meme_creator: Principal,
}

impl ICRC1MemeToken {
    pub fn new(
        id: u64,
        total_supply: u128,
        meme_url: String,
        meme_description: String,
        meme_creator: Principal,
    ) -> Self {
        ICRC1MemeToken {
            id,
            name: "Kadudu".to_string(),
            symbol: "KD".to_string(),
            total_supply,
            ledger: HashMap::new(),
            meme_url,
            meme_description,
            meme_creator,
        }
    }

    pub fn transfer(&mut self, from: Principal, to: Principal, amount: u128) -> Result<(), String> {
        if let Some(balance) = self.ledger.get(&from) {
            if *balance >= amount {
                let new_balance: u128 = balance - amount;
                self.ledger.insert(to, new_balance);
                Ok(())
            } else {
                Err("Insufficient balance".to_string())
            }
        } else {
            Err("Account not found".to_string())
        }
    }

    pub fn balance_of(&self, account: Principal) -> u128 {
        let amount :u128 = *self.ledger.get(&account).unwrap_or(&0);
        amount
    }

    pub fn mint(&mut self, to: Principal, amount: u128) {
        let balance :u128 = *self.ledger.get(&to).unwrap_or(&0);
        self.ledger.insert(to, balance + amount);
        self.total_supply += amount;
    }

    pub fn burn(&mut self, from: Principal, amount: u128) -> Result<(), String> {
        if let Some(balance) = self.ledger.get_mut(&from) {
            if *balance >= amount {
                *balance -= amount;
                self.total_supply -= amount;
                Ok(())
            } else {
                Err("Insufficient balance".to_string())
            }
        } else {
            Err("Account not found".to_string())
        }
    }

    pub fn get_meme_creator(&self) -> Principal {
        self.meme_creator
    }

    pub fn get_meme_description(&self) -> String {
        self.meme_description.clone()
    }
}
// need this to generate candid
ic_cdk::export_candid!();