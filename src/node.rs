struct Balance {
    balance: u128,
    pending: u128,
}

struct Transactions {
    type_: String,
    account: String,
    amount: u128,
    local_timestamp: u128,
    weight: u128,
    hash: String,
}

struct AccountInfoResponse {
    frontier: String,
    open_block: String,
    representative_block: String,
    balance: u128,
    modified_timestamp: u128,
    block_count: u128,
    account_version: u32,
    confirmation_height: u32,
    confirmation_height_frontier: String,
}

trait NodeRPC {
    fn balance(acct: &str) -> Balance {
        todo!();
    }

    fn block_count(acct: &str) -> u128 {
        todo!();
    }

    fn key_to_account(pub_key: &str) -> Account {
        todo!();
    }

    fn account_history(acct: &str) {
        todo!();
    }

    fn account_info(acct: &str) {
        todo!();
    }

    fn account_to_pubkey(acct: &str) -> String {
        todo!();
    }

    fn account_repr(acct: &str) -> String {
        todo!();
    }
}
