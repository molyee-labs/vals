pub struct Address {
    id: Id,
    owner: UserId,
    fund: u64,
}

pub struct Wallet {
    id: String,
    key: Vec<u8>,
    address: Address
}

pub fn