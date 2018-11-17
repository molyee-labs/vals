pub enum Currency {
    V,
    USD,
    EUR,
    RUB,
}

pub trait Fund {

    fn name(&self) -> String;

    fn currency(&self) -> Currency;

    fn deposit(&self, amount) -> Result<String, err>;

    fn withdraw(&self, amount) -> Result<String, err>;
}

pub struct Address {
    id: Id,
    owner: UserId,
    balance: i64,
    available: i64,
    fund: Fund
}