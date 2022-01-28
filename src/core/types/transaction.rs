use ethereum_types::{Address, U256};

/// An Ethereum transaction.
pub struct Transaction {
    nonce: U256,
    gas_price: U256,
    gas_limit: U256,
    to: Option<Address>,
    value: U256,
    v: U256,
    r: U256,
    s: U256,
    init: Option<Vec<u8>>,
    data: Option<Vec<u8>>,
}

impl Transaction {
    pub fn nonce(&self) -> U256 {
        self.nonce
    }

    pub fn gas_price(&self) -> U256 {
        self.gas_price
    }

    pub fn gas_limit(&self) -> U256 {
        self.gas_limit
    }

    pub fn to(&self) -> &Option<Address> {
        &self.to
    }

    pub fn value(&self) -> U256 {
        self.value
    }

    pub fn v(&self) -> U256 {
        self.v
    }

    pub fn r(&self) -> U256 {
        self.r
    }

    pub fn s(&self) -> U256 {
        self.s
    }

    pub fn init(&self) -> &Option<Vec<u8>> {
        &self.init
    }

    pub fn data(&self) -> &Option<Vec<u8>> {
        &self.data
    }
}
